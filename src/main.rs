fn main() {
    let ctx = zmq::Context::new();

    let socket = ctx.socket(zmq::SUB).unwrap();
    socket
        .set_linger(0)
        .expect("failed to set linger time for subcriber");
    socket
        .connect("tcp://127.0.0.1:7000")
        .expect("failed connecting subscriber");
    socket.set_subscribe(b"").expect("failed to subscribe");

    let sync = ctx.socket(zmq::REQ).unwrap();
    sync.set_linger(0)
        .expect("failed to set linger time for sync socket");
    sync.connect("tcp://127.0.0.1:7001")
        .expect("failed connecting sync socket");
    sync.send("", 0).expect("failed to send sync request");
    sync.recv_msg(0).expect("failed to receive sync message");

    let message = socket
        .recv_string(0)
        .expect("failed receiving string")
        .unwrap();

    println!("Received the message {}", message);
}

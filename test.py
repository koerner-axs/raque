import zmq


ctx = zmq.Context()
publisher = ctx.socket(zmq.PUB)
publisher.bind("tcp://127.0.0.1:7000")

sync_socket = ctx.socket(zmq.REP)
sync_socket.bind("tcp://127.0.0.1:7001")
sync_socket.recv()
sync_socket.send_string("")

publisher.send_string("hello rust!")

ctx.destroy()
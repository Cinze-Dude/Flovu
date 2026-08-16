import socket

s = socket.create_connection(("localhost", 1600))

s.sendall(
    b"GET / HTTP/1.1\r\n"
    b"Host: localhost\r\n"
    b"\r\n"
)

print(s.recv(4096))

s.sendall(
    b"GET /favicon.ico HTTP/1.1\r\n"
    b"Host: localhost\r\n"
    b"\r\n"
)

print(s.recv(4096))

s.close()
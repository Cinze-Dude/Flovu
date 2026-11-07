import socket
import time

s = socket.socket()
s.connect(("127.0.0.1", 1600))
# def send(request):
#     s.sendall(request)
    
#     response = s.recv(4096)
#     print(response.decode(errors="replace"))
    
#     s.close()


s.sendall(
    b"POST /echo HTTP/1.1\r\n"
    b"Host: localhost\r\n"
    b"Content-Length: 5\r\n"
    b"\r\n"
)

s.sendall(b"he")

time.sleep(10)

s.sendall(b"llo")

response = s.recv(4096)
print(response.decode(errors="replace"))

s.close()
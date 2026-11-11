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
    b"CONNECT www.youtube.com:443 HTTP/1.1\r\n"
    b"Host: www.youtube.com:443\r\n"
    b"\r\n"
)

response = s.recv(4096)
print(response.decode(errors="replace"))

s.close()
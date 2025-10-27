# udp_client.py
import socket
import threading

SERVER_IP = "127.0.0.1"
SERVER_PORT = 8080

def listen(sock):
    """Receive messages from the server."""
    while True:
        try:
            data, _ = sock.recvfrom(1024)
            print(data.decode())
        except Exception:
            break

def main():
    sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)

    # Start background thread to listen for messages
    threading.Thread(target=listen, args=(sock,), daemon=True).start()

    sock.sendto("HELLO NIGGER".encode(), (SERVER_IP, SERVER_PORT))
    print("[CLIENT] Type messages and press Enter. Ctrl+C to quit.")
    while True:
        try:
            msg = input("> ")
            if not msg:
                continue
            sock.sendto(msg.encode(), (SERVER_IP, SERVER_PORT))
        except KeyboardInterrupt:
            print("\n[CLIENT] Exiting...")
            break

    sock.close()

if __name__ == "__main__":
    main()
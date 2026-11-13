#!/usr/bin/env python3
"""Send one command to the Amiberry Unix-domain IPC socket and print its reply."""
import socket
import sys

if len(sys.argv) < 3:
    raise SystemExit("usage: amiberry-ipc.py SOCKET COMMAND [ARG ...]")

socket_path, *parts = sys.argv[1:]
message = "\t".join(parts) + "\n"
with socket.socket(socket.AF_UNIX, socket.SOCK_STREAM) as client:
    client.settimeout(5)
    client.connect(socket_path)
    client.sendall(message.encode("utf-8"))
    response = client.recv(4096).decode("utf-8", errors="replace").strip()

if not response:
    raise SystemExit("Amiberry IPC returned an empty response")
print(response)
if response.startswith("ERROR\t"):
    raise SystemExit(response)
if not response.startswith("OK\t") and response != "OK":
    raise SystemExit("Unexpected Amiberry IPC response")

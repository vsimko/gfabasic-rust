#!/usr/bin/env python3
"""Send one command to the Amiberry Unix-domain IPC socket and print its reply."""
import socket
import sys
import time

if len(sys.argv) < 3:
    raise SystemExit("usage: amiberry-ipc.py SOCKET COMMAND [ARG ...]")

socket_path, *parts = sys.argv[1:]
message = "\t".join(parts) + "\n"
last_error = None

# Amiberry creates its IPC socket before the emulation thread is always
# ready to answer commands. Retry idempotent setup/query commands during
# that short startup window instead of failing on the first empty response.
for attempt in range(6):
    try:
        with socket.socket(socket.AF_UNIX, socket.SOCK_STREAM) as client:
            client.settimeout(5)
            client.connect(socket_path)
            client.sendall(message.encode("utf-8"))
            chunks = []
            while True:
                chunk = client.recv(4096)
                if not chunk:
                    break
                chunks.append(chunk)
                if b"\n" in chunk:
                    break
            response = b"".join(chunks).decode("utf-8", errors="replace").strip()
        if not response:
            raise RuntimeError("Amiberry IPC returned an empty response")
        break
    except (OSError, RuntimeError) as error:
        last_error = error
        if attempt == 5:
            raise SystemExit(f"Amiberry IPC failed after retries: {error}") from error
        time.sleep(1)

print(response)
if response.startswith("ERROR\t"):
    raise SystemExit(response)
if not response.startswith("OK\t") and response != "OK":
    raise SystemExit("Unexpected Amiberry IPC response")

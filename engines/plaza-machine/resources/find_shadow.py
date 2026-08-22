import re

with open('alpine.img', 'rb') as f:
    data = f.read()

# Look for shadow file contents
# It usually contains 'root:' followed by the hash or '!', then colons
matches = re.finditer(b'root:[^:]*:[0-9]*:[0-9]*:[0-9]*:[0-9]*:::', data)
for m in matches:
    start = max(0, m.start() - 50)
    end = min(len(data), m.end() + 50)
    print(f"Found at {m.start()}: {data[start:end]}")

# Since it might be formatted differently, just search for b'root:!:'
matches = re.finditer(b'root:!:', data)
for m in matches:
    start = max(0, m.start() - 20)
    end = min(len(data), m.end() + 50)
    print(f"Found at {m.start()}: {data[start:end]}")


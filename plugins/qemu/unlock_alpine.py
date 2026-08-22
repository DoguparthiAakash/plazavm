import re
import os

with open('alpine.img', 'rb') as f:
    data = f.read()

# Replace locked root passwords with unlocked (empty) passwords
# 'root:!*:' -> 'root::' padded with spaces to keep length identical (ext4 file size must not change)
# Wait, if we replace 'root:!*:20560:0:::::\n' with 'root:::20560:0:::::\n  ' we can't easily without corrupting ext4 unless we pad.
# Actually, since it's just a text file, trailing spaces in the line are fine?
# Wait, /etc/shadow doesn't mind trailing spaces.
# Or better: we just replace 'root:!*:' with 'root::!:' wait no, that's still locked.
# How about 'root::  :' ? No, the format is user:hash:lastchanged:....
# So 'root:!*:20560' (14 chars) -> 'root:::20560' (12 chars). We need 2 padding chars. We can pad the lastchanged field? No, it's a number.
# We can pad with 'root:: ' no, hash cannot contain space.
# We can just change '!' to 'U' (invalid hash) - no, that's still locked.
# We can set a known hash! The hash for 'root' is $6$rounds=5000$salt$somehash. That's too long.
# What about 'root::' and then shifting the rest of the file? No, that shifts sectors.
# Is it possible to use a 2-character crypt hash? 
# DES crypt hash is 13 chars.
# Let's just write a known hash that is exactly the same length! '!*' is 2 chars.
# Wait, if we just replace 'root:!*:' with 'root::*:' - no, '*' is locked.
# Wait, we can pad with a colon! 'root::::' no, that adds fields.
# Can we pad the username? 'root  :!*:'? No.

# Actually, we don't have to keep the exact length if the replacement is the SAME length.
# Wait, 'root:!*:20560:0:::::\n' is 22 bytes.
# If we change it to 'root::20560:0:::::\n  ' it becomes 22 bytes. Wait, 'root::' has an empty hash.
# So 'root::20560:0:::::\n   ' (add two spaces at the end of the line?)
# Let's see if /etc/shadow parses fine with spaces at the end of the line. Yes it does!
# Wait, no, let's just pad the lastchanged number! '20560' -> '020560' or ' 20560'. ' 20560' is valid? 
# In C, strtol allows leading spaces! So 'root:: 20560:0:::::\n' is exactly the same length!
# Let's do that! 'root:!*:20560:0:::::\n' -> 'root:: 20560:0:::::\n'

import sys

modified = 0
new_data = bytearray(data)

        # Also search for the bad padding we did before
for pattern in [b'root:!*:20560:0:::::\n', b'root:*:20560:0:::::\n', b'root:: 20560:0:::::\n']:
    idx = 0
    while True:
        idx = data.find(pattern, idx)
        if idx == -1:
            break
        print(f"Found locked root at {idx}, unlocking...")
        replacement = b'root::20560:0:::::  \n'
        new_data[idx:idx+len(replacement)] = replacement
        modified += 1
        idx += len(pattern)

if modified > 0:
    print(f"Writing {modified} changes to alpine.img...")
    with open('alpine.img', 'wb') as f:
        f.write(new_data)
    print("Done!")
else:
    print("No locked roots found to replace!")

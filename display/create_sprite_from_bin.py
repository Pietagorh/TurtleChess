from bitarray import bitarray
from os import path, remove


data = \
"0000000000000000000000000000000000000000001000000000000000101000000000000010001000000000000101010000000000001000100000000000011011000000000000010100000000000001101100000000000010001000000000001100011000000000110000011000000011000000011000001111111111111000000000000000000000000000000000000"
sprite_name = "bishop"


sprite_path = f"sprites/{sprite_name}.bs"

if path.exists(sprite_path):
    remove(sprite_path)

with open(sprite_path, "xb") as f:
    print((8 - len(data) + len(data) // 8 * 8))
    f.write((8 - len(data) + len(data) // 8 * 8).to_bytes() + bitarray(data).tobytes())


s = ""
with open(sprite_path, "rb") as f:
    bits = 0
    unused_bits = int(f.read(8)[0])

    for b in f.read():
        for i in range(8):
            bits += 1
            if bits - 1 <= unused_bits:
                continue

            bits += 1
            s += "1" if (b >> i) & 1 else "0"

print(len(s), len(data))
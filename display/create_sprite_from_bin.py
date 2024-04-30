from typing import Iterator

from bitarray import bitarray
from os import path, remove


class Sprite:
    def __init__(self, name: str) -> None:
        self.sprite_path = f"sprites/{name}.bs"

        with open(self.sprite_path, "rb") as f:
            self.unused_bits = int(f.read(1)[0])
            self.data = f.read()

    @staticmethod
    def write_sprite(name: str, data: str) -> None:
        sprite_path = f"sprites/{name}.bs"

        if path.exists(sprite_path):
            remove(sprite_path)

        with open(sprite_path, "xb") as f:
            unused_bits = 8 - len(data) + len(data) // 8 * 8
            f.write(unused_bits.to_bytes() + bitarray("".join("0" for _ in range(unused_bits)) + "".join(reversed(list(data)))).tobytes())

    def __repr__(self) -> str:
        res = ""
        for b in self.data:
            bin_repr = f"{b:0b}"
            res += "".join("0" for _ in range(8 - len(bin_repr))) + bin_repr
        return res[self.unused_bits:]

    def __iter__(self) -> Iterator[bool]:
        bits = 0
        for b in self.data:
            for i in range(8):
                bits += 1
                if bits <= self.unused_bits:
                    continue

                yield bool(b & 2 ** (7 - i))

Sprite.write_sprite("bishop", "0000000000000000000000000000000000000000001000000000000000101000000000000010001000000000000101010000000000001000100000000000011011000000000000010100000000000001101100000000000010001000000000001100011000000000110000011000000011000000011000001111111111111000000000000000000000000000000000000")
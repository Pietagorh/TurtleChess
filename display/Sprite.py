from os import path, mkdir, listdir, remove
from typing import Iterator
from bitarray import bitarray
from PIL import Image


class Sprite:
    def __init__(self, name: str) -> None:
        self.sprite_path = f"sprites/{name}.bs"

        with open(self.sprite_path, "rb") as f:
            self.unused_bits = int(f.read(1)[0])
            self.data = f.read()

    @staticmethod
    def convert_image(image_path: str) -> str:
        image = Image.open(image_path).getdata()
        data = ""
        for pixel in image:
            data += "1" if sum(pixel) >= 512 else "0"
        return data

    @staticmethod
    def write_sprite(name: str, data: str, *, is_anim=False) -> None:
        dir = "sprites/"

        if is_anim:
            dir += f"{name}"
            if not path.exists(dir):
                mkdir(dir)
            
            name = len(listdir(dir))

        sprite_path = f"{dir}/{name}.bs"

        if path.exists(sprite_path):
            remove(sprite_path)

        with open(sprite_path, "xb") as f:
            unused_bits = 8 - len(data) + len(data) // 8 * 8
            f.write(unused_bits.to_bytes(1, byteorder="big") + bitarray("".join("0" for _ in range(unused_bits)) + "".join(reversed(list(data)))).tobytes())

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

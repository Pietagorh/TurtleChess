import turtle as t
from config import board_origin, tile_size


class Piece:
    pixel_size = tile_size // 16

    def __init__(self, x, y, is_white: bool):
        self.x = x
        self.y = y
        self.is_white = is_white

    def go_to(self):
        t.penup()
        t.setheading(0)
        t.goto(board_origin + self.x * tile_size, board_origin + self.y * tile_size)
        t.pendown()

    def draw(self):
        self.go_to()
        t.color("#ffffff" if self.is_white else "#0000000")
        with open(f"display/pieces/imgs/{self.__class__.__name__}.bin", "br") as image:
            for i in range(17 ** 2):
                if i % 17 == 0:
                    self.go_to()
                    t.left(90)
                    t.forward(i // 17 * self.pixel_size)
                    t.right(90)
                a = image.read(1)
                print(a)
                if a == b'1':
                    t.pendown()
                self.draw_pixel()
                t.penup()

    def draw_pixel(self):
        for _ in range(4):
            t.forward(self.pixel_size)
            t.left(90)
        t.forward(self.pixel_size)

    def __repr__(self) -> str:
        return f"{self.__class__.__name__} {'blanc' if self.is_white else 'noir'} at ({self.x}, {self.y})"

import turtle as t
from config import board_origin, tile_size
from utils import bits_from_file_end_first


class Piece:
    pixel_size = tile_size // 17

    def __init__(self, x, y, is_white: bool):
        self.x = x
        self.y = y
        self.is_white = is_white

    def go_to(self):
        t.penup()
        t.setheading(0)
        t.goto(board_origin + self.x * tile_size, board_origin + self.y * tile_size)

    def draw(self):
        t.penup()
        t.color("#ffffff" if self.is_white else "#000000")

        for i, pixel in enumerate(bits_from_file_end_first(f"pieces/imgs/{self.__class__.__name__}.bin")):
            if i % 17 == 0:  # place on the right line
                self.go_to()
                t.left(90)
                t.forward(i // 17 * self.pixel_size)
                t.right(90)

            if pixel:
                t.pendown()
                self.draw_pixel()
            t.penup()
            t.forward(self.pixel_size)

    def draw_pixel(self):
        t.begin_fill()
        for _ in range(4):
            t.forward(self.pixel_size - 1)
            t.left(90)
        t.end_fill()

    def __repr__(self) -> str:
        return f"{self.__class__.__name__} {'blanc' if self.is_white else 'noir'} at ({self.x}, {self.y})"

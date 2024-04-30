import turtle
import turtle as t
from config import board_origin, tile_size, piece_colors
from Sprite import Sprite


class Piece:
    pixel_size = tile_size // 17

    def __init__(self, x, y, is_white: bool) -> None:
        self.x = x
        self.y = y
        self.is_white = is_white

        self.sprite = Sprite(self.__class__.__name__)

    def go_to(self) -> None:
        """
        Moves turtle to the bottom left corner of the piece's tile
        """
        is_down = turtle.isdown()
        t.penup()
        t.goto(board_origin + self.x * tile_size, board_origin + self.y * tile_size)
        if is_down:
            turtle.pendown()

    def draw(self) -> None:
        """
        Draws the piece pixel by pixel, left to right then down to up
        """
        t.penup()
        t.color(piece_colors[0] if self.is_white else piece_colors[1])

        for i, pixel in enumerate(self.sprite):
            if i % 17 == 0:  # Place on the right line
                self.go_to()
                t.left(90)
                t.forward(i // 17 * self.pixel_size)
                t.right(90)

            if pixel:
                t.pendown()
                self.draw_pixel()
            t.penup()
            t.forward(self.pixel_size)

    def draw_pixel(self) -> None:
        """
        Draws one pixel from the current turtle position
        """
        t.begin_fill()
        for _ in range(4):
            t.forward(self.pixel_size - 1)
            t.left(90)
        t.end_fill()

    def __repr__(self) -> str:
        return f"{'white' if self.is_white else 'black'} {self.__class__.__name__} at ({self.x}, {self.y})"

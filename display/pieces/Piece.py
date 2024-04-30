import turtle
from config import BOARD_ORIGIN, TILE_SIZE, PIECE_COLORS
from Sprite import Sprite


class Piece:
    pixel_size = TILE_SIZE // 17

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
        turtle.penup()
        turtle.goto(BOARD_ORIGIN + self.x * TILE_SIZE, BOARD_ORIGIN + self.y * TILE_SIZE)
        if is_down:
            turtle.pendown()

    def draw(self) -> None:
        """
        Draws the piece pixel by pixel, left to right then down to up
        """
        turtle.penup()
        turtle.color(PIECE_COLORS[0] if self.is_white else PIECE_COLORS[1])

        for i, pixel in enumerate(self.sprite):
            if i % 17 == 0:  # Place on the right line
                self.go_to()
                turtle.left(90)
                turtle.forward(i // 17 * self.pixel_size)
                turtle.right(90)

            if pixel:
                turtle.pendown()
                self.draw_pixel()
            turtle.penup()
            turtle.forward(self.pixel_size)

    def draw_pixel(self) -> None:
        """
        Draws one pixel from the current turtle position
        """
        turtle.begin_fill()
        for _ in range(4):
            turtle.forward(self.pixel_size - 1)
            turtle.left(90)
        turtle.end_fill()

    def __repr__(self) -> str:
        return f"{'white' if self.is_white else 'black'} {self.__class__.__name__} at ({self.x}, {self.y})"

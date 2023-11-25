import turtle
from config import tile_size


class Board:
    def __init__(self):
        pass  # todo

    @staticmethod
    def draw_tile(x: int, y: int):
        turtle.penup()
        turtle.goto(x * tile_size, y * tile_size)
        turtle.pendown()
        turtle.color("#26611a" if (x + y) % 2 == 0 else "#9cff81")
        turtle.begin_fill()
        for _ in range(4):
            turtle.forward(tile_size)
            turtle.left(90)
        turtle.forward(tile_size + 1)
        turtle.end_fill()

    @staticmethod
    def draw_background():
        turtle.penup()
        turtle.setpos(0, 0)
        turtle.pendown()

        for row in range(8):
            for tile in range(8):
                Board.draw_tile(row, tile)

import turtle
from config import tile_size, board_origin


class Board:

    def __init__(self):
        pass  # todo

    @staticmethod
    def draw_tile(x: int, y: int):
        turtle.penup()
        turtle.goto(board_origin + x * tile_size, board_origin + y * tile_size)
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
        turtle.setpos(board_origin, board_origin)
        turtle.pendown()

        for row in range(8):
            for tile in range(8):
                Board.draw_tile(row, tile)


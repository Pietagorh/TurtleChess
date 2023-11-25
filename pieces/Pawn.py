import turtle
from config import tile_size
from pieces.Piece import Piece


class Pawn(Piece):
    def draw(self):
        distance = (tile_size * 2 - 1) / 2

        turtle.pendown()
        turtle.back(distance / 2)
        for _ in range(4):
            turtle.forward(distance + 1)
            turtle.right(90)
        turtle.forward((distance / 2))

        turtle.back(distance / 4)
        for _ in range(4):
            turtle.forward(distance / 2 + 1)
            turtle.left(90)
        turtle.penup()
        turtle.update()

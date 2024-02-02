import turtle
from config import tile_size
from pieces.Piece import Piece


class Pawn(Piece):
    def draw(self): # tout pété mais c'est reconnaissable
        self.go_to()
        turtle.color(self.color)
        distance = (tile_size - 1) / 2
        turtle.forward(distance // 2)
        turtle.pendown()
        turtle.begin_fill()
        turtle.circle(distance / 2)
        turtle.end_fill()

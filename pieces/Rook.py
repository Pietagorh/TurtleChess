import turtle as t
from config import tile_size
from pieces.Piece import Piece


class Rook(Piece):
    def draw(self):
        t.color(self.color)
        t.penup()
        self.go_to()
        t.left(90)
        t.forward(1 * 2)
        t.right(90)
        t.forward(5 * 2)

        t.pendown()
        t.right(90)
        t.forward(8 * 2)
        t.right(90)
        t.forward(10 * 2)
        t.right(90)
        t.forward(8 * 2)
        t.goto(t.xcor() - (2 * 2), t.ycor() + (2 * 2))
        t.forward(4 * 2)
        for _ in range(2):
            t.right(90)
            t.forward(2 * 2)

        for _ in range(3):
            for _ in range(2):
                t.left(90)
                t.forward(2 * 2)
            for _ in range(2):
                t.right(90)
                t.forward(2 * 2)
        t.forward(2 * 2)
        t.goto(t.xcor() - (2 * 2), t.ycor() - (2 * 2))
        t.penup()

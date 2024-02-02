import turtle
from config import tile_size
from Board import Board
from pieces.Rook import Rook
from pieces.Pawn import Pawn


def setup_turtle():
    turtle.reset()
    turtle.speed(0)
    turtle.hideturtle()
    turtle.tracer(0)
    turtle.title("chess")
    turtle.screensize((tile_size + 1) * 8, (tile_size + 1) * 8, "#000000")
    turtle.setup((tile_size + 1) * 8 + 10, (tile_size + 1) * 8 + 10, 0, 0)

setup_turtle()
board = Board.draw_background()
board.convert_from_bitarray(bitarray())

turtle.mainloop()

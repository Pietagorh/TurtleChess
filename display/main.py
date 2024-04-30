import turtle
from config import TILE_SIZE
from Board import Board
from bitarray import bitarray


def setup_turtle() -> None:
    """
    Sets up the turtle module
    """
    turtle.reset()
    turtle.speed(0)
    turtle.tracer(0)
    turtle.hideturtle()
    turtle.title("chess")
    turtle.screensize((TILE_SIZE + 1) * 8, (TILE_SIZE + 1) * 8, "#000000")
    turtle.setup((TILE_SIZE + 1) * 8 + 10, (TILE_SIZE + 1) * 8 + 10, 0, 0)


setup_turtle()
board = Board()
board.draw_background()
board.convert_from_bitarray(bitarray("10101011110011011110110010111010100110011001100110011001100110010000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"
                                     "00000000000000000000000000000000001000100010001000100010001000100100011010001010110010000110010"))
board.draw_pieces()
turtle.mainloop()

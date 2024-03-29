import turtle
from config import tile_size
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
    turtle.screensize((tile_size + 1) * 8, (tile_size + 1) * 8, "#000000")
    turtle.setup((tile_size + 1) * 8 + 10, (tile_size + 1) * 8 + 10, 0, 0)


setup_turtle()
board = Board()
board.draw_background()
board.convert_from_bitarray(bitarray("1010101111001101111011001011101010011001100110011001100110011001000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001000100010001000100010001000100100011010001010110010000110010"))
board.draw_pieces()
turtle.mainloop()

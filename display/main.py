import turtle
from config import tile_size
from Board import Board
from bitarray import bitarray


def setup_turtle():
    turtle.reset()
    turtle.speed(0)
    turtle.hideturtle()
    turtle.tracer(0)
    turtle.title("chess")
    turtle.screensize((tile_size + 1) * 8, (tile_size + 1) * 8, "#000000")
    turtle.setup((tile_size + 1) * 8 + 10, (tile_size + 1) * 8 + 10, 0, 0)


setup_turtle()
board = Board()
board.draw_background()
board.convert_from_bitarray(bitarray("101010111100110111101100101110101001100110011001100110011001100100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000100010001000100010001000100010010001101000101011001000110010"))
board.draw_pieces()
turtle.mainloop()

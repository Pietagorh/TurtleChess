import turtle
from config import tile_size, board_origin
from bitarray import bitarray
from pieces.Pawn import Pawn
from pieces.Rook import Rook

class Board:
    def __init__(self):
        self.pieces = tuple()


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
    
    def convert_from_bitarray(position: bitarray):
        pieces = [Pawn, Rook]
        for i in range(0, 64, 4):
            self.pieces.add(int(position[i: i + 1]), pieces[int(position[i + 1: i + 4])])

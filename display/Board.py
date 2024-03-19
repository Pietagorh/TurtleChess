import turtle
from config import tile_size, board_origin
from bitarray import bitarray
from bitarray.util import ba2int
from display.pieces.Pawn import Pawn
from display.pieces.Rook import Rook


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

    def convert_from_bitarray(self, position: bitarray):
        pieces = [Pawn, Rook]
        for i in range(0, 64, 4):

            piece_type = ba2int(position[i + 1: i + 4])
            if piece_type != 0:
                piece = pieces[piece_type - 1]

                x, y = divmod(i, 8)
                is_white = bool(ba2int(position[i: i + 1]))

                self.pieces += (piece(x, y, is_white),)
        print(self.pieces)
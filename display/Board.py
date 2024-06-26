import turtle
from bitarray import bitarray
from bitarray.util import ba2int
from config import TILE_SIZE, BOARD_ORIGIN, BOARD_COLORS
from pieces.Bishop import Bishop
from pieces.King import King
from pieces.Knight import Knight
from pieces.Pawn import Pawn
from pieces.Piece import Piece
from pieces.Queen import Queen
from pieces.Rook import Rook


class Board:
    def __init__(self) -> None:
        self.pieces: list[Piece] = []

    @staticmethod
    def draw_tile(x: int, y: int) -> None:
        """
        Draws one empty tile on the chess board.
        :param x: the x position of the tile to draw
        :param y: the y position of the tile to draw
        """
        turtle.penup()
        turtle.goto(BOARD_ORIGIN + x * TILE_SIZE, BOARD_ORIGIN + y * TILE_SIZE)
        turtle.pendown()

        turtle.color(BOARD_COLORS[0] if (x + y) % 2 == 0 else BOARD_COLORS[1])

        turtle.begin_fill()
        for _ in range(4):
            turtle.forward(TILE_SIZE)
            turtle.left(90)
        turtle.end_fill()

    @staticmethod
    def draw_background() -> None:
        """
        Draws all background tiles
        """
        turtle.penup()
        turtle.setpos(BOARD_ORIGIN, BOARD_ORIGIN)
        turtle.pendown()

        for row in range(8):
            for tile in range(8):
                Board.draw_tile(row, tile)

    def draw_pieces(self) -> None:
        """
        Draws all the pieces
        """
        for piece in self.pieces:
            piece.draw()

    def convert_from_bitarray(self, position: bitarray) -> None:
        """
        Converts from the bitarrays sent from core
        :param position: a 256 bits long bitarray (see README)
        """
        pieces = [Pawn, Rook, Knight, Bishop, Queen, King]
        for i in range(0, 64 * 4, 4):
            piece_type = ba2int(position[i + 1: i + 4])
            if piece_type != 0:
                piece = pieces[piece_type - 1]

                y, x = divmod(i // 4, 8)
                is_white = bool(ba2int(position[i: i + 1]))

                self.pieces.append(piece(x, y, is_white))

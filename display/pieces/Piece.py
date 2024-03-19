from abc import abstractmethod
import turtle as t
from display.config import board_origin, tile_size


class Piece:
    def __init__(self, x, y, is_white: bool):
        self.x = x
        self.y = y
        self.is_white = is_white

    def go_to(self):
        t.penup()
        t.setheading(0)
        t.goto(board_origin + self.x * tile_size + tile_size / 2, board_origin + self.y * tile_size  + tile_size / 2)
        #on place au milieu de la case

    def draw(self):
        pass

    def __repr__(self) -> str:
        return f"{self.__class__.__name__} {'blanc' if self.is_white else 'noir'} at ({self.x}, {self.y})"

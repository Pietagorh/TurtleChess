from abc import abstractmethod
import turtle as t
from config import board_origin, tile_size


class Piece:
    def __init__(self, x, y, color):
        self.x = x
        self.y = y
        self.color = color

    def go_to(self):
        t.goto(board_origin + self.x * tile_size + tile_size / 2, board_origin + self.y * tile_size  + tile_size / 2)
        #on place au milieu de la case

    @abstractmethod
    def draw(self):
        pass

from abc import abstractmethod
import turtle as t
from config import board_origin, tile_size


class Piece:
    def __init__(self, x, y, is_white: bool):
        self.x = x
        self.y = y
        self.color = "#ffffff" if is_white else "#000000" 

    def go_to(self):
        t.penup()
        t.setheading(0)
        t.goto(board_origin + self.x * tile_size + tile_size / 2, board_origin + self.y * tile_size  + tile_size / 2)
        #on place au milieu de la case

    @abstractmethod
    def draw(self):
        pass

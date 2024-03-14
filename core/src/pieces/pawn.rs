use crate::board::board::Board;
use crate::pieces::pieces::{Color, Piece, retrieve_color_from_int};

pub struct Pawn{
    x: u8,
    y: u8,
    color: Color
}

impl Piece for Pawn{
    fn new(x: u8, y: u8, color: Color) -> Self {
        Pawn {
            x,
            y,
            color,
        }
    }

    fn binary_image() -> u8 {
        1
    }

    fn get_x(&self) -> &u8 {
        &self.x
    }

    fn get_y(&self) -> &u8 {
        &self.y
    }

    fn set_x(&mut self, x: u8) {
        self.x = x;
    }

    fn set_y(&mut self, y: u8) {
        self.y = y;
    }

    fn get_color(&self) -> &Color {
        &self.color
    }

    fn can_reach(&self, board: &Board, x: u8, y: u8) -> bool{
        let direction: i8;
        if self.color == Color::WHITE{
            direction = 1;
        }else{
            direction = -1;
        }
        if (x == self.x) & (y == (self.y as i8 + direction) as u8) & (board.get(x, y) == 0){
            return true;
        }
        if ((x == self.x + 1) | (x == self.x - 1)) & (y == (self.y as i8 + direction) as u8)
            & (board.get(x, y) != 0)
            & (retrieve_color_from_int(board.get(x, y)) != self.color) {
            return true;
        }
        //TODO gérer la position initiale pour le déplacement de 2
        false
    }
}
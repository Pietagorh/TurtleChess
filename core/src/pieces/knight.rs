use crate::board::board::Board;
use crate::pieces::pieces::{Color, Piece, retrieve_piece_from_int};

pub struct Knight{
    x: u8,
    y: u8,
    color: Color
}

impl Piece for Knight{
    fn new(x: u8, y: u8, color: Color) -> Self {
        Knight {
            x,
            y,
            color,
        }
    }

    fn binary_image(&self) -> u8 {
        3
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

    fn can_reach(&self, board: &Board, x: u8, y: u8) -> bool {
        let piece = board.get(x, y);
        if (((((x - self.x) as i8).abs() == 2) & (((y - self.y) as i8).abs() == 1))
            | ((((x - self.x) as i8).abs() == 1) & (((y - self.y) as i8).abs() == 2)))
            & (*piece == 0) | (*retrieve_piece_from_int(piece).get_color() != self.color){
            return true;
        }
        false
    }
}
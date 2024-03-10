use crate::board::board::Board;
use crate::pieces::pieces::{Color, Piece, retrieve_color_from_int};

struct Knight{
    x: u8,
    y: u8,
    color: Color
}

impl Piece for Knight{
    fn binary_image() -> u8 {
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

    fn can_reach(&self, board: Board, x: u8, y: u8) -> bool {
        if ((((x - self.x).abs() == 2) & ((y - self.y).abs() == 1))
            | (((x - self.x).abs() == 1) & ((y - self.y).abs() == 2)))
            & (retrieve_color_from_int(board.get(x, y)) != self.color){
            true
        }
        false
    }
}
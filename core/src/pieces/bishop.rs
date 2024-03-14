use crate::board::board::Board;
use crate::pieces::pieces::{Color, DiagWalker, Piece};

pub struct Bishop{
    x: u8,
    y: u8,
    color: Color
}

impl DiagWalker for Bishop {}
impl Piece for Bishop {
    fn new(x: u8, y: u8, color: Color) -> Self {
        Bishop {
            x,
            y,
            color,
        }
    }

    fn binary_image() -> u8 {
        4
    }

    fn get_x(&self) -> &u8 {
        &self.x
    }

    fn get_y(&self) -> &u8 {
        &self.y
    }

    fn set_x(&mut self, x: u8) {
        self.x = x
    }

    fn set_y(&mut self, y: u8) {
        self.y = y;
    }

    fn get_color(&self) -> &Color {
        &self.color
    }

    fn can_reach(&self, board: &Board, x: u8, y: u8) -> bool {
        self.can_reach_on_diag(board, x, y)
    }
}
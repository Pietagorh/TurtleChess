use crate::board::board::Board;
use crate::pieces::pieces::{Color, ColumnWalker, DiagWalker, Piece};

pub struct Queen{
    x: u8,
    y: u8,
    color: Color
}

impl ColumnWalker for Queen {}

impl DiagWalker for Queen {}

impl Piece for Queen {
    fn new(x: u8, y: u8, color: Color) -> Self {
        Queen {
            x,
            y,
            color,
        }
    }

    fn binary_image() -> u8 {
        5
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
        self.y = y
    }

    fn get_color(&self) -> &Color {
        &self.color
    }

    fn can_reach(&self, board: &Board, x: u8, y: u8) -> bool {
        self.can_reach_on_column(board, x, y) | self.can_reach_on_diag(board, x, y)
    }
}
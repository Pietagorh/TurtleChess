use crate::board::board::Board;
use crate::pieces::pieces::{Castleable, Color, ColumnWalker, DiagWalker, Piece};

pub struct King{
    x: u8,
    y: u8,
    color: Color,
    has_moved: bool
}

impl DiagWalker for King {}

impl ColumnWalker for King {}

impl Castleable for King {
    fn can_castle(&self, board: &Board, other_piece_x: u8) -> bool {
        todo!()
    }

    fn castle(&mut self, board: &mut Board, respective_piece_x: u8) {
        todo!()
    }

    fn set_has_moved(&mut self, has_moved: bool) {
        self.has_moved = has_moved;
    }
}

impl Piece for King {
    fn new(x: u8, y: u8, color: Color) -> Self {
        King {
            x,
            y,
            color,
            has_moved: false,
        }
    }

    fn binary_image() -> u8 {
        6
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
        let x_difference = ((x - self.x) as i8).abs();
        let y_difference = ((y - self.y) as i8).abs();
        if !((x_difference < 2) & (y_difference < 2)) {
            return false;
        }
        if !(self.can_reach_on_diag(board, x, y) | self.can_reach_on_column(board, x, y)){
            //TODO check if it's castle move
            return false;
        }
        true
    }
}
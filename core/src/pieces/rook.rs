use crate::board::board::Board;
use crate::pieces::pieces::{Castleable, Color, ColumnWalker, Piece};

pub struct Rook{
    x: u8,
    y: u8,
    color: Color,
    has_moved: bool
}

impl ColumnWalker for Rook {}

impl Castleable for Rook{
    fn can_castle(&self, board: &Board, knight_x: u8) -> bool{
        if !self.has_moved {
            let difference = ((knight_x - self.x) as i8).abs();
            let direction = (knight_x - self.x) as i8 / difference;
            for i in 1 ..difference {
                if *board.get((self.x as i8 + i * direction) as u8, self.y) == 0{
                    return false;
                }
            }
            return true;
        }
        false
    }

    fn castle(&mut self, board: &mut Board, respective_piece_x: u8) {
        let difference: i8 = (respective_piece_x - self.x) as i8;
        let direction = difference / difference.abs();
        Castleable::move_to(self, board, (respective_piece_x as i8 - direction) as u8, self.y);
    }

    fn set_has_moved(&mut self, has_moved: bool) {
        self.has_moved = has_moved;
    }
}

impl Piece for Rook{
    fn new(x: u8, y: u8, color: Color) -> Self {
        Rook {
            x,
            y,
            color,
            has_moved: false,
        }
    }

    fn binary_image() -> u8 {
        2
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
        self.can_reach_on_column(board, x, y)
    }
}
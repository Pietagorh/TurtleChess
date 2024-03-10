use crate::board::board::Board;
use crate::pieces::pieces::{Castleable, Color, Piece};

struct Rook{
    x: u8,
    y: u8,
    color: Color,
    has_moved: bool
}

impl Castleable for Rook{
    fn can_castle(&self, board: Board, knight_x: u8) -> bool{
        if !self.has_moved {
            let difference = (knight_x - self.x).abs();
            let direction = (knight_x - self.x) / difference;
            for i in 1 ..difference {
                if board.get(self.x + i * direction, self.y) != 0{
                    false
                }
            }
            true
        }
        false
    }

    fn castle(&mut self, board: Board, respective_piece_x: u8) {
        let difference = (respective_piece_x - self.x);
        let direction = difference / difference.abs();
        self.move_to(board, respective_piece_x - direction, self.y);
    }
}

impl Piece for Rook{
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

    fn can_reach(&self, board: Board, x: u8, y: u8) -> bool {
        if x == self.x {
            let difference = (y - self.y).abs();
            let direction = (y - self.y) / difference;
            for i in 1 ..= difference {
                if(board.get(x, self.y + i * direction) != 0) {
                    false
                }
            }
            true
        }
        if y == self.y {
            let difference = (x - self.x).abs();
            let direction = (x - self.x) / difference;
            for i in 1 ..= difference {
                if(board.get(self.x + i * direction, y) != 0) {
                    false
                }
            }
            true
        }
        false
    }

    fn move_to(&mut self, board: Board, x: u8, y: u8) {
        super:self.move_to(board, x, y);
        self.has_moved = true;
    }
}
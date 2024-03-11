use crate::board::board::Board;

pub enum Color{
    WHITE,
    BLACK
}

pub trait Piece {
    fn binary_image() -> u8;
    fn get_x(&self) -> &u8;
    fn get_y(&self) -> &u8;
    fn set_x(&mut self, x: u8);
    fn set_y(&mut self, y: u8);
    fn get_color(&self) -> &Color;
    fn can_reach(&self, board: &Board, x: u8, y: u8) -> bool;
    fn move_to(&mut self, board: &Board, x: u8, y: u8){
        if(self.can_reach(board, x, y)){
            self.set_x(x);
            self.set_y(y);
            board.move_piece(self.get_x(), self.get_y(), x, y)
        }
    }
}

pub trait Castleable {
    fn can_castle(&self, board: &Board, other_piece_x: u8) -> bool;
    fn castle(&self, board: &Board, respective_piece_x: u8);
}

pub trait DiagWalker: Piece {
    fn can_reach_on_diag(&self, board: &Board, x: u8, y: u8) -> bool {
        let x_difference = x - self.get_x();
        let y_difference = y - self.get_y();
        if(y_difference.abs() != x_difference.abs()){
            false
        }
        let x_direction = x_difference / x_difference.abs();
        let y_direction = y_difference / y_difference.abs();
        for i in 1..x_difference.abs() {
            if board.get(self.get_x() + x_direction * i, self.get_y() + y_direction * i) != 0{
                false
            }
        }
        let piece = board.get(x, y);
        if (piece != 0) & (retrieve_color_from_int(piece) != self.get_color()) {
            false
        }
        true
    }
}

pub trait ColumnWalker: Piece {
    fn can_reach_on_column(&self, board: &Board, x: u8, y: u8) -> bool {
        if x == *self.get_x() {
            let difference = (y - self.get_y()).abs();
            let direction = (y - self.get_y()) / difference;
            for i in 1 ..= difference {
                let piece = board.get(x, self.get_y() + i * direction);
                if (piece != 0) & (retrieve_color_from_int(piece) != self.get_color()) {
                    false
                }
            }
            true
        }
        if y == self.get_y() {
            let difference = (x - self.get_x()).abs();
            let direction = (x - self.get_x()) / difference;
            for i in 1 ..= difference {
                let piece = board.get(self.get_x() + i * direction, y);
                if (piece != 0) & (retrieve_color_from_int(piece) != self.get_color()) {
                    false
                }
            }
            true
        }
        false
    }
}

struct King{
    x: u8,
    y: u8,
    color: Color,
    has_moved: bool
}

fn retrieve_piece_from_int(p: u8) -> Box<dyn Piece> {
    let color= retrieve_color_from_int(p);
    //TODO
}

pub fn retrieve_color_from_int(i: u8) -> Color{
    if i << 3 == 0{
        Color::WHITE
    }else{
        Color::BLACK
    }
}
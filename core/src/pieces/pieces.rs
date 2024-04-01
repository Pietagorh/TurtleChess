use crate::board::board::Board;
use crate::pieces::pawn::Pawn;

#[derive(PartialEq)]
pub enum Color{
    WHITE,
    BLACK,
    NONE
}

pub trait Piece {
    fn new(x: u8, y: u8, color: Color) -> Self where Self: Sized;
    fn binary_image() -> u8 where Self: Sized;
    fn get_x(&self) -> &u8;
    fn get_y(&self) -> &u8;
    fn set_x(&mut self, x: u8);
    fn set_y(&mut self, y: u8);
    fn get_color(&self) -> &Color;
    fn can_reach(&self, board: &Board, x: u8, y: u8) -> bool;
    fn move_to(&mut self, board: &mut Board, x: u8, y: u8){
        if self.can_reach(board, x, y) {
            self.set_x(x);
            self.set_y(y);
            board.move_piece(self.get_x(), self.get_y(), x, y)
        }
    }
}

pub trait Castleable: Piece {
    fn can_castle(&self, board: &Board, other_piece_x: u8) -> bool;
    fn castle(&mut self, board: &mut Board, respective_piece_x: u8);
    fn set_has_moved(&mut self, has_moved: bool);
    fn move_to(&mut self, board: &mut Board, x: u8, y: u8) {
        if self.can_reach(board, x, y) {
            self.set_x(x);
            self.set_y(y);
            board.move_piece(self.get_x(), self.get_y(), x, y);
            self.set_has_moved(true);
        }
    }
}

pub trait DiagWalker: Piece {
    fn can_reach_on_diag(&self, board: &Board, x: u8, y: u8) -> bool {
        let x_difference: i8 = (x - self.get_x()) as i8;
        let y_difference: i8 = (y - self.get_y()) as i8;
        if y_difference.abs() != x_difference.abs() {
            return false
        }
        let x_direction = x_difference / x_difference.abs();
        let y_direction = y_difference / y_difference.abs();
        for i in 1..x_difference.abs() {
            if *board.get((*self.get_x() as i8 + x_direction * i) as u8, (*self.get_y() as i8 + y_direction * i) as u8) == 0{
                return false;
            }
        }
        let piece = board.get(x, y);
        if (*piece != 0) & (*retrieve_piece_from_int(piece).get_color() != *self.get_color()) {
            return false;
        }
        true
    }
}

pub trait ColumnWalker: Piece {
    fn can_reach_on_column(&self, board: &Board, x: u8, y: u8) -> bool {
        if x == *self.get_x() {
            let difference = ((y - self.get_y()) as i8).abs();
            let direction: i8 = (y - self.get_y()) as i8 / difference;
            for i in 1 ..= difference {
                let piece = board.get(x, (*self.get_y() as i8 + i * direction) as u8);
                if (*piece != 0) & (*retrieve_piece_from_int(piece).get_color() != *self.get_color()) {
                    return false;
                }
            }
            return true;
        }
        if y == *self.get_y() {
            let difference = ((x - self.get_x()) as i8).abs();
            let direction = (x - self.get_x()) as i8 / difference;
            for i in 1 ..= difference {
                let piece = board.get((*self.get_x() as i8 + i * direction) as u8, y);
                if (*piece != 0) & (*retrieve_piece_from_int(piece).get_color() != *self.get_color()) {
                    return false;
                }
            }
            return true;
        }
        false
    }
}

pub fn retrieve_piece_from_int(p: &u8) -> Box<dyn Piece> {
    let color= retrieve_color_from_int(p);
    //TODO
    return Box::new(Pawn::new(0, 0, Color::WHITE)); //temp
}

pub fn retrieve_color_from_int(i: &u8) -> Color{
    if i << 3 == 0{
        Color::WHITE
    }else{
        Color::BLACK
    }
}
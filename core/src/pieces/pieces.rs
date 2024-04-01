use crate::board::board::Board;
use crate::pieces::pawn::Pawn;
use std::collections::HashMap;
use std::ops::Deref;
use crate::pieces::bishop::Bishop;
use crate::pieces::king::King;
use crate::pieces::knight::Knight;
use crate::pieces::queen::Queen;
use crate::pieces::rook::Rook;

#[derive(PartialEq)]
pub enum Color{
    WHITE,
    BLACK,
    NONE
}

const BINARY_IMAGES: HashMap<u8, dyn FnOnce(u8, u8, Color) -> dyn Piece> = HashMap::from(
    [
        (1, |x, y, color| Pawn::new(x, y, color)),
        (2, |x, y, color| Rook::new(x, y, color)),
        (3, |x, y, color| Knight::new(x, y, color)),
        (4, |x, y, color| Bishop::new(x, y, color)),
        (5, |x, y, color| Queen::new(x, y, color)),
        (6, |x, y, color| King::new(x, y, color))
    ]
);

pub trait Piece {
    fn new(x: u8, y: u8, color: Color) -> Self where Self: Sized;
    fn binary_image(&self) -> u8 ;
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
        if (*piece != 0) & (*retrieve_piece_from_int(piece, x, y).get_color() != *self.get_color()) {
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
                let y: u8 = (*self.get_y() as i8 + i * direction) as u8;
                let piece = board.get(x, y);
                if (*piece != 0) & (*retrieve_piece_from_int(piece, x, y).get_color() != *self.get_color()) {
                    return false;
                }
            }
            return true;
        }
        if y == *self.get_y() {
            let difference = ((x - self.get_x()) as i8).abs();
            let direction = (x - self.get_x()) as i8 / difference;
            for i in 1 ..= difference {
                let x: u8 = (*self.get_x() as i8 + i * direction) as u8;
                let piece = board.get(x, y);
                if (*piece != 0) & (*retrieve_piece_from_int(piece, x, y).get_color() != *self.get_color()) {
                    return false;
                }
            }
            return true;
        }
        false
    }
}

pub fn retrieve_piece_from_int(p: &u8, x: u8, y: u8) -> Box<dyn Piece> {
    let color= retrieve_color_from_int(p);
    let binary_image = (p << 5) >> 5;
    return Box::new(BINARY_IMAGES.get(binary_image).expect("Could not read a piece value")(x, y, color));
}

pub fn retrieve_color_from_int(i: &u8) -> Color{
    if i >> 3 == 0{
        Color::WHITE
    }else{
        Color::BLACK
    }
}
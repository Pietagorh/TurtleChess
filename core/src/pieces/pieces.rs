use crate::board::board::Board;

pub enum Color{
    WHITE,
    BLACK
}

pub trait Piece{
    fn binary_image() -> u8;
    fn get_x(&self) -> u8;
    fn get_y(&self) -> u8;
    fn get_color(&self) -> Color;
    fn can_reach(&self, board: Board, x: u8, y: u8) -> bool;
    fn move_to(&self, board: Board, x: u8, y: u8);
}

struct Rook{
    x: u8,
    y: u8,
    color: Color,
    has_moved: bool
}

struct Knight{
    x: u8,
    y: u8,
    color: Color
}

struct Bishop{
    x: u8,
    y: u8,
    color: Color
}

struct Queen{
    x: u8,
    y: u8,
    color: Color
}

struct King{
    x: u8,
    y: u8,
    color: Color,
    has_moved: bool
}

fn retrieve_piece_from_int(p: u8) -> Box<dyn Piece> {
    let color = retrieve_color_from_int(p);
    //TODO
}

pub(crate) fn retrieve_color_from_int(i: u8) -> Color{
    if i << 3 == 0{
        Color::WHITE
    }else{
        Color::BLACK
    }
}
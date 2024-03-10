use crate::board::board::Board;

pub enum Color{
    WHITE,
    BLACK
}

pub trait Piece{
    fn binary_image() -> u8;
    fn get_x(&self) -> &u8;
    fn get_y(&self) -> &u8;
    fn set_x(&mut self, x: u8);
    fn set_y(&mut self, y: u8);
    fn get_color(&self) -> &Color;
    fn can_reach(&self, board: Board, x: u8, y: u8) -> bool;
    fn move_to(&mut self, board: Board, x: u8, y: u8){
        if(self.can_reach(board, x, y)){
            self.set_x(x);
            self.set_y(y);
            board.move_piece(self.get_x(), self.get_y(), x, y)
        }
    }
}

pub trait Castleable{
    fn can_castle(&self, board: Board, other_piece_x: u8) -> bool;
    fn castle(&self, board: Board, respective_piece_x: u8);
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
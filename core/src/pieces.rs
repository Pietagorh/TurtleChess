pub mod pieces;

enum Color{
    WHITE,
    BLACK
}

trait Piece{
    fn binary_image() -> u8;
    fn get_x(&self) -> u8;
    fn get_y(&self) -> u8;
    fn get_color(&self) -> Color;
    fn can_reach(&self, board: Board, x: u8, y: u8) -> bool;
    fn move_to(&self, board: Board, x: u8, y: u8);
}

struct Pawn{
    x: u8,
    y: u8,
    color: Color
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

impl Piece for Pawn{
    fn binary_image() -> u8 {
        1
    }
    fn get_x(&self) -> u8 {
        x
    }
    fn get_y(&self) -> u8 {
        y
    }
    fn get_color(&self) -> Color {
        color
    }
    fn can_reach(&self, board: Board, x: u8, y: u8) -> bool{
        let direction;
        if self.color == Color:WHITE{
            direction = 1;
        }else{
            direction = -1;
        }
        if (x == self.x) & (y == self.y + direction) & (board.get(x, y) == 0){
            true
        }
        if ((x == self.x + 1) | (x == self.x - 1)) & (y == self.y + direction)
            & (retrieve_color_from_int(board.get(x, y)) != color){
            true
        }
        //TODO gérer la position initiale pour le déplacement de 2
        false
    }
}

fn retrieve_piece_from_int(p: u8) -> Box<dyn Piece> {
    color = retrieve_color_from_int(p);
    //TODO
}

fn retrieve_color_from_int(i: u8) -> Color{
    if i << 3 == 0{
        Color::WHITE
    }else{
        Color::BLACK
    }
}
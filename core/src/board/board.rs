use crate::pieces::pieces::Piece;

static EDGE_SIZE: u8 = 8;

pub struct Board{
    cases: Vec<u8>
}

pub struct BoardBuffer{
    head: u128,
    tail: u128
}

impl Board{
    pub fn new() -> Board{
        let mut cases: Vec<u8> = Vec::new();
        for i in 0..64{
            cases[i] = 0;
        }
        Board{
            cases
        }
    }

    pub fn get(&self, x: u8, y: u8) -> &u8 {
        &self.cases[(Board::get_index_from_coord(x, y)) as usize]
    }

    fn get_index_from_coord(x: u8, y: u8) -> usize {
        (EDGE_SIZE * y + x) as usize
    }

    pub fn move_piece(&mut self, x_from: &u8, y_from: &u8, x_to: u8, y_to: u8){
        self.cases[Board::get_index_from_coord(x_to, y_to)] = self.cases[Board::get_index_from_coord(*x_from, *y_from)];
        self.cases[Board::get_index_from_coord(*x_from, *y_from)] = 0;
    }

    pub fn to_buffer(&self) -> BoardBuffer {
        let mut head: u128 = 0;
        let mut tail: u128 = 0;
        let half_board_size = (EDGE_SIZE * EDGE_SIZE) / 2;
        for i in 1..=half_board_size {
            head |= (self.cases[(i - 1) as usize] << 128 - (i * 4)) as u128
        }
        for i in 1..=half_board_size {
            tail |= (self.cases[(i - 1 + half_board_size) as usize] << 128 - (i * 4)) as u128
        }
        BoardBuffer{
            head,
            tail
        }
    }
}
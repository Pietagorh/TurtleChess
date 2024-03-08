use std::vec;

pub mod board;

static EDGE_SIZE: i32 = 8;

struct Board{
    cases: [u8; 64]
}

impl Board{
    pub fn new() -> Board{
        let cases: Vec<u8> = Vec::new();
        for i in 0..64{
            cases[i] = 0;
        }
        Board{
            cases
        }
    }

    pub fn get(&self, x: u8, y: u8) -> u8{
        self.cases[y * EDGE_SIZE + x]
    }
}
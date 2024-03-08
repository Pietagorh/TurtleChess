static EDGE_SIZE: i32 = 8;

pub(crate) struct Board{
    cases: Vec<u8>
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
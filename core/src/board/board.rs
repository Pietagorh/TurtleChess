static EDGE_SIZE: u8 = 8;

pub struct Board{
    cases: Vec<u8>
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

    pub fn get(&self, x: u8, y: u8) -> u8{
        self.cases[(y * EDGE_SIZE + x) as usize]
    }

    pub fn move_piece(&self, x_from: &u8, y_from: &u8, x_to: u8, y_to: u8){
        //TODO
    }
}
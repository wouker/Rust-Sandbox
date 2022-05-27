// a Tetris-playfield is called a well
// the playfield is 10 columns wide and 24 rows high
// so we create a 10x24-matrix
// (inner array = rows - spread over 24 columns)
pub type Well = [[u8; 10]; 24];

fn empty_well() -> Well {
    [[0u8; 10]; 24]
}

pub trait NewWell {
    fn new() -> Self;
}

impl NewWell for Well {
    fn new() -> Well {
        empty_well()
    }
}

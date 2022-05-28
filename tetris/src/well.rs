// a Tetris-playfield is called a well
// the playfield is 10 columns wide and 24 rows high
// so we create a 10x24-matrix
// inner array = rows - spread over 24 columns
// low row = high up the well
pub type Well = [[u8; 10]; 24];

const START_ROW : u8 = 2;
const START_COL : u8 = 3;

pub struct WellPoint {
    pub row_ix: u8,
    pub col_ix: u8
}

pub trait WellDefaults {
    fn new() -> Self;   
    fn get_start_position(&self) -> WellPoint;
}

impl WellDefaults for Well {
    fn new() -> Well {
        empty_well()
    }

    fn get_start_position(&self) -> WellPoint {
        WellPoint { row_ix: START_ROW, col_ix: START_COL }
    }
}

fn empty_well() -> Well {
    [[0u8; 10]; 24]
}

#[cfg(test)]
mod tests {
    use super::*;
        
    #[test]
    fn well_new() {
        let new_well: Well = WellDefaults::new();

        assert_eq!(new_well, [[0u8; 10]; 24]);            
    }

    #[test]
    fn well_starting_positions() {
        let new_well: Well = WellDefaults::new();
        let starting_position = WellDefaults::get_start_position(&new_well);

        assert_eq!(starting_position.row_ix, START_ROW);
        assert_eq!(starting_position.col_ix, START_COL);      
    }     
}
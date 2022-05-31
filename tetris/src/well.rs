// a Tetris-playfield is called a well
// low row = high up the well
// normally 10 rows by 24 cols, but we go for 14 by 20
pub type Well = [[u8; WELL_COLUMN_COUNT]; WELL_ROW_COUNT];

pub const WELL_COLUMN_COUNT : usize = 14;
pub const WELL_ROW_COUNT : usize = 28;

pub const START_ROW : u8 = 1;
pub const START_COL : u8 = 5;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct WellPoint {
    pub row_ix: u8,
    pub col_ix: u8
}

pub trait WellDefaults {    
    fn new(default_value : u8) -> Self;   
    fn get_start_position(&self) -> WellPoint;
}

impl WellDefaults for Well {
    fn new(default_value : u8) -> Well {
        empty_well(default_value)
    }

    fn get_start_position(&self) -> WellPoint {
        //our const is 1-based for readability, our index 0-based (row 1 = index 0)
        WellPoint { row_ix: START_ROW, col_ix: START_COL }
    }
}

fn empty_well(default : u8) -> Well {
    [[default; WELL_COLUMN_COUNT]; WELL_ROW_COUNT]
}

#[cfg(test)]
mod tests {
    use super::*;
        
    #[test]
    fn well_new_empty() {
        let new_well: Well = WellDefaults::new(0);

        assert_eq!(new_well, [[0u8; WELL_COLUMN_COUNT]; WELL_ROW_COUNT]);            
    }

    #[test]
    fn well_new_filled() {
        let new_well: Well = WellDefaults::new(1);

        assert_eq!(new_well, [[1u8; WELL_COLUMN_COUNT]; WELL_ROW_COUNT]);            
    }

    #[test]
    fn well_starting_positions() {
        let new_well: Well = WellDefaults::new(0);
        let starting_position = WellDefaults::get_start_position(&new_well);

        assert_eq!(starting_position.row_ix, START_ROW);
        assert_eq!(starting_position.col_ix, START_COL);      
    }     
}
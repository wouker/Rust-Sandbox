use crate::block::Block;

// a Tetris-playfield is called a well
// low row = high up the well
// normally 10 rows by 24 cols, but we go for 14 by 20
pub type Well = [[u8; WELL_COLUMN_COUNT]; WELL_ROW_COUNT];

pub const WELL_COLUMN_COUNT : usize = 14;
pub const WELL_ROW_COUNT : usize = 28;

pub const START_ROW : i8 = 1;
pub const START_COL : i8 = 5;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct WellPoint {
    //a column & row can be negative as it indicates the topleft of a block. so we can go to -3 if the actual parts are on the outer sides
    pub row_ix: i8,    
    pub col_ix: i8
}

pub trait WellDefaults {    
    fn new(default_value : u8) -> Self;   
    //todo wouter: can we handle this without reference to self? (because we don't need it)
    fn get_start_position(&self) -> WellPoint;
}

impl WellDefaults for Well {
    fn new(default_value : u8) -> Well {
        empty_well(default_value)
    }

    fn get_start_position(&self) -> WellPoint {
        //our const is 1-based for readability, our index 0-based (row 1 = index 0)
        WellPoint { row_ix: START_ROW as i8, col_ix: START_COL as i8 }
    }
}

pub trait Freeze {
    fn freeze_block(&mut self, current_block : &Block, position : &WellPoint);
}

impl Freeze for Well {
    fn freeze_block(&mut self, current_block : &Block, position : &WellPoint) {
        //get coördinates of current blockparts and save them to well
        for (i, row) in current_block.shape.into_array().iter().enumerate() {            
            for (j, value) in row.iter().enumerate() {                
                // if no part on this position of the block, we can't hit anything
                if value == &0u8 {                
                    continue;
                }
                let well_row_ix = (i as i8 + position.row_ix) as usize;
                let well_col_ix = (j as i8 + position.col_ix) as usize;
                self[well_row_ix][well_col_ix] = 1;
            }
        }
    }
}

fn empty_well(default : u8) -> Well {
    [[default; WELL_COLUMN_COUNT]; WELL_ROW_COUNT]
}

#[cfg(test)]
mod tests {
    use crate::block::BlockType;

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

    #[test]
    fn well_freeze() {
        let new_well: &mut Well = &mut WellDefaults::new(0);

        let block = Block::new(BlockType::I);
        let position = WellPoint { row_ix: 20, col_ix: 0 };

        Freeze::freeze_block(new_well, &block, &position);

        assert_eq!(new_well[19][2], 0);
        assert_eq!(new_well[20][0], 0);
        assert_eq!(new_well[20][1], 0);
        assert_eq!(new_well[20][2], 1);
        assert_eq!(new_well[21][0], 0);  
        assert_eq!(new_well[21][1], 0);  
        assert_eq!(new_well[21][2], 1);
        assert_eq!(new_well[22][2], 1);
        assert_eq!(new_well[23][2], 1);
    }
}
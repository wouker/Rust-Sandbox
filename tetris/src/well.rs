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
        //todo wouter
        for (i, row) in current_block.shape.into_array().iter().enumerate() {            
            for (j, value) in row.iter().enumerate() {                
                // if no part on this position of the block, we can't hit anything
                if value == &0u8 {                
                    continue;
                }
                let well_row_ix = i + position.row_ix as usize;
                let well_col_ix = j + position.col_ix as usize;
                self[well_row_ix][well_col_ix] = *value;
            }
        }
        /*/// Copies the given tetrimino's squares into the given well at the given (well_row, well_col).
fn freeze_to_well(ttmo: &Tetrimino, well: &mut Well, well_row: &i32, well_col: &i32)
{
    for row in 0..4 {
        for col in 0..4 {
            if ttmo.shape[row][col] == 0 { continue; }
            // println!("well[{}][{}] = 1", (*well_row + row as i32) as usize, (*well_col + col as i32) as usize);
            well[(*well_row + row as i32) as usize][(*well_col + col as i32) as usize] = ttmo.shape[row][col];
        }
    }
} */
        /*for (i, row) in block.shape.into_array().iter().enumerate() {            
        for (j, value) in row.iter().enumerate() {                
            // if no part on this position of the block, we can't hit anything
            if value == &0u8 {                
                continue;
            }

            //if a part is found, we need to calculate the exact position in the well it will be
            //the new_block_point always refers to the topleft part of a block (even if that part is empty)
            let new_well_row_ix = new_block_point.row_ix + i as i8;
            let new_well_col_ix = new_block_point.col_ix  + j as i8;
            
            //check if these coördinates lay inside the bounds of the well
            if new_well_row_ix == WELL_ROW_COUNT as i8 {
                //hits bottom
                return true;
            }
            if new_well_col_ix == -1 || new_well_col_ix == WELL_COLUMN_COUNT as i8 {
                return true;
            }

            //todo: check if these coördinates are not yet taken by another part            
            //if well[well_row as usize][well_col as usize] != 0 { return true; }
        }
    } */
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
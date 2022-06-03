use crate::{well::{WellPoint, WELL_COLUMN_COUNT, WELL_ROW_COUNT, Well}, block::Block};
use conditional::conditional;

pub fn move_block(block: &Block, well: &Well, current_point : &mut WellPoint, is_left : bool) {
    //a new wellpoint can lay outside the bounds of the well
    let new_col_ix = current_point.col_ix + conditional![is_left ? -1i8 : 1i8];
    let new_well_point = WellPoint { row_ix:current_point.row_ix, col_ix: new_col_ix };
    
    if !is_move_blocked(block, well, new_well_point)  {
        current_point.col_ix = new_col_ix;
    }
}

pub fn is_move_blocked(block : &Block, well: &Well, new_block_point: WellPoint) -> bool {
    //we move a block. potentially all items can be in a single row or a single column, on either side of the block
    //so we need to find the most left - right - top and bottom of a piece and check if we do not hit anything (we just iterate all found parts till we hit something)
    //this can be the border of the well or an part of another block in the well
    //at first thought, you would only need to check collissions for the parts in the movedirection, but we can navigate under/into holes of existing parts between 2 drops
    
    for (i, row) in block.shape.into_array().iter().enumerate() {            
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

            if well[new_well_row_ix as usize][new_well_col_ix as usize] == 1u8 {
                return true;
            }
        }
    }

    //if nothing hits: we are fine
    false
}


//todo wouter: unittest this shit
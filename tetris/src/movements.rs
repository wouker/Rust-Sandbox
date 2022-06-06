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

//todo wouter: when O-block hits bottom, we crash for some reason...

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::well::WellDefaults;
    use crate::block::BlockType;

    use super::*;

    #[test]
    fn move_block_left() {
        let block = Block::new(crate::block::BlockType::O);
        let block_point = &mut WellPoint { row_ix: 10, col_ix: 5 };
        let well = &mut WellDefaults::new(0);

        move_block(&block, well, block_point, true);

        assert_eq!(block_point.col_ix, 4);
        assert_eq!(block_point.row_ix, 10);
    }

    #[test]
    fn move_block_right() {
        let block = Block::new(crate::block::BlockType::O);
        let block_point = &mut WellPoint { row_ix: 10, col_ix: 5 };
        let well = &mut WellDefaults::new(0);

        move_block(&block, well, block_point, false);

        assert_eq!(block_point.col_ix, 6);
        assert_eq!(block_point.row_ix, 10);
    }

    #[rstest]
    #[case(WellPoint { row_ix: 0, col_ix: 5 }, false)] 
    #[case(WellPoint { row_ix: 20, col_ix: 4 }, true)] 
    #[case(WellPoint { row_ix: 20, col_ix: 5 }, true)] 
    #[case(WellPoint { row_ix: 0, col_ix: 0 }, false)] 
    #[case(WellPoint { row_ix: 0, col_ix: -1 }, true)] 
    #[case(WellPoint { row_ix: 0, col_ix: (WELL_COLUMN_COUNT - 4) as i8 }, false)] 
    #[case(WellPoint { row_ix: 0, col_ix: (WELL_COLUMN_COUNT - 3) as i8 }, false)] 
    #[case(WellPoint { row_ix: 0, col_ix: (WELL_COLUMN_COUNT - 2) as i8 }, true)] 
    #[case(WellPoint { row_ix: WELL_ROW_COUNT as i8, col_ix: 5 }, true)] 
    #[case(WellPoint { row_ix: (WELL_ROW_COUNT - 4) as i8, col_ix: 5 }, false)] 
    #[case(WellPoint { row_ix: (WELL_ROW_COUNT - 3) as i8, col_ix: 5 }, false)] 
    #[case(WellPoint { row_ix: (WELL_ROW_COUNT - 2) as i8, col_ix: 5 }, false)] 
    #[case(WellPoint { row_ix: (WELL_ROW_COUNT - 1) as i8, col_ix: 5 }, true)]
    #[case(WellPoint { row_ix: 19, col_ix: 4 }, true)] 
    fn is_move_blocked_for_case(#[case] new_point: WellPoint, #[case] expected: bool) {
        //shape:  BlockShape::new([[0,0],[0,1],[1,1],[1,2]])
        let block = Block::new(BlockType::Z); 
        let well: &mut Well = &mut WellDefaults::new(0);
        well[20][4] = 1;
        well[20][5] = 1;
        well[21][4] = 1;
        well[22][5] = 1;

        let result = is_move_blocked(&block, well, new_point);

        assert_eq!(result, expected);
    }    
}

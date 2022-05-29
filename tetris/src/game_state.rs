// keep track of the state of the game during play
use crate::{well::{Well, WellDefaults}, block_bag::{BlockBag, RandomBag}, block::Block};

#[derive(Debug)]
pub struct GameState {
    pub game_over: bool,

    pub well: Well,
    
    pub block_bag: BlockBag,
    pub current_block: Block,
    pub next_block: Block,

    pub block_row: u8,
    pub block_col: u8,

    //todo wouter: fill when relevant: keymap & fallcounter
}

impl GameState {
    pub fn new() -> Self {
        let mut block_bag : BlockBag = RandomBag::get();
        let current = block_bag.pop().unwrap();
        let next = block_bag.pop().unwrap();
        let well = WellDefaults::new(0);
        let starting_point = WellDefaults::get_start_position(&well);

        GameState { 
            game_over: false, 
            well,
            block_bag,
            current_block: current,        
            next_block: next,
            block_row: starting_point.row_ix,
            block_col: starting_point.col_ix
        }            
    }

    pub fn handle_game_over(&mut self, blink_counter:i32) -> i32{
        //we want a blink-effect of an empty & filled well (called on every-update, expected +-60 times per second)
        match blink_counter {
            15 => self.well = WellDefaults::new(0),                
            30 => 
            {
                self.well = WellDefaults::new(1);
                return 0;
            },
            _ => ()            
        }        
        blink_counter + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn game_state_new() {
        let new_state = GameState::new();

        assert!(!new_state.game_over);
        assert_eq!(new_state.well, [[0; 10]; 24]);
        assert_eq!(new_state.block_row, 2);
        assert_eq!(new_state.block_col, 3);
        assert_eq!(new_state.block_bag.len(), 5);
    }

    #[rstest]
    #[case(15, [[0; 10]; 24])]
    #[case(30, [[1; 10]; 24])]    
    #[case(8, [[2; 10]; 24])]    
    #[case(88, [[2; 10]; 24])]    
    fn game_state_game_over(#[case] blink_counter: i32, #[case] expected_well : Well ) {
        //2 is a faulty-well-state in reality, but makes it easy to test if well has been updated on game over
        let test_well = [[2; 10]; 24];

        let mut game_state = GameState::new();
        game_state.well = test_well;

        game_state.handle_game_over(blink_counter);

        assert_eq!(game_state.well, expected_well);
    }    
}

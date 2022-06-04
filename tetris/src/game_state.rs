// keep track of the state of the game during play
use crate::{well::{Well, WellDefaults, WellPoint}, block_bag::{BlockBag, RandomBag}, block::Block};

//how lower speed, how quicker. speed 1 is on each update
const DEFAULT_SPEED: u8 = 20;

#[derive(Debug)]
pub struct GameState {
    pub game_over: bool,

    pub well: Well,
    
    pub block_bag: BlockBag,
    pub current_block: Block,
    pub next_block: Block,

    pub current_block_point: WellPoint,

    pub executed_actions: Vec<Actions>,
    pub block_fall_counter: u8,
    pub current_speed: u8
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
            current_block_point: starting_point,  
            executed_actions: Vec::new(),
            block_fall_counter: 0,
            current_speed: DEFAULT_SPEED  
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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Actions {
    MoveLeft,
    MoveRight,
    RotateClockWise,
    //RotateCounterClockWise,
    Drop,
    DropHard
}

#[cfg(test)]
mod tests {
    use crate::well::{WELL_COLUMN_COUNT, WELL_ROW_COUNT, START_ROW, START_COL};

    use super::*;
    use rstest::rstest;

    #[test]
    fn game_state_new() {
        let new_state = GameState::new();

        assert!(!new_state.game_over);
        assert_eq!(new_state.well, [[0; WELL_COLUMN_COUNT]; WELL_ROW_COUNT]);
        assert_eq!(new_state.current_block_point, WellPoint { row_ix: START_ROW, col_ix: START_COL });
        assert_eq!(new_state.block_bag.len(), 19);
        assert_eq!(new_state.executed_actions, Vec::new());
        assert_eq!(new_state.block_fall_counter,0);
        assert_eq!(new_state.current_speed, DEFAULT_SPEED);
    }

    #[rstest]
    #[case(15, [[0; WELL_COLUMN_COUNT]; WELL_ROW_COUNT])]
    #[case(30, [[1; WELL_COLUMN_COUNT]; WELL_ROW_COUNT])]    
    #[case(8, [[2; WELL_COLUMN_COUNT]; WELL_ROW_COUNT])]    
    #[case(88, [[2; WELL_COLUMN_COUNT]; WELL_ROW_COUNT])]    
    fn game_state_game_over(#[case] blink_counter: i32, #[case] expected_well : Well ) {
        //2 is a faulty-well-state in reality, but makes it easy to test if well has been updated on game over
        let test_well = [[2; WELL_COLUMN_COUNT]; WELL_ROW_COUNT];

        let mut game_state = GameState::new();
        game_state.well = test_well;

        game_state.handle_game_over(blink_counter);

        assert_eq!(game_state.well, expected_well);
    }    
}

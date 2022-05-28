// keep track of the state of the game during play
use crate::{well::{Well, WellDefaults}, block_bag::{BlockBag, RandomBag}, block::Block};

pub struct GameState {
    game_over: bool,

    well: Well,
    
    block_bag: BlockBag,
    current_block: Block,
    next_block: Block,

    block_row: u8,
    block_col: u8,

    //todo wouter: fill when relevant: keymap & fallcounter
}

impl GameState {
    pub fn new() -> Self {
        let mut block_bag : BlockBag = RandomBag::get();
        let current = block_bag.pop().unwrap();
        let next = block_bag.pop().unwrap();
        let well = WellDefaults::new();
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
}
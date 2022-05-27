// keep track of the state of the game during play
use crate::{well::{Well, NewWell}, block_bag::{BlockBag, RandomBag}, block::Block};

pub struct GameState {
    game_over: bool,
    well: Well,
    block_bag: BlockBag,
    current_block: Block,
    next_block: Block

    //todo wouter: fill when relevant
}

impl GameState {
    pub fn new() -> Self {
        let mut block_bag : BlockBag = RandomBag::get();
        let current = block_bag.pop().unwrap();
        let next = block_bag.pop().unwrap();

        GameState { 
            game_over: false, 
            well: NewWell::new(),
            block_bag,
            current_block: current,        
            next_block: next,
        }            
    }
}
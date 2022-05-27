use rand::{thread_rng, prelude::SliceRandom};
use strum::IntoEnumIterator;

use crate::block::{Block, BlockType};

pub type BlockBag = Vec<Block>;

fn get_random_block_bag() -> BlockBag {
    let mut block_bag = BlockBag::new();

    for block_type in BlockType::iter() {
        block_bag.push(Block::new(block_type));
    }
    
    block_bag.shuffle(&mut thread_rng());
    //tbd: in example, multiple shuffles were executed as the result wasn't random enough after 1. for now: try with default

    block_bag
}

pub trait RandomBag {
    fn get() -> Self;
}

impl RandomBag for BlockBag {
    fn get() -> BlockBag {
        get_random_block_bag()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;
    
    #[test]
    fn block_bag_get_random_block_bag() {
        let block_bag = get_random_block_bag();

        //we expect 7 blocks in our bag and them to be different. we can't really test the randomizer as original order is also a random allowed order
        assert_eq!(block_bag.len(), 7);

        let grouped_by_type =  block_bag.iter()            
            .unique_by(|block| block.block_type)            ;
        assert_eq!(grouped_by_type.count(), 7);            
    }    
}

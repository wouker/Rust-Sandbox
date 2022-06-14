use rand::{thread_rng, prelude::SliceRandom};
use strum::IntoEnumIterator;

use crate::block::{Block, BlockType};

pub type BlockBag = Vec<Block>;

fn get_random_block_bag() -> BlockBag {
    let mut block_bag = BlockBag::new();
    
    //we shuffle 3 sets of blocks together for a bit variance.
    //this way, it should feel more random vs. fixed set of 7
    block_bag.append(&mut get_new_block_bag());
    block_bag.append(&mut get_new_block_bag());
    block_bag.append(&mut get_new_block_bag());

    block_bag.shuffle(&mut thread_rng());
    //tbd: in example, multiple shuffles were executed as the result wasn't random enough after 1. for now: try with default

    block_bag
}

fn get_new_block_bag() -> BlockBag {
    let mut block_bag = BlockBag::new();
       
    for block_type in BlockType::iter() {
        block_bag.push(Block::new(block_type));
    }

    block_bag
}

pub trait RandomBag {
    fn get() -> Self;
    fn refresh_if_needed(&mut self);
}

impl RandomBag for BlockBag {
    fn get() -> BlockBag {
        get_random_block_bag()
    }

    fn refresh_if_needed(&mut self) {
        if self.len() <= 3 {
            self.append(&mut get_random_block_bag());
        }
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
        assert_eq!(block_bag.len(), 21);

        let grouped_by_type =  block_bag.iter()            
            .unique_by(|block| block.block_type)            ;
        assert_eq!(grouped_by_type.count(), 7);            
    }    

    #[test]
    fn block_bag_refresh_if_needed() {
        let block_bag = &mut get_random_block_bag();

        RandomBag::refresh_if_needed(block_bag);

        assert_eq!(block_bag.len(), 21);

        for _ in 0..20 {
            block_bag.pop();
        }

        assert_eq!(block_bag.len(), 1);

        RandomBag::refresh_if_needed(block_bag);
        assert_eq!(block_bag.len(), 22);
    }
}

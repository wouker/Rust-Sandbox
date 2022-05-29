use strum_macros::EnumIter;

use crate::color::Color;

// each block is named after its shape.
// each block has 4 parts, which will be later translated to 4x4 matrix
// atm we define the 7 classic Tetrimo's
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, EnumIter)]
pub enum BlockType {
    // * * * *
    I,
    // *
    // * * *
    J,
    //     *
    // * * *
    L,
    // * *
    // * *
    O,
    //   * *
    // * *
    S,
    //   *
    // * * *
    T,
    // * *
    //   * *
    Z,
}

type BlockShapeType = [[u8; 4]; 4];
type Position = [u8;2];

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct BlockShape {
    shape: BlockShapeType,
}

impl BlockShape {
    fn new(positions : [Position;4]) -> Self {
        let mut shape: BlockShapeType = [[0; 4];4];
        
        for i in 0..4 {
            for j in 0..4 {
                
                for(_, position) in positions.iter().enumerate() {
                    if position[0] == i && position[1] == j {
                        shape[i as usize][j as usize] = 1;
                    }
                }
            }
        }

        BlockShape { shape }        
    }

    fn into_array(self) -> [[u8; 4]; 4] {
        self.shape
    }    
}

//todo wouter later: implement rotate R/L on BlockShape to alter shape on rotation

#[derive(Debug, Copy, Clone)]
pub struct Block {
    pub block_type: BlockType,
    pub color: Color,   
    pub shape: BlockShape,
}

//I is rotated in a 4x4 - all the rest in a 3x3
impl Block {
    pub fn new(block_type: BlockType) -> Self {
        match block_type {
            BlockType::I => Block {
                block_type: BlockType::I,
                color: Color::RED,
                shape: BlockShape::new([[0,2],[1,2],[2,2],[3,2]]),
            },           
            BlockType::J => Block {
                block_type: BlockType::J,
                color: Color::BLUE,
                shape: BlockShape::new([[0,0],[1,0],[1,1],[1,2]]),
            },
            BlockType::L => Block {
                block_type: BlockType::L,
                color: Color::GREEN,
                shape:  BlockShape::new([[0,2],[1,0],[1,1],[1,2]]),
            },
            BlockType::S => Block {
                block_type: BlockType::S,
                color: Color::ORANGE,
                shape:  BlockShape::new([[0,1],[0,2],[1,0],[1,1]]),
            },
            BlockType::Z => Block {
                block_type: BlockType::Z,
                color: Color::YELLOW,
                shape:  BlockShape::new([[0,0],[0,1],[1,1],[1,2]]),
            },
            BlockType::O => Block {
                block_type: BlockType::O,
                color: Color::AQUAMARINE,
                shape:  BlockShape::new([[0,1],[0,2],[1,1],[1,2]]),
            },
            BlockType::T => Block {
                block_type: BlockType::T,
                color: Color::PINK,
                shape:  BlockShape::new([[0,1],[1,0],[1,1],[1,2]]),
            }
        }          
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use conditional::conditional;

    #[test]
    fn blockshape_into_array() {
        let blockshape = BlockShape::new([[0,0],[0,1],[0,2],[0,3]]);
        assert_eq!([[1,1,1,1],[0,0,0,0],[0,0,0,0],[0,0,0,0]], blockshape.into_array());
    }    

    #[rstest]
    #[case(BlockType::I, Color::RED, [[0,2],[1,2],[2,2],[3,2]])]
    #[case(BlockType::J, Color::BLUE, [[0,0],[1,0],[1,1],[1,2]])]
    #[case(BlockType::L, Color::GREEN, [[0,2],[1,0],[1,1],[1,2]])]
    #[case(BlockType::S, Color::ORANGE, [[0,1],[0,2],[1,0],[1,1]])]
    #[case(BlockType::Z, Color::YELLOW, [[0,0],[0,1],[1,1],[1,2]])]
    #[case(BlockType::O, Color::AQUAMARINE, [[0,1],[0,2],[1,1],[1,2]])]
    #[case(BlockType::T, Color::PINK, [[0,1],[1,0],[1,1],[1,2]])]
    fn block_byblocktype_should_map(#[case] block_type : BlockType, #[case] color : Color, #[case] active_positions : [[usize;2];4]) { 
        let block = Block::new(block_type);
        assert_eq!(block.block_type, block_type);
        assert_eq!(block.color, color);

        for (i, row) in block.shape.into_array().iter().enumerate() {            
            for (j, value) in row.iter().enumerate() {                
                assert_eq!(*value, conditional!(active_positions.contains(&[i,j]) ? 1 : 0));
            }
        }  
    }
}

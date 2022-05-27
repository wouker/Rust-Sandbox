use strum_macros::EnumIter;

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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    const fn new(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b }
    }

    const fn white() -> Self {
        Color::new(255, 255, 255)
    }

    const fn black() -> Self {
        Color::new(0, 0, 0)
    }

    const fn red() -> Self {
        Color::new(255, 51, 51)
    }

    const fn blue() -> Self {
        Color::new(51, 51, 255)
    }

    const fn green() -> Self {
        Color::new(0, 153, 0)
    }

    const fn orange() -> Self {
        Color::new(255, 128, 0)
    }

    const fn yellow() -> Self {
        Color::new(255, 255, 0)
    }

    const fn purple() -> Self {
        Color::new(102, 0, 204)
    }

    const fn pink() -> Self {
        Color::new(255, 0, 255)
    }

    const fn aquamarine() -> Self {
        Color::new(128, 128, 128)
    }
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
                color: Color::red(),
                shape: BlockShape::new([[0,2],[1,2],[2,2],[3,2]]),
            },           
            BlockType::J => Block {
                block_type: BlockType::J,
                color: Color::blue(),
                shape: BlockShape::new([[0,0],[1,0],[1,1],[1,2]]),
            },
            BlockType::L => Block {
                block_type: BlockType::L,
                color: Color::green(),
                shape:  BlockShape::new([[0,2],[1,0],[1,1],[1,2]]),
            },
            BlockType::S => Block {
                block_type: BlockType::S,
                color: Color::orange(),
                shape:  BlockShape::new([[0,1],[0,2],[1,0],[1,1]]),
            },
            BlockType::Z => Block {
                block_type: BlockType::Z,
                color: Color::yellow(),
                shape:  BlockShape::new([[0,0],[0,1],[1,1],[1,2]]),
            },
            BlockType::O => Block {
                block_type: BlockType::O,
                color: Color::aquamarine(),
                shape:  BlockShape::new([[0,1],[0,2],[1,1],[1,2]]),
            },
            BlockType::T => Block {
                block_type: BlockType::T,
                color: Color::pink(),
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
    fn color_should_display_debug() {
        let color = Color::new(1, 2, 255);
        assert_eq!(format!("{color:?}"), "Color { r: 1, g: 2, b: 255 }");
    }

    #[rstest]
    #[case(Color::white(), 255, 255, 255)]
    #[case(Color::black(), 0, 0, 0)]
    #[case(Color::blue(), 51, 51, 255)]
    #[case(Color::red(), 255, 51, 51)]
    #[case(Color::green(), 0, 153, 0)]
    #[case(Color::yellow(), 255, 255, 0)]
    #[case(Color::orange(), 255, 128, 0)]
    #[case(Color::purple(), 102, 0, 204)]
    #[case(Color::pink(), 255, 0, 255)]
    #[case(Color::aquamarine(), 128, 128, 128)]
    fn color_predef_should_map(#[case] predef: Color, #[case] r: u8, #[case] g: u8, #[case] b: u8) {
        assert_eq!(predef, Color::new(r, g, b));
    }

    #[test]
    fn blockshape_into_array() {
        let blockshape = BlockShape::new([[0,0],[0,1],[0,2],[0,3]]);
        assert_eq!([[1,1,1,1],[0,0,0,0],[0,0,0,0],[0,0,0,0]], blockshape.into_array());
    }    

    #[rstest]
    #[case(BlockType::I, Color::red(), [[0,2],[1,2],[2,2],[3,2]])]
    #[case(BlockType::J, Color::blue(), [[0,0],[1,0],[1,1],[1,2]])]
    #[case(BlockType::L, Color::green(), [[0,2],[1,0],[1,1],[1,2]])]
    #[case(BlockType::S, Color::orange(), [[0,1],[0,2],[1,0],[1,1]])]
    #[case(BlockType::Z, Color::yellow(), [[0,0],[0,1],[1,1],[1,2]])]
    #[case(BlockType::O, Color::aquamarine(), [[0,1],[0,2],[1,1],[1,2]])]
    #[case(BlockType::T, Color::pink(), [[0,1],[1,0],[1,1],[1,2]])]
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

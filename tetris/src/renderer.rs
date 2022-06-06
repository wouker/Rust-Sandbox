use piston_window::{clear, rectangle, Event, EventLoop, PistonWindow, WindowSettings, types};

use crate::{
    block::Block,
    color::Color,
    game_state::GameState,
    well::{Well, WellPoint, WELL_COLUMN_COUNT, WELL_ROW_COUNT},
};

const PLAYFIELD_OFFSET_LEFT: f64 = 100.0;
const PLAYFIELD_OFFSET_TOP: f64 = 10.0;
const PLAYFIELD_WIDTH: f64 = (WELL_COLUMN_COUNT * BLOCK_SPACE_SIZE_PX) as f64;
const PLAYFIELD_HEIGHT: f64 = (WELL_ROW_COUNT * BLOCK_SPACE_SIZE_PX) as f64;
const PLAYFIELD_RECT: [f64; 4] = [
    PLAYFIELD_OFFSET_LEFT,
    PLAYFIELD_OFFSET_TOP,
    PLAYFIELD_WIDTH,
    PLAYFIELD_HEIGHT,
];
const BLOCK_SPACE_SIZE_PX: usize = 25;
const BLOCK_PART_SIZE_PX: usize = 23;

const WINDOW_WIDTH: f64 = 1280.0;
const WINDOW_HEIGHT: f64 = 720.0;

//todo wouter: learn more about Piston to fool around - for now: use defaults
pub fn get_window() -> PistonWindow {
    let mut window: PistonWindow = WindowSettings::new("Wouter Amazing Tetris", [WINDOW_WIDTH, WINDOW_HEIGHT])
        .exit_on_esc(true)
        .vsync(true)
        .build()
        .unwrap();

    //do some magic with rendering
    //we have vsync on and we lower Piston-events from 120 update-events per sec to 30 eps
    //this will also result in more rendering as events will be sent.
    window.events.set_ups(30);

    window
}

pub trait TetrisWindow {
    fn render(&mut self, event: &Event, game_state: &GameState);
}

impl TetrisWindow for PistonWindow {
    fn render(&mut self, event: &Event, game_state: &GameState) {
        //first we clear all before actual rendering. we do this by giving all pixels a background-color...
        self.draw_2d(event, |_context, graphics, _device| {
            let empty_color = Color::PURPLE.into();
            clear(empty_color, graphics);
        });
        
        //...draw playing field...
        self.draw_2d(event, |context, graphics, _device| {
            let rect_color = Color::BLACK.into();
            rectangle(rect_color, PLAYFIELD_RECT, context.transform, graphics);
        });
        
        //uncomment this if you want a gridview of the well - handy in debugging positions
        //draw_all_blocks(self, event); 

        //...(re)draw the well with content...
        draw_well(self, event, game_state.well);
        //...(re)draw the falling block (tetrimo that is in play)
        draw_falling_block(self, event, game_state);
        //..finally draw the next block that will be played
        //todo wouter: when we have something working, we want a right panel with nextblock & score as a real game
        draw_next_block(self, event, game_state.next_block);
    }
}

fn draw_well(window: &mut PistonWindow, event: &Event, well: Well) {
    for (row_ix, row) in well.iter().enumerate() {
        for (col_ix, value) in row.iter().enumerate() {
            if *value == 1u8 {
                //only draw when there is a part
                let point = WellPoint {
                    row_ix: row_ix as i8,
                    col_ix: col_ix as i8,
                };
                
                let color: [f32; 4] = Color::GRAY.into();
                let (x,y) = point.into();
                draw_part(window, event, x, y, 0, 0, color);
            }
        }
    }

    /*
    
/// Renders the squares of the given playfield.
fn draw_well_blocks(win: &mut PistonWindow, e: &Event, well: &Well)
{
    for row in 0..24 {
        for col in 0..10 {
            
            if well[row][col] == 0 { continue; }    // No square to be drawn here.

            let (x_offs, y_offs) = well_to_pixel(row as i32, col as i32);
            win.draw_2d(e,
                |context, graphics, _device| {
                    // Draw 33x33 square inside 35x35 space.
                    rectangle( [1.0, 1.0, 1.0, 1.0], [x_offs + 1.0, y_offs + 1.0, 33.0, 33.0], context.transform, graphics);
                }
            );
        }
    }
} */
}

#[allow(dead_code)]
fn draw_all_blocks(window: &mut PistonWindow, event: &Event) {
    for col_ix in 0..WELL_COLUMN_COUNT {
        for row_ix in 0..WELL_ROW_COUNT {
            let point = WellPoint {
                row_ix: row_ix as i8,
                col_ix: col_ix as i8,
            };
            
            let color: [f32; 4] = Color::WHITE.into();
            let (x,y) = point.into();
            draw_part(window, event, x, y, 0, 0, color);
        }
    }
}

fn draw_falling_block(window: &mut PistonWindow, event: &Event, game_state: &GameState) {
    //a block is a 4x4 matrix, with some parts filled.
    //we need to identify the current block it's filled positions (4 parts) and draw them
    //we have a location in our state (in the form of a wellpoint) that indicate where we start topleft of our block
    let (x, y) = game_state.current_block_point.into();
    let current_block = game_state.current_block;
    let color: [f32; 4] = current_block.color.into();

    for (row_ix, block_row) in current_block.shape.into_array().iter().enumerate() {
        for (col_ix, part) in block_row.iter().enumerate() {
            //only draw for parts that are 1
            if *part == 1u8 {
                //we need to go from 1-based to 0-based indexing
                draw_part(window, event, x, y, col_ix as usize, row_ix as usize, color);                
            }
        }
    }
}

fn draw_part(window: &mut PistonWindow, event: &Event, x_offset :f64, y_offset: f64, col_offset : usize, row_offset: usize, color: types::Color) {
    let x_offset = x_offset + (BLOCK_SPACE_SIZE_PX * col_offset) as f64;
    let y_offset = y_offset + (BLOCK_SPACE_SIZE_PX * row_offset) as f64;

    let part_rectangle = [
        x_offset,
        y_offset,
        BLOCK_PART_SIZE_PX as f64,
        BLOCK_PART_SIZE_PX as f64,
    ];

    window.draw_2d(event, |context, graphics, _device| {
        rectangle(color, part_rectangle, context.transform, graphics);
    });
}

fn draw_next_block(_window: &mut PistonWindow, _event: &Event, _next_block: Block) {
    //todo draw next block on fixed position (in seperate panel)
}

impl From<WellPoint> for (f64, f64) {
    fn from(well_point: WellPoint) -> Self {
        // The pixel value is the upper-left-most pixel of the square at the given well coordinate.
        let x = (well_point.col_ix as i32 * (BLOCK_SPACE_SIZE_PX as i32)) as f64 + PLAYFIELD_OFFSET_LEFT;
        let y = (well_point.row_ix as i32 * (BLOCK_SPACE_SIZE_PX as i32)) as f64 + PLAYFIELD_OFFSET_TOP;

        (x, y)
    }
}

//todo wouter: unittest to avoid overflow + the to i32 assignments in from-method feel dirty

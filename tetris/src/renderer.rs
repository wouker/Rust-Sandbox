use piston_window::{clear, rectangle, Event, EventLoop, PistonWindow, WindowSettings};

use crate::{color::Color, game_state::GameState, well::{Well, WellPoint}, block::Block};

const PLAYFIELD_OFFSET_LEFT : f64 = 100.0; 
const PLAYFIELD_OFFSET_TOP : f64 = 20.0; 
const PLAYFIELD_WIDTH : f64 = 354.0;
const PLAYFIELD_HEIGHT : f64 = 842.0;
const PLAYFIELD_RECT :[f64;4] = [PLAYFIELD_OFFSET_LEFT, PLAYFIELD_OFFSET_TOP, PLAYFIELD_WIDTH, PLAYFIELD_HEIGHT];
const BLOCK_SPACE_SIZE_PX : u8 = 35;
const BLOCK_PART_SIZE_PX : u8 = 33;

//todo wouter: learn more about Piston to fool around - for now: use defaults
pub fn get_window() -> PistonWindow {
    let mut window: PistonWindow = WindowSettings::new("Wouter Amazing Tetris", [1280, 720])
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
            rectangle(
                rect_color,
                PLAYFIELD_RECT,
                context.transform,
                graphics,
            );
        });

        //...(re)draw the well with content...
        draw_well(self, event, game_state.well);
        //...(re)draw the falling block (tetrimo that is in play)
        draw_falling_block(self, event, game_state);
        //..finally draw the next block that will be played
        //todo wouter: when we have something working, we want a right panel with nextblock & score as a real game 
        draw_next_block(self, event, game_state.next_block);
    }
}

fn draw_well(_window: &mut PistonWindow, _event: &Event, _well: Well) {
    //todo draw well once we have items in them
    println!("drawing well");
}

fn draw_falling_block(_window: &mut PistonWindow, _event: &Event, game_state: &GameState) {
    //a block is a 4x4 matrix, with some parts filled.
    //we need to identify the current block it's filled positions (4 parts) and draw them
    //we have a location in our state (in the form of a wellpoint) that indicate where we start topleft of our block
    let (x,y) = game_state.current_block_point.into();

    //todo: iterate the block & calc drawing-rectangles + draw
    /*

/// Renders the given Tetrimino at the given pixel coordinates.
fn draw_tetrimino_pixel(win: &mut PistonWindow, e: &Event, px: f64, py: f64, ttmo: &Tetrimino)
{
    // DEBUG ONLY: Draw transparent grey bounding box around tetrimino.
    // win.draw_2d(e, |context, graphics, _device| { rectangle([0.5; 4], [px, py, 140.0, 140.0], context.transform, graphics); } );

    for ttmo_row in 0..4 {
        for ttmo_col in 0..4 {
            
            if ttmo.shape[ttmo_row][ttmo_col] == 0 { continue; }    // No square to be drawn here.

            let x_offs = px + 35.0 * ttmo_col as f64;    // Each square in the Tetrimino is 35x35 pixels.
            let y_offs = py + 35.0 * ttmo_row as f64;    // Pixel Y coords increase downward.

            win.draw_2d(e,
                |context, graphics, _device| {
                    // Draw 33x33 square inside 35x35 space.
                    rectangle(ttmo.color, [x_offs + 1.0, y_offs + 1.0, 33.0, 33.0], context.transform, graphics);
                }
            );
        }
    }
} */

}

fn draw_next_block(_window: &mut PistonWindow, _event: &Event, _next_block : Block) {
    //todo draw next block on fixed position (in seperate panel)
    println!("drawing next block");
}

impl From<WellPoint> for (f64, f64) {
    fn from(well_point : WellPoint) -> Self {
        // The pixel value is the upper-left-most pixel of the square at the given well coordinate.
        let x = (well_point.col_ix as f64) * BLOCK_SPACE_SIZE_PX as f64 + PLAYFIELD_OFFSET_LEFT;
        let y= (well_point.row_ix as f64) * BLOCK_SPACE_SIZE_PX as f64 + PLAYFIELD_OFFSET_TOP;

        (x, y)
    }
}

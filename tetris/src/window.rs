use piston_window::{clear, rectangle, Event, EventLoop, PistonWindow, WindowSettings};

use crate::{color::Color, game_state::GameState, well::Well, block::Block};

 const PLAYFIELD_RECT :[f64;4] = [463.0, -140.0, 354.0, 842.0];

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
        
        //first we clear all before actual rendering. we do this by making all pixels gray...        
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
        draw_next_block(self, event, game_state.next_block);
        //todo wouter: when we have something working, we want a right panel with nextblock & score as a real game        
        /*

                    line of the playfield. 350 wide + 2 pixel gap on left and right => 354 pixels wide.
            win.draw_2d(re, |context, graphics, _device| { rectangle([0.0, 0.0, 0.0, 1.0], [463.0, -140.0, 354.0, 842.0], context.transform, graphics); } );

            draw_well_blocks(win, re, well);                      // Draw the contents of the playfield.
            draw_tetrimino_well(win, re, row, col, curr);         // Draw the currently falling tetrimino.
            draw_tetrimino_pixel(win, re, 320.0, 115.0, next);    // Draw the next tetrimino, always at the same place.
        } */
    }
}

fn draw_well(_window: &mut PistonWindow, _event: &Event, _well: Well) {
    //todo draw well once we have items in them
    println!("drawing well");
}

fn draw_falling_block(_window: &mut PistonWindow, _event: &Event, _game_state: &GameState) {
    //todo draw falling block
    println!("drawing falling block");
}

fn draw_next_block(_window: &mut PistonWindow, _event: &Event, _next_block : Block) {
    //todo draw next block on fixed position (in seperate panel)
    println!("drawing next block");
}

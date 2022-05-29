//todo wouter: remove later on when more is implemented
#![allow(dead_code)]

use game_state::GameState;
use music::get_music_handler;
use piston_window::{Event, Loop, Input, ButtonState};
use window::{get_window, TetrisWindow};

mod block;
mod color;
mod well;
mod game_state;
mod window;
mod music;
mod block_bag;

//todo wouter: remove later on when more is implemented
#[allow(unused_variables)]
#[allow(unused_mut)]
fn main() {
    
    let mut window = get_window();
    let (music_stream, rodio_sink) = get_music_handler();

    let mut game_state = GameState::new();
    let mut blink_counter = 0;

    //Piston is event-based engine. so we need to listen to events and act accordingly    
    //no for-loop as it consumes window and we would need borrowing (which requires lifetimes ...)
    while let Some(event) = window.next() {
        match event {

            //vsync = true => render-event per screen refresh (= 60 fps = 60 times)
            Event::Loop(Loop::Render(_)) => {
                TetrisWindow::render(&mut window, &event, &game_state);
            },

            //update event is set to 30 per second (from 120)
            Event::Loop(Loop::Update(_)) => {                
                if game_state.game_over {
                    blink_counter = game_state.handle_game_over(blink_counter);                    
                } else {
                    //handle play
                    game_update(&mut game_state);

                    //todo wouter handle music (and move to music-mod)
                    /* if game_state.game_over {
                       music_sink.stop();
                    } else {
                        if music_sink.empty() {
                           let music_file = File::open("NESTetrisMusic3.ogg").unwrap();    // Path relative to Cargo.toml
                           let music_source = rodio::Decoder::new(BufReader::new(music_file)).unwrap();
                           music_sink.append(music_source);
                           music_sink.play();
                       }
                   } */
                }

            },

            Event::Input(Input::Button(button_args), _time_stamp) => { 
                //key pressed only
                if button_args.state == ButtonState::Press {
                    store_key_pressed(&mut game_state, button_args);
                }
            },

            // ignore the other piston-events
            _ => { }
        }
    }
}

fn game_update (_game_state : &GameState) {
    //todo wouter: actual update while playing

    //todo wouter: once working: add a level-layer. each level each reached on number of points.
    //higher level is higher speed.
    //level is score multiplier
    //implement achievements (lvl 1- 5- 10 ...// 4 rows in 1 // ...)
}

fn store_key_pressed(game_state: &mut GameState, button_args: piston_window::ButtonArgs) {
    //todo wouter: handle key_pressed-event: we simply store listened keys to gamestate 
    //todo wouter: create enum to store mapped-key/action
    dbg!(game_state);
    dbg!(button_args);
}
//todo wouter: remove later on when more is implemented
#![allow(dead_code)]

use game_state::GameState;
use music::get_music_handler;
use piston_window::{Event, Loop};
use window::{get_window, TetrisWindow};

mod block;
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

    //Piston is event-based engine. so we need to listen to events and act accordingly    
    //no for-loop as it consumes window and we would need borrowing (which requires lifetimes ...)
    while let Some(event) = window.next() {
        match event {

            //vsync = true => render-event per screen refresh (= 60 fps = 60 times)
            Event::Loop(Loop::Render(_)) => {
                //render(&mut window, &event,
                //    &game_state.ttmo_row, &game_state.ttmo_col, &game_state.curr_ttmo,
                //    &game_state.next_ttmo, &mut game_state.well);
                //let starting_point = WellDefaults::get_start_position(&well);
                TetrisWindow::render(&window, &event, &mut game_state);
            },

            Event::Loop(Loop::Update(_)) => {                
                println!("updating.");
            },

            // Rust forces you to consider all possible Event types. This "discard all other events" clause satisfies that requirement. 
            // other are AfterRender & Idle
            _ => {
                //dbg!(event); //occurs a lot                  
            }
        }
    }
}
//todo wouter: remove later on when more is implemented
#![allow(dead_code)]

use game_state::GameState;
use music::get_music_handler;
use window::get_window;

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
    
    let window = get_window();
    let (music_stream, rodio_sink) = get_music_handler();

    let mut game_state = GameState::new();
}
//todo wouter: remove later on when more is implemented
//#![allow(dead_code)]

use game_state::{GameState, Actions};
use music::get_music_handler;
use piston_window::{Event, Loop, Input, ButtonState, Button, Key};
use renderer::{get_window, TetrisWindow};
use well::{WellPoint, WELL_COLUMN_COUNT};

mod block;
mod color;
mod well;
mod game_state;
mod renderer;
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

fn game_update (game_state : &mut GameState) {
    //todo wouter: actual update while playing

    //when we do nothing, a piece would fall
    //todo wouter : implement pauze-action

    //we don't want to fall every update-tick, but on the defined speed.
    //by using speed as a variable, we can dynamically speed up (eg. on a certain score in future)
    game_state.block_fall_counter += 1;
    if game_state.block_fall_counter >= game_state.current_speed {
        //todo uncomment to allow block to auto fall
        //game_state.current_block_point.row_ix += 1;

        //todo handle collisions
        /* if would_collide(&game_state.curr_ttmo, &game_state.well, &(game_state.ttmo_row + 1), &game_state.ttmo_col)
        {
            freeze_to_well(&game_state.curr_ttmo, &mut game_state.well, &game_state.ttmo_row, &game_state.ttmo_col);
            game_state.well = clear_complete_rows(game_state.well);

            if game_state.ttmo_bag.is_empty() { game_state.ttmo_bag = create_random_bag(); }
            game_state.curr_ttmo = game_state.next_ttmo;
            game_state.next_ttmo = game_state.ttmo_bag.pop().unwrap();

            game_state.ttmo_row = 2;    // Place near top...
            game_state.ttmo_col = 3;    // ...and near center.

            // THAT'S IT, MAN! GAME OVER, MAN!!
            if would_collide(&game_state.curr_ttmo, &game_state.well, &game_state.ttmo_row, &game_state.ttmo_col)
            {
                game_state.game_over = true;
            }
        }
          
        else { game_state.ttmo_row += 1; }    // Move curr piece down one row.
    } */
        //reset counter for next row-drop
        game_state.block_fall_counter = 0;
    }

    //check actions-queue
    for (_, action) in game_state.executed_actions.iter().enumerate() {
        match action {
            Actions::MoveLeft => move_left(&mut game_state.current_block_point),
            Actions::MoveRight => move_right(&mut game_state.current_block_point),
            //todo: handle drops & rotations
            _ => ()
        }
    }
    //reset actions
    game_state.executed_actions.clear();
        /*


// SoftDrop
if game_state.key_map[4] && !would_collide(&game_state.curr_ttmo, &game_state.well, &(game_state.ttmo_row + 1), &game_state.ttmo_col)
{ game_state.ttmo_row += 1; }

// HardDrop
if game_state.key_map[5]
{
for row in game_state.ttmo_row..24 {
    if would_collide(&game_state.curr_ttmo, &game_state.well, &row, &game_state.ttmo_col) {
        game_state.ttmo_row = row - 1;
        break;
    }
}
}

// RotateCCW
if game_state.key_map[2] {
rotate_tetrimino(&mut game_state.curr_ttmo, false);
if would_collide(&game_state.curr_ttmo, &game_state.well, &game_state.ttmo_row, &game_state.ttmo_col) {
    rotate_tetrimino(&mut game_state.curr_ttmo, true);
}
}

// RotateCW
if game_state.key_map[3] {
rotate_tetrimino(&mut game_state.curr_ttmo, true);
if would_collide(&game_state.curr_ttmo, &game_state.well, &game_state.ttmo_row, &game_state.ttmo_col) {
    rotate_tetrimino(&mut game_state.curr_ttmo, false);
}

*/


    //todo wouter: once working: add a level-layer. each level each reached on number of points.
    //higher level is higher speed.
    //level is score multiplier
    //implement achievements (lvl 1- 5- 10 ...// 4 rows in 1 // ...)

}

fn store_key_pressed(game_state: &mut GameState, button_args: piston_window::ButtonArgs) {
    let action: Option<Actions> = match button_args.button {
        Button::Keyboard(Key::Left) => Some(Actions::MoveLeft),
        Button::Keyboard(Key::Right) => Some(Actions::MoveRight),
        Button::Keyboard(Key::Up) => Some(Actions::RotateClockWise), //atm no ccw - todo wouter
        Button::Keyboard(Key::Down) => Some(Actions::Drop),
        Button::Keyboard(Key::Space) => Some(Actions::DropHard),
        _ => None
    };

    //use the gamestate-actions as a queue. will be executed in the update
    if let Some(action) = action {        
        game_state.executed_actions.push(action);
    }    
}

fn move_left(block_point: &mut WellPoint) {
    //todo: check collissions with other blocks
    // also it is not enough to just check if the block hits the wall, we need to check the most left-part
    if block_point.col_ix > 0 {
        block_point.col_ix -= 1;
    }
}

fn move_right(block_point: &mut WellPoint) {
    //todo: check collissions with other blocks
    // also it is not enough to just check if the block hits the wall, we need to check the most right-part
    if block_point.col_ix < WELL_COLUMN_COUNT as u8 {
        block_point.col_ix += 1;
    }
}

/*/// Returns true if the given Tetrimino, placed in the given playfield,
/// at the given row and col, would collide with something.
fn would_collide(ttmo: &Tetrimino, well: &Well, row: &i32, col: &i32) -> bool
{
    let mut well_row: i32;
    let mut well_col: i32;

    for ttmo_row in 0..4 {
        for ttmo_col in 0..4 {

            // Tetrimino has no square here, collison is not possible.
            if ttmo.shape[ttmo_row][ttmo_col] == 0 { continue; }

            // Compute well coords of ttmo square.
            well_row = ttmo_row as i32 + *row;
            well_col = ttmo_col as i32 + *col;

            // Collisions with well walls, floor.
            if well_col < 0 { return true; }
            if well_col > 9 { return true; }
            if well_row > 23 { return true; }
    
            // Collision with a block already frozen in the well.
            if well[well_row as usize][well_col as usize] != 0 { return true; }
        }
    }

    false
} */
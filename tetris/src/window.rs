use piston_window::{PistonWindow, WindowSettings, EventLoop};

//todo wouter: learn more about Piston to fool around - for now: use defaults 
pub fn get_window() -> PistonWindow {
    let mut window :PistonWindow = WindowSettings::new("Wouter Amazing Tetris", [1280,720])
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
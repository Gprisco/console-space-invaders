mod config;
mod input;

use input::{CrosstermEventPoller, GameInput, Input, TerminalInputHandler};
use std::time::Duration;

fn main() {
    let config = config::GameConfig::new();
    let input_handler = TerminalInputHandler::new(CrosstermEventPoller::new());

    // Simple input test loop
    loop {
        match input_handler.poll_input(Duration::from_millis(16)) {
            GameInput::Quit => break,
            input => println!("Received input: {:?}", input),
        }
    }
}

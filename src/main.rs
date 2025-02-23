mod config;
mod input;
mod renderer;

use crossterm::style::Color;
use input::{CrosstermEventPoller, Input, TerminalInputHandler};
use renderer::Renderer;
use std::time::Duration;

fn main() -> Result<(), ()> {
    let config = config::GameConfig::new();
    let input_handler = TerminalInputHandler::new(CrosstermEventPoller::new());
    let mut renderer = Renderer::new(config.window_width, config.window_height)?;

    // Test rendering loop
    loop {
        match input_handler.poll_input(Duration::from_millis(16)) {
            input::GameInput::Quit => break,
            input::GameInput::Left => renderer.draw_char(10, 10, '◄', Color::Blue)?,
            input::GameInput::Right => renderer.draw_char(10, 10, '►', Color::Green)?,
            input::GameInput::Fire => renderer.draw_char(10, 10, '▲', Color::Red)?,
            input::GameInput::None => {}
        }
    }

    Ok(())
}

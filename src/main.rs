mod config;
mod game;
mod input;
mod renderer;

use crossterm::style::Color;
use game::GameState;
use input::{CrosstermEventPoller, Input, TerminalInputHandler};
use renderer::Renderer;
use std::time::Duration;

fn main() -> Result<(), ()> {
    let config = config::GameConfig::new();
    let input_handler = TerminalInputHandler::new(CrosstermEventPoller::new());
    let mut renderer = Renderer::new(config.window_width, config.window_height)?;
    let mut game_state = GameState::new(config);

    loop {
        // Handle input
        let input = input_handler.poll_input(Duration::from_millis(16));
        if let input::GameInput::Quit = input {
            break;
        }

        // Update game state
        game_state.handle_input(input);
        game_state.update();

        // Render
        renderer.clear()?;

        // Draw player
        renderer.draw_char(
            game_state.player.x as u16,
            game_state.player.y as u16,
            game_state.player.symbol,
            Color::Green,
        )?;

        // Draw aliens
        for alien in &game_state.aliens {
            if alien.alive {
                renderer.draw_char(alien.x as u16, alien.y as u16, alien.symbol, Color::Red)?;
            }
        }

        // Draw bullets
        for bullet in &game_state.bullets {
            if bullet.active {
                renderer.draw_char(
                    bullet.x as u16,
                    bullet.y as u16,
                    bullet.symbol,
                    Color::Yellow,
                )?;
            }
        }

        // Draw score
        let score_text = format!("Score: {}", game_state.score);
        for (i, c) in score_text.chars().enumerate() {
            renderer.draw_char(i as u16, 0, c, Color::White)?;
        }

        renderer.present()?;
    }

    Ok(())
}

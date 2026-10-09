mod config;
mod game;
mod input;
mod renderer;

use crossterm::style::Color;
use game::GameState;
use input::{CrosstermEventPoller, GameInput, Input, TerminalInputHandler};
use renderer::Renderer;
use std::time::Duration;

fn main() -> std::io::Result<()> {
    let config = config::GameConfig::new();
    let frame_time = Duration::from_millis(1000 / u64::from(config.fps.max(1)));
    let input_handler = TerminalInputHandler::new(CrosstermEventPoller::new());
    let mut renderer = Renderer::new(config.window_width, config.window_height)?;
    let mut game_state = GameState::new(config);

    loop {
        let input = input_handler.poll_input(frame_time);
        if let GameInput::Quit = input {
            break;
        }

        game_state.handle_input(input);
        game_state.update();
        render(&mut renderer, &game_state)?;
    }

    Ok(())
}

fn render(renderer: &mut Renderer, game: &GameState) -> std::io::Result<()> {
    renderer.clear();
    renderer.draw_str(0, 0, &format!("Score: {}", game.score), Color::White);

    for alien in &game.aliens {
        if alien.alive {
            renderer.draw_char(alien.x as u16, alien.y as u16, alien.symbol, Color::Red);
        }
    }

    for bullet in &game.bullets {
        if bullet.active {
            renderer.draw_char(
                bullet.x as u16,
                bullet.y as u16,
                bullet.symbol,
                Color::Yellow,
            );
        }
    }

    renderer.draw_char(
        game.player.x as u16,
        game.player.y as u16,
        game.player.symbol,
        Color::Green,
    );

    if let Some(banner) = game.banner() {
        let x = (renderer.width().saturating_sub(banner.len() as u16)) / 2;
        let y = renderer.height() / 2;
        renderer.draw_str(x, y, banner, Color::Cyan);
    }

    renderer.present()
}

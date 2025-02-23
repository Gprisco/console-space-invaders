use crossterm::{
    cursor::{Hide, MoveTo, Show},
    execute,
    style::{Color, Print, SetForegroundColor},
    terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::stdout;

pub struct Renderer {
    width: u16,
    height: u16,
}

impl Renderer {
    pub fn new(width: u16, height: u16) -> Result<Self, ()> {
        let mut renderer = Self { width, height };
        renderer.init()?;
        Ok(renderer)
    }

    fn init(&mut self) -> Result<(), ()> {
        execute!(stdout(), EnterAlternateScreen, Hide, Clear(ClearType::All));
        Ok(())
    }

    pub fn draw_char(&mut self, x: u16, y: u16, ch: char, color: Color) -> Result<(), ()> {
        if x < self.width && y < self.height {
            execute!(stdout(), MoveTo(x, y), SetForegroundColor(color), Print(ch));
        }
        Ok(())
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        // Cleanup terminal when renderer is dropped
        execute!(stdout(), Show, LeaveAlternateScreen).unwrap_or(());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::style::Color;

    #[test]
    fn test_renderer_creation() {
        let renderer = Renderer::new(80, 24);
        assert!(renderer.is_ok());
    }

    #[test]
    fn test_draw_char_within_bounds() {
        let mut renderer = Renderer::new(80, 24).unwrap();
        let result = renderer.draw_char(5, 5, '■', Color::Green);
        assert!(result.is_ok());
    }

    #[test]
    fn test_draw_char_out_of_bounds() {
        let mut renderer = Renderer::new(80, 24).unwrap();
        let result = renderer.draw_char(81, 25, '■', Color::Green);
        assert!(result.is_ok()); // Should silently ignore out-of-bounds
    }
}

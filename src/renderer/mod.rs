mod frame;

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    execute, queue,
    style::{Color, Print, SetForegroundColor},
    terminal::{
        Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode,
        enable_raw_mode,
    },
};
use frame::{Cell, Frame, char_width};
use std::io::{self, Write, stdout};

pub struct Renderer {
    frame: Frame,
}

impl Renderer {
    pub fn new(width: u16, height: u16) -> io::Result<Self> {
        enable_raw_mode()?;
        execute!(stdout(), EnterAlternateScreen, Hide, Clear(ClearType::All))?;
        Ok(Self {
            frame: Frame::new(width, height),
        })
    }

    pub fn width(&self) -> u16 {
        self.frame.width()
    }

    pub fn height(&self) -> u16 {
        self.frame.height()
    }

    pub fn clear(&mut self) {
        self.frame.clear();
    }

    pub fn draw_char(&mut self, x: u16, y: u16, ch: char, color: Color) {
        self.frame.put(x, y, ch, color);
    }

    pub fn draw_str(&mut self, x: u16, y: u16, text: &str, color: Color) {
        let mut cursor = x;
        for ch in text.chars() {
            self.frame.put(cursor, y, ch, color);
            cursor = cursor.saturating_add(char_width(ch));
        }
    }

    pub fn present(&mut self) -> io::Result<()> {
        let mut buffer: Vec<u8> = Vec::new();
        let mut current_color: Option<Color> = None;
        let height = self.frame.height();

        for y in 0..height {
            queue!(buffer, MoveTo(0, y))?;

            let row_width = if y == height - 1 {
                self.frame.width().saturating_sub(1)
            } else {
                self.frame.width()
            };

            let mut x = 0;
            while x < row_width {
                match self.frame.cell(x, y) {
                    Cell::Continuation => {
                        x += 1;
                    }
                    Cell::Blank => {
                        queue!(buffer, Print(' '))?;
                        x += 1;
                    }
                    Cell::Glyph(ch, color) => {
                        if current_color != Some(color) {
                            queue!(buffer, SetForegroundColor(color))?;
                            current_color = Some(color);
                        }
                        queue!(buffer, Print(ch))?;
                        x += char_width(ch);
                    }
                }
            }
        }

        let mut out = stdout();
        out.write_all(&buffer)?;
        out.flush()?;
        Ok(())
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let _ = execute!(stdout(), Show, LeaveAlternateScreen);
    }
}

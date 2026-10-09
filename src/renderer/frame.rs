use crossterm::style::Color;
use unicode_width::UnicodeWidthChar;

#[derive(Clone, Copy, PartialEq, Debug)]
pub(super) enum Cell {
    Blank,
    Glyph(char, Color),
    Continuation,
}

pub struct Frame {
    width: u16,
    height: u16,
    cells: Vec<Cell>,
}

impl Frame {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            width,
            height,
            cells: vec![Cell::Blank; width as usize * height as usize],
        }
    }

    pub fn width(&self) -> u16 {
        self.width
    }

    pub fn height(&self) -> u16 {
        self.height
    }

    pub fn clear(&mut self) {
        self.cells.fill(Cell::Blank);
    }

    pub fn put(&mut self, x: u16, y: u16, ch: char, color: Color) {
        let width = char_width(ch);
        if width == 0 {
            return;
        }

        self.unanchor(x, y);

        if let Some(index) = self.index(x, y) {
            self.cells[index] = Cell::Glyph(ch, color);
        }

        if width == 2
            && let Some(index) = self.index(x + 1, y)
        {
            self.cells[index] = Cell::Continuation;
        }
    }

    pub(super) fn cell(&self, x: u16, y: u16) -> Cell {
        self.index(x, y)
            .map(|index| self.cells[index])
            .unwrap_or(Cell::Blank)
    }

    fn index(&self, x: u16, y: u16) -> Option<usize> {
        if x < self.width && y < self.height {
            Some(y as usize * self.width as usize + x as usize)
        } else {
            None
        }
    }

    fn unanchor(&mut self, x: u16, y: u16) {
        let Some(index) = self.index(x, y) else {
            return;
        };

        match self.cells[index] {
            Cell::Continuation => {
                self.cells[index] = Cell::Blank;
                if x > 0
                    && let Some(head) = self.index(x - 1, y)
                {
                    self.cells[head] = Cell::Blank;
                }
            }
            Cell::Glyph(ch, _) if char_width(ch) == 2 => {
                if let Some(tail) = self.index(x + 1, y) {
                    self.cells[tail] = Cell::Blank;
                }
            }
            _ => {}
        }
    }
}

pub(super) fn char_width(ch: char) -> u16 {
    UnicodeWidthChar::width(ch).unwrap_or(0) as u16
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn puts_glyph_in_cell() {
        let mut frame = Frame::new(10, 5);
        frame.put(3, 2, 'X', Color::Red);

        assert_eq!(frame.cell(3, 2), Cell::Glyph('X', Color::Red));
        assert_eq!(frame.cell(4, 2), Cell::Blank);
    }

    #[test]
    fn marks_continuation_for_wide_glyph() {
        let mut frame = Frame::new(10, 5);
        frame.put(2, 1, '👾', Color::Red);

        assert_eq!(frame.cell(2, 1), Cell::Glyph('👾', Color::Red));
        assert_eq!(frame.cell(3, 1), Cell::Continuation);
    }

    #[test]
    fn overwriting_head_clears_continuation() {
        let mut frame = Frame::new(10, 5);
        frame.put(2, 1, '👾', Color::Red);
        frame.put(2, 1, 'X', Color::Green);

        assert_eq!(frame.cell(2, 1), Cell::Glyph('X', Color::Green));
        assert_eq!(frame.cell(3, 1), Cell::Blank);
    }

    #[test]
    fn overwriting_continuation_clears_head() {
        let mut frame = Frame::new(10, 5);
        frame.put(2, 1, '👾', Color::Red);
        frame.put(3, 1, 'X', Color::Green);

        assert_eq!(frame.cell(2, 1), Cell::Blank);
        assert_eq!(frame.cell(3, 1), Cell::Glyph('X', Color::Green));
    }

    #[test]
    fn ignores_out_of_bounds() {
        let mut frame = Frame::new(10, 5);
        frame.put(20, 20, 'X', Color::Red);

        assert_eq!(frame.cell(0, 0), Cell::Blank);
    }

    #[test]
    fn clear_resets_all_cells() {
        let mut frame = Frame::new(10, 5);
        frame.put(1, 1, 'X', Color::Red);
        frame.clear();

        assert_eq!(frame.cell(1, 1), Cell::Blank);
    }
}

use std::convert::TryFrom;

use tui::{buffer::Buffer, layout::Rect, widgets::Widget};
use unicode_width::UnicodeWidthChar;

pub struct CharTable {
    offset: u32,
    pub max_char: Option<u32>,
}

impl CharTable {
    pub fn new(offset: u32) -> Self {
        Self {
            offset,
            max_char: None,
        }
    }
}

impl Widget for CharTable {
    fn draw(&mut self, area: Rect, buf: &mut Buffer) {
        (0..area.width).filter(|x| x % 2 == 0).for_each(|x| {
            (0..area.height).filter(|y| y % 2 == 0).for_each(|y| {
                let index = u32::from(x + y * area.width) + self.offset;
                let c = char::try_from(index);

                match c {
                    Ok(c) => {
                        let cell = buf.get_mut(area.x + x, area.y + y);

                        if let Some(width) = c.width() {
                            if width > 0 {
                                cell.set_char(c);
                            } else {
                                cell.symbol = [' ', c].iter().collect();
                            }
                        }
                    }
                    Err(_) => {
                        self.max_char = Some(index);
                    }
                }
            });
        });
    }
}

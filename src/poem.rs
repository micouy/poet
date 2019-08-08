use crate::{InputResult, Inputee};

use termion::event::Key;
use tui::{
    buffer::{Buffer, Cell},
    layout::Rect,
    style::Color,
    widgets::Widget,
};

pub struct Poem {
    buffer: Buffer,
    mode: Mode,
    cursor_x: u16,
    cursor_y: u16,
}

impl Poem {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            buffer: Buffer::filled(
                Rect::new(0, 0, width, height),
                Cell::default().set_symbol("A"),
            ),
            mode: Mode::Normal,
            cursor_x: 0,
            cursor_y: 0,
        }
    }
}

impl Widget for Poem {
    fn draw(&mut self, area: Rect, buf: &mut Buffer) {
        let width = area.width.min(self.buffer.area().width);
        let height = area.height.min(self.buffer.area().height);

        (0..width).for_each(|x| {
            (0..height).for_each(|y| {
                (*buf.get_mut(area.x + x, area.y + y)) = self.buffer.get(x, y).clone();
            });
        });

        let (cursor_fg, cursor_bg) = match self.mode {
            Mode::Normal => (Color::White, Color::Blue),
            Mode::Insert => (Color::Blue, Color::White),
        };

        buf.get_mut(area.x + self.cursor_x, area.y + self.cursor_y)
            .set_fg(cursor_fg)
            .set_bg(cursor_bg);
    }
}

enum Direction {
    Left,
    Down,
    Up,
    Right,
}

enum Action {
    Move(Direction),
    SwitchMode(Mode),
    PutChar(char),
    Ignore,
}

enum Mode {
    Normal,
    Insert,
}

impl Mode {
    fn translate_event_to_action(&self, key: Key) -> Action {
        use self::Mode::*;

        match self {
            Normal => Self::normal(key),
            Insert => Self::insert(key),
        }
    }

    fn normal(key: Key) -> Action {
        if let Key::Char(c) = key {
            match c {
                'h' | 'j' | 'k' | 'l' => {
                    use self::Direction::*;

                    let direction = match c {
                        'h' => Left,
                        'j' => Down,
                        'k' => Up,
                        'l' => Right,
                        _ => unreachable!(),
                    };

                    Action::Move(direction)
                }
                'i' => Action::SwitchMode(Mode::Insert),
                _ => Action::Ignore,
            }
        } else {
            Action::Ignore
        }
    }

    fn insert(key: Key) -> Action {
	match key {
    	    Key::Esc => Action::SwitchMode(Mode::Normal),
    	    Key::Char(c) => Action::PutChar(c),
    	    _ => Action::Ignore,
	}
    }
}

impl Inputee for Poem {
    fn handle(&mut self, key: Key) -> InputResult {
        use self::{Action::*, InputResult::*};

        let action = self.mode.translate_event_to_action(key);

        match action {
            Move(direction) => {
                self.move_cursor(direction);

         	Done
            }
            SwitchMode(mode) => {
                self.switch_mode(mode);

                Done
            }
            Ignore => InputResult::Ignored(key),
            PutChar(c) => {
                self.put_char(c);

                Done
            }
            _ => InputResult::Ignored(key),
        }
    }
}

impl Poem {
    fn move_cursor(&mut self, direction: Direction) {
        use self::Direction::*;

        match direction {
            Left => {
                self.cursor_x = self.cursor_x.saturating_sub(1);
            }
            Down => {
                self.cursor_y = self.cursor_y.saturating_add(1);
            }
            Up => {
                self.cursor_y = self.cursor_y.saturating_sub(1);
            }
            Right => {
                self.cursor_x = self.cursor_x.saturating_add(1);
            }
        }
    }

    fn put_char(&mut self, c: char) {
        self.buffer.get_mut(self.cursor_x, self.cursor_y).set_char(c);
    }

    fn switch_mode(&mut self, mode: Mode) {
        self.mode = mode;
    }
}

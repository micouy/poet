use std::{convert::TryFrom, io, thread::sleep_ms as sleep};

use termion::{
    async_stdin, event::Key, input::TermRead, raw::IntoRawMode, screen::AlternateScreen,
};
use tui::{
    backend::Backend,
    backend::TermionBackend,
    buffer::{Buffer, Cell},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Widget},
    Terminal,
};
use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

mod char_table;
mod events;
mod poem;

use self::char_table::CharTable;
use self::events::{Event, Events};
use self::poem::Poem;

enum InputResult {
    Done,
    Ignored(Key),
}

trait Inputee {
    fn handle(&mut self, key: Key) -> InputResult;
}

struct App<B>
where
    B: Backend,
{
    terminal: Terminal<B>,
    events: Events,
}

fn main() -> Result<(), io::Error> {
    let stdout = io::stdout().into_raw_mode()?;
    // let alternate_screen = AlternateScreen::from(stdout);
    // let backend = TermionBackend::new(alternate_screen);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor();
    terminal.clear()?;
    let mut offset = 0;
    let events = Events::new();
    let mut poem = Poem::new(10, 10);
    let mut e = 0;

    loop {
        terminal.draw(|mut f| {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .margin(0)
                .constraints(
                    [
                        Constraint::Percentage(40),
                        Constraint::Percentage(40),
                        Constraint::Min(0),
                    ]
                    .as_ref(),
                )
                .split(f.size());
            poem.render(&mut f, chunks[0]);
        })?;

        match events.next() {
            Event::Key(key) => {
                let _ = poem.handle(key);
            }
            Event::Quit => {
                break;
            }
        }

        e += 1;
    }

    terminal.clear();
    terminal.show_cursor();
    Ok(())
}

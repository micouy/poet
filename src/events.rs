use std::{
    io,
    sync::mpsc::{channel, Receiver},
    thread,
};

use termion::{event::Key, input::TermRead};

pub enum Event {
    Key(Key),
    Quit,
}

pub struct Events {
    rx: Receiver<Event>,
    input_handle: thread::JoinHandle<()>,
}

impl Events {
    pub fn new() -> Self {
        let (tx, rx) = channel();

        let input_handle = thread::spawn(move || {
            let stdin = io::stdin();
            let mut handle = stdin.lock();

            for evt in handle.keys() {
                match evt {
                    Ok(key) => {
                        if key == Key::Char('q') {
                            let _ = tx.send(Event::Quit);

                            return;
                        } else {
                            if tx.send(Event::Key(key)).is_err() {
                                return;
                            }
                        }
                    }
                    Err(_) => {}
                }
            }
        });

        Self { rx, input_handle }
    }

    pub fn next(&self) -> Event {
        self.rx.recv().unwrap()
    }
}

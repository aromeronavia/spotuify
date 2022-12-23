use std::{
    sync::mpsc::{Receiver, Sender},
    thread,
    time::Duration,
};

use crossterm::event;
use std::sync::mpsc;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Key {
    Enter,
    Up,
    Down,
    Exit,
    Tick,
    Unknown,
}

impl From<event::KeyEvent> for Key {
    fn from(key_event: event::KeyEvent) -> Self {
        match key_event {
            event::KeyEvent {
                code: event::KeyCode::Enter,
                ..
            } => Key::Enter,
            event::KeyEvent {
                code: event::KeyCode::Char('k'),
                ..
            } => Key::Up,
            event::KeyEvent {
                code: event::KeyCode::Char('j'),
                ..
            } => Key::Down,
            event::KeyEvent {
                code: event::KeyCode::Char('q'),
                ..
            } => Key::Exit,
            _ => Key::Unknown,
        }
    }
}

pub enum Event<I> {
    Input(I),
    Tick,
}

pub struct Events {
    rx: Receiver<Event<Key>>,
    _tx: Sender<Event<Key>>,
}

impl Events {
    pub fn new(tick_rate: Duration) -> Events {
        let (tx, rx) = mpsc::channel();

        let event_tx = tx.clone();

        thread::spawn(move || loop {
            if crossterm::event::poll(tick_rate).unwrap() {
                if let crossterm::event::Event::Key(key) = crossterm::event::read().unwrap() {
                    let key = Key::from(key);
                    event_tx.send(Event::Input(key)).unwrap();
                }
            }
        });
        Events { rx, _tx: tx }
    }

    pub fn next(&self) -> Result<Event<Key>, mpsc::RecvError> {
        self.rx.recv()
    }
}

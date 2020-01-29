use std::io::{stdin};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use termion::event::Key;
use termion::input::TermRead;

pub enum Event {
    Input(Key),
    Tick(Duration),
}

#[allow(dead_code)]
pub struct Events {
    rx: mpsc::Receiver<Event>,
    input_handle: thread::JoinHandle<()>,
    time_handle: thread::JoinHandle<()>,
}

impl Events {
    pub fn new() -> Events {
        let (tx, rx) = mpsc::channel();
        let input_handle = {
            let tx = tx.clone();
            thread::spawn(move || {
                let stdin = stdin();
                let locked = stdin.lock();
                for evt in locked.keys() {
                    match evt {
                        Ok(key) => {
                            if let Err(_) = tx.send(Event::Input(key)) {
                                return;
                            }
                        }
                        Err(_) => {panic!("aaaaargh")}
                    }
                }
            })
        };
        let time_handle = {
            let tx = tx.clone();
            thread::spawn(move || {
                let tx = tx.clone();
                loop {
                    let amount = Duration::from_millis(1000);
                    if let Err(_) = tx.send(Event::Tick(amount)) {
                        return;
                    }
                    thread::sleep(amount)
                }
            })
        };
        Events {
            rx,
            input_handle,
            time_handle,
        }
    }

    pub fn next(&self) -> Result<Event, mpsc::RecvError> {
        self.rx.recv()
    }
}

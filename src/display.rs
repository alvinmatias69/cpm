use std::sync::atomic::{AtomicBool, Ordering};
use std::{sync, thread, time};
use time::Duration;

use super::add::interface;

const FRAMES: [&'static str; 6] = ["⠟", "⠯", "⠷", "⠾", "⠽", "⠻"];

pub struct Display {
    handle: Option<thread::JoinHandle<()>>,
    alive: sync::Arc<AtomicBool>,
}

impl Display {
    pub fn new() -> Display {
        Display {
            handle: None,
            alive: sync::Arc::new(AtomicBool::new(false)),
        }
    }
}

impl interface::Display for Display {
    fn start_loading(&mut self, message: &str) {
        self.alive.store(true, Ordering::SeqCst);
        let message_string = String::from(message);
        let alive = self.alive.clone();

        self.handle = Some(thread::spawn(move || {
            let mut idx: usize = 0;
            while alive.load(Ordering::SeqCst) {
                print!("\r{} {}", FRAMES[idx % 6], message_string);
                idx = idx + 1;
                thread::sleep(time::Duration::from_millis(1 / 60));
            }
        }));
    }

    fn stop_loading(&mut self) {
        self.alive.store(false, Ordering::SeqCst);
        self.handle
            .take()
            .expect("Called stop on non-running thread")
            .join()
            .expect("Could not join spawned thread");
        println!("\r✓");
    }

    fn success(&self, message: String) {
        unimplemented!();
    }

    fn failure(&self, message: String) {
        unimplemented!();
    }
}

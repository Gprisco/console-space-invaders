use crossterm::event::{self, Event};
use std::time::Duration;

use super::EventPoller;

pub struct CrosstermEventPoller;

impl CrosstermEventPoller {
    pub fn new() -> Self {
        Self
    }
}

impl EventPoller for CrosstermEventPoller {
    fn poll(&self, timeout: Duration) -> bool {
        event::poll(timeout).unwrap_or(false)
    }

    fn read(&self) -> Result<Event, std::io::Error> {
        event::read()
    }
}

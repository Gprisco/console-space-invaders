mod event_poller;

use crossterm::event::{Event, KeyCode, KeyEvent};
use std::time::Duration;

pub use event_poller::CrosstermEventPoller;

pub trait EventPoller {
    fn poll(&self, timeout: Duration) -> bool;

    // TODO: a good improvement would be to decouple
    // this trait from crossterm::event::Event
    fn read(&self) -> Result<Event, std::io::Error>;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GameInput {
    Left,
    Right,
    Fire,
    Quit,
    None,
}

/// Trait defining the interface for input handling
pub trait Input {
    fn poll_input(&self, timeout: Duration) -> GameInput;
}

/// Input handler to use the event poller
pub struct TerminalInputHandler<T: EventPoller> {
    event_poller: T,
}

impl<T: EventPoller> TerminalInputHandler<T> {
    /// Creates a new [`TerminalInputHandler<T>`].
    pub fn new(event_poller: T) -> Self {
        Self { event_poller }
    }
}

impl<T: EventPoller> Input for TerminalInputHandler<T> {
    fn poll_input(&self, timeout: Duration) -> GameInput {
        if self.event_poller.poll(timeout) {
            if let Ok(Event::Key(KeyEvent { code, .. })) = self.event_poller.read() {
                return match code {
                    KeyCode::Left => GameInput::Left,
                    KeyCode::Right => GameInput::Right,
                    KeyCode::Char(' ') => GameInput::Fire,
                    KeyCode::Char('q') | KeyCode::Esc => GameInput::Quit,
                    _ => GameInput::None,
                };
            }
        }
        GameInput::None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockEventPoller {
        key_code: Option<KeyCode>,
    }

    impl EventPoller for MockEventPoller {
        fn poll(&self, _timeout: Duration) -> bool {
            self.key_code.is_some()
        }

        fn read(&self) -> Result<Event, std::io::Error> {
            Ok(Event::Key(KeyEvent::new(
                self.key_code.unwrap(),
                crossterm::event::KeyModifiers::empty(),
            )))
        }
    }

    #[test]
    fn test_input_with_mock_events() {
        // Given an event poller returning keycode left
        let event_poller = MockEventPoller {
            key_code: Some(KeyCode::Left),
        };
        let input_handler = TerminalInputHandler::new(event_poller);

        // Expect to poll GameInput::Left
        assert_eq!(
            input_handler.poll_input(Duration::from_millis(1)),
            GameInput::Left
        );
    }

    #[test]
    fn test_input_with_no_event() {
        // Given an event poller returning no keycode
        let event_poller = MockEventPoller { key_code: None };
        let input_handler = TerminalInputHandler::new(event_poller);

        // Expect to poll GameInput::None
        assert_eq!(
            input_handler.poll_input(Duration::from_millis(1)),
            GameInput::None
        );
    }
}

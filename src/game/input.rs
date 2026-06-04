use std::time::Duration;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};

use crate::game::direction::Direction;

pub enum InputAction{
    Move(Direction),
    Quit,
    None
}

pub fn read_input() -> std::io::Result<InputAction> {
    if event::poll(Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                return Ok(match key.code{
                    KeyCode::Char('w') => InputAction::Move(Direction::Up),
                    KeyCode::Char('s') => InputAction::Move(Direction::Down),
                    KeyCode::Char('a') => InputAction::Move(Direction::Left),
                    KeyCode::Char('d') => InputAction::Move(Direction::Right),
                    KeyCode::Esc => InputAction::Quit,
                    _ => InputAction::None,
                });
            }
        }
    }
    Ok(InputAction::None)
}
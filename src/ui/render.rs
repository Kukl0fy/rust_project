use crate::game::position::Position;
use crate::game::state::State;

use std::io::{stdout, Write};

use crossterm::{
    cursor::MoveTo,
    execute,
    terminal::{Clear, ClearType},
};

pub struct View;

impl View {
    pub fn new() -> Self {
        Self
    }

    pub fn render(&self, state: &State) -> std::io::Result<()> {
        let mut stdout = stdout();

        execute!(
            stdout,
            MoveTo(0, 0),
            Clear(ClearType::All)
        )?;

        for y in 0..state.map().height() {
            for x in 0..state.map().width() {
                write!(
                    stdout,
                    "{}",
                    state.char_at(Position {
                        x: x as i32,
                        y: y as i32,
                    })
                )?;
            }

            writeln!(stdout)?;
        }

        stdout.flush()?;

        Ok(())
    }
}
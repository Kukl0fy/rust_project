use crate::game::{direction::{self, Direction}, position::Position};

pub struct Player {
    pos: Position,
}

impl Player {
    pub fn new(pos: Position) -> Self {
        Self { pos }
    }

    pub fn pos(&self) -> Position {
        self.pos
    }

    pub fn set_pos(&mut self, pos: Position) {
        self.pos = pos;
    }

    pub fn move_player(&mut self, dir: Direction) {
        self.pos =self.pos + dir.to_pos() 
    }
}

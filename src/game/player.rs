use crate::game::{direction::{self, Direction}, position::Position};
use crate::game::combat_stats::CombatStats;
use crate::game::character_class::CharacterClass;

pub struct Player {
    pos: Position,
    class: CharacterClass,
    pub stats: CombatStats,
}

impl Player {
    pub fn new(pos: Position, class: CharacterClass) -> Self {
        let stats = class.default_stats();
        Self { pos, class, stats }
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

use crate::game::chest::ChestItem;
use crate::game::character_class::CharacterClass;
use crate::game::combat_stats::CombatStats;
use crate::game::{direction::Direction, position::Position};

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

    pub fn with_stats(pos: Position, class: CharacterClass, stats: CombatStats) -> Self {
        Self { pos, class, stats }
    }

    pub fn pos(&self) -> Position {
        self.pos
    }

    pub fn class(&self) -> CharacterClass {
        self.class
    }

    pub fn set_pos(&mut self, pos: Position) {
        self.pos = pos;
    }

    pub fn move_player(&mut self, dir: Direction) {
        self.pos = self.pos + dir.to_pos();
    }

    pub fn apply_item(&mut self, item: &ChestItem) {
        item.apply_to(&mut self.stats);
    }
}

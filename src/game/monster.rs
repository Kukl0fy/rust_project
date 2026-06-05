use crate::game::position::Position;
use crate::game::combat_stats::CombatStats;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MonsterType {
    Goblin,
    Spider,
    MindFlayer,
    Vampire,

}

impl MonsterType{
    pub fn default_stats(&self) -> CombatStats{
        match self{
            MonsterType::Goblin => CombatStats {
                hp: 30,
                max_hp: 30,
                attack: 5,
                defense: 2,
                sp_attack: 0,
                sp_defense: 0,
            },
            MonsterType::Spider => CombatStats {
                hp: 20,
                max_hp: 20,
                attack: 3,
                defense: 1,
                sp_attack: 0,
                sp_defense: 0,
            },
            MonsterType::MindFlayer => CombatStats {
                hp: 50,
                max_hp: 50,
                attack: 10,
                defense: 5,
                sp_attack: 15,
                sp_defense: 10,
            },
            MonsterType::Vampire => CombatStats {
                hp: 40,
                max_hp: 40,
                attack: 8,
                defense: 3,
                sp_attack: 12,
                sp_defense: 8,
            }
        }
    }
}
pub struct Monster {
    pos: Position,
    pub monster_type: MonsterType,
    pub stats: CombatStats,
}

impl Monster{
    pub fn new(pos: Position, monster_type: MonsterType) -> Self{
        Self { pos, monster_type, stats: monster_type.default_stats() }
    }

    pub fn pos(&self) -> Position {
        self.pos
    }
}

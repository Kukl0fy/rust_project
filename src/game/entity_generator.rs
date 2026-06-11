use crate::game::{
    monster::{Monster, MonsterType},
    position::Position,
};
use rand::Rng;
use rand::prelude::{IndexedRandom, SliceRandom};

const MONSTER_TYPES: [MonsterType; 4] = [
    MonsterType::Goblin,
    MonsterType::Spider,
    MonsterType::MindFlayer,
    MonsterType::Vampire,
];

#[derive(Clone)]
pub struct EntityGeneratorConfig {
    monster_count: usize,
}

impl EntityGeneratorConfig {
    pub fn new(monster_count: usize) -> Self {
        EntityGeneratorConfig { monster_count }
    }
}

pub struct Entities {
    monsters: Vec<Monster>,
}

impl Entities {
    pub fn monsters(&self) -> &[Monster] {
        &self.monsters
    }

    pub fn into_monsters(self) -> Vec<Monster> {
        self.monsters
    }
}

pub struct EntitiesGenerator {
    config: EntityGeneratorConfig,
}

impl EntitiesGenerator {
    pub fn new(config: EntityGeneratorConfig) -> Self {
        EntitiesGenerator { config }
    }

    pub fn generate_entities(
        &self,
        room_space: &[Position],
        player_start: Position,
    ) -> Entities {
        let mut rng = rand::rng();

        let mut valid_positions: Vec<Position> = room_space
            .iter()
            .copied()
            .filter(|pos| !Self::is_too_close_to_player(*pos, player_start))
            .collect();

        let count = self.config.monster_count.min(valid_positions.len());
        let (picked, _) = valid_positions.partial_shuffle(&mut rng, count);

        let monsters = picked
            .iter()
            .map(|&pos| Self::place_monster(pos, Self::choose_monster_type(&mut rng)))
            .collect();

        Entities { monsters }
    }

    fn is_too_close_to_player(pos: Position, player_start: Position) -> bool {
        let dx = (pos.x - player_start.x).abs();
        let dy = (pos.y - player_start.y).abs();
        dx.max(dy) <= 1
    }

    fn place_monster(position: Position, monster_type: MonsterType) -> Monster {
        Monster::new(position, monster_type)
    }

    fn choose_monster_type(rng: &mut impl Rng) -> MonsterType {
        *MONSTER_TYPES.choose(rng).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blocks_player_spawn_and_adjacent_tiles() {
        let player_start = Position { x: 5, y: 5 };

        assert!(EntitiesGenerator::is_too_close_to_player(
            Position { x: 5, y: 5 },
            player_start
        ));
        assert!(EntitiesGenerator::is_too_close_to_player(
            Position { x: 6, y: 5 },
            player_start
        ));
        assert!(EntitiesGenerator::is_too_close_to_player(
            Position { x: 4, y: 4 },
            player_start
        ));

        assert!(!EntitiesGenerator::is_too_close_to_player(
            Position { x: 7, y: 5 },
            player_start
        ));
        assert!(!EntitiesGenerator::is_too_close_to_player(
            Position { x: 5, y: 7 },
            player_start
        ));
    }
}

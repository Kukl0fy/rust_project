use crate::game::{
    chest::{random_loot, Chest, ChestItem},
    monster::{Monster, MonsterType},
    position::Position,
    room::Room,
};
use rand::Rng;
use rand::RngExt;
use rand::prelude::{IndexedRandom, SliceRandom};

const MONSTER_TYPES: [MonsterType; 4] = [
    MonsterType::Goblin,
    MonsterType::Spider,
    MonsterType::MindFlayer,
    MonsterType::Vampire,
];

#[derive(Clone)]
pub struct EntityGeneratorConfig {
    min_monsters_per_room: usize,
    max_monsters_per_room: usize,
    chest_spawn_chance: f64,
}

impl EntityGeneratorConfig {
    pub fn new(
        min_monsters_per_room: usize,
        max_monsters_per_room: usize,
        chest_spawn_chance: f64,
    ) -> Self {
        Self {
            min_monsters_per_room,
            max_monsters_per_room,
            chest_spawn_chance,
        }
    }
}

pub struct Entities {
    monsters: Vec<Monster>,
    chests: Vec<Chest>,
}

impl Entities {
    pub fn monsters(&self) -> &[Monster] {
        &self.monsters
    }

    pub fn chests(&self) -> &[Chest] {
        &self.chests
    }

    pub fn into_monsters(self) -> Vec<Monster> {
        self.monsters
    }

    pub fn into_parts(self) -> (Vec<Monster>, Vec<Chest>) {
        (self.monsters, self.chests)
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
        rooms: &[Room],
        start_room_index: usize,
        player_start: Position,
    ) -> Entities {
        let mut rng = rand::rng();
        let mut monsters = Vec::new();
        let mut chests = Vec::new();

        for room in rooms {
            let (room_monsters, room_chest) = self.populate_room(
                room,
                room.index == start_room_index,
                player_start,
                &mut rng,
            );
            monsters.extend(room_monsters);
            if let Some(chest) = room_chest {
                chests.push(chest);
            }
        }

        Entities { monsters, chests }
    }

    fn populate_room(
        &self,
        room: &Room,
        is_start_room: bool,
        player_start: Position,
        rng: &mut impl Rng,
    ) -> (Vec<Monster>, Option<Chest>) {
        if is_start_room {
            return (Vec::new(), None);
        }

        let mut positions: Vec<Position> = room
            .floor_positions()
            .into_iter()
            .filter(|pos| !Self::is_too_close_to_player(*pos, player_start))
            .collect();

        if positions.is_empty() {
            return (Vec::new(), None);
        }

        let max_monsters = self
            .config
            .max_monsters_per_room
            .max(1)
            .min(positions.len());
        let min_monsters = self.config.min_monsters_per_room.max(1).min(max_monsters);
        let monster_count = if min_monsters == max_monsters {
            min_monsters
        } else {
            rng.random_range(min_monsters..=max_monsters)
        };

        let (picked, remaining) = positions.partial_shuffle(rng, monster_count);
        let monsters = picked
            .iter()
            .map(|&pos| Self::place_monster(pos, Self::choose_monster_type(rng), room.index))
            .collect();

        let chest = if rng.random_bool(self.config.chest_spawn_chance) && !remaining.is_empty() {
            let chest_pos = remaining[rng.random_range(0..remaining.len())];
            Some(Self::place_chest(chest_pos, random_loot(rng), room.index))
        } else {
            None
        };

        (monsters, chest)
    }

    fn place_chest(position: Position, item: ChestItem, room_index: usize) -> Chest {
        Chest::new(position, item, room_index)
    }

    fn is_too_close_to_player(pos: Position, player_start: Position) -> bool {
        let dx = (pos.x - player_start.x).abs();
        let dy = (pos.y - player_start.y).abs();
        dx.max(dy) <= 1
    }

    fn place_monster(position: Position, monster_type: MonsterType, room_index: usize) -> Monster {
        Monster::new(position, monster_type, room_index)
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
        assert!(!EntitiesGenerator::is_too_close_to_player(
            Position { x: 7, y: 5 },
            player_start
        ));
    }

    #[test]
    fn non_start_rooms_get_at_least_one_monster() {
        let config = EntityGeneratorConfig::new(1, 2, 0.0);
        let generator = EntitiesGenerator::new(config);
        let rooms = vec![
            Room::new(0, 1, 1, 5, 5),
            Room::new(1, 10, 10, 5, 5),
            Room::new(2, 20, 20, 5, 5),
        ];

        let entities = generator.generate_entities(&rooms, 0, rooms[0].center());

        let monsters_in_start = entities
            .monsters()
            .iter()
            .filter(|m| m.room_index() == 0)
            .count();
        let monsters_in_room_1 = entities
            .monsters()
            .iter()
            .filter(|m| m.room_index() == 1)
            .count();
        let monsters_in_room_2 = entities
            .monsters()
            .iter()
            .filter(|m| m.room_index() == 2)
            .count();

        assert_eq!(monsters_in_start, 0);
        assert!(monsters_in_room_1 >= 1);
        assert!(monsters_in_room_2 >= 1);
    }
}

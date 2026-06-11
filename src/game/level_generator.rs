use crate::game::{
    entity_generator::{EntitiesGenerator, EntityGeneratorConfig},
    map::Map,
    map_generator::{MapGenerator, MapGeneratorConfig},
    monster::Monster,
    position::Position,
};

pub struct LevelGenerator {
    map_config: MapGeneratorConfig,
    entity_config: EntityGeneratorConfig,
}

pub struct Level {
    map: Map,
    monsters: Vec<Monster>,
    player_start: Position,
}

impl Level {
    pub fn map(&self) -> &Map {
        &self.map
    }

    pub fn monsters(&self) -> &[Monster] {
        &self.monsters
    }

    pub fn player_start(&self) -> Position {
        self.player_start
    }

    pub fn into_parts(self) -> (Map, Vec<Monster>, Position) {
        (self.map, self.monsters, self.player_start)
    }
}

impl LevelGenerator {
    pub fn new(
        width: usize,
        height: usize,
        room_min_width: usize,
        room_max_width: usize,
        room_min_height: usize,
        room_max_height: usize,
        max_rooms: usize,
        monster_count: usize,
    ) -> Self {
        LevelGenerator {
            map_config: MapGeneratorConfig::new(
                width,
                height,
                room_min_width,
                room_max_width,
                room_min_height,
                room_max_height,
                max_rooms,
            ),
            entity_config: EntityGeneratorConfig::new(monster_count),
        }
    }

    pub fn generate_level(&self) -> Level {
        let map_generator = MapGenerator::new(self.map_config.clone());
        let map_result = map_generator.generate_map();

        let entity_generator = EntitiesGenerator::new(self.entity_config.clone());
        let entities = entity_generator.generate_entities(
            &map_result.room_space,
            map_result.player_start,
        );

        Level {
            map: map_result.map,
            monsters: entities.into_monsters(),
            player_start: map_result.player_start,
        }
    }
}

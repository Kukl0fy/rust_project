use crate::game::{
    chest::Chest,
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
    chests: Vec<Chest>,
    player_start: Position,
}

impl Level {
    pub fn map(&self) -> &Map {
        &self.map
    }

    pub fn monsters(&self) -> &[Monster] {
        &self.monsters
    }

    pub fn chests(&self) -> &[Chest] {
        &self.chests
    }

    pub fn player_start(&self) -> Position {
        self.player_start
    }

    pub fn into_parts(self) -> (Map, Vec<Monster>, Vec<Chest>, Position) {
        (self.map, self.monsters, self.chests, self.player_start)
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
        min_monsters_per_room: usize,
        max_monsters_per_room: usize,
        chest_spawn_chance: f64,
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
            entity_config: EntityGeneratorConfig::new(
                min_monsters_per_room,
                max_monsters_per_room,
                chest_spawn_chance,
            ),
        }
    }

    pub fn generate_level(&self) -> Level {
        let map_generator = MapGenerator::new(self.map_config.clone());
        let map_result = map_generator.generate_map();

        let entity_generator = EntitiesGenerator::new(self.entity_config.clone());
        let entities = entity_generator.generate_entities(
            &map_result.rooms,
            map_result.start_room_index,
            map_result.player_start,
        );
        let (monsters, chests) = entities.into_parts();

        Level {
            map: map_result.map,
            monsters,
            chests,
            player_start: map_result.player_start,
        }
    }
}

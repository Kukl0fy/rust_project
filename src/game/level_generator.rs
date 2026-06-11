use crate::game::{entity_generator::EntityGeneratorConfig, map::Map, map_generator::{MapGeneratorConfig,MapGenerator}};

pub struct LevelGenerator{
    mapConfig: MapGeneratorConfig,
    entityConfig: EntityGeneratorConfig,
    
}

pub struct Level{
    map: Map

}

impl LevelGenerator{
    pub fn new(
        width: usize,
        height: usize,
        room_min_width: usize,
        room_max_width: usize,
        room_min_height: usize,
        room_max_height: usize,
        max_rooms: usize,
        monster_count: i32) -> Self{
        LevelGenerator{mapConfig: MapGeneratorConfig::new(width, height, room_min_width, room_max_width, room_min_height, room_max_height, max_rooms),entityConfig: EntityGeneratorConfig::new(monster_count)}
    }
    pub fn generateLevel(&self) -> Level {
        let mapGenerator = MapGenerator::new(self.mapConfig);
        let mapGeneratorResult = mapGenerator.generate_map();

        Level {
            map: mapGeneratorResult.map
        }
    }
}
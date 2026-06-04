use crate::game::map::{Map, Tile};
use rand::prelude::*;

struct MapGenerator{
    config: MapGeneratorConfig
}
struct MapGeneratorConfig{
    width: usize,
    height: usize,
    room_max_width: usize,
    room_min_width: usize,
    room_max_height: usize,
    room_min_height: usize,
    max_rooms: usize,

}

struct Room {
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

impl MapGenerator {
    pub fn new(config: MapGeneratorConfig) -> Self{
        MapGenerator{
            config
        }
    }

    pub fn generate_map(&self) -> Map{
        let width = self.config.width;
        let height = self.config.width;
        let tile = vec![vec![Tile::Wall; width]; height];
        let rooms: Vec<Room>;






        Map
    }

    fn generate_room(&self) -> Room{
        let mut rng = rand::rng();
        let width = rng.random_range(self.config.room_min_width ..= self.config.room_max_width);
        let height = rng.random_range(self.config.room_min_height ..= self.config.room_max_height);
        let x = rng.random_range(0..self.width - width);
        let y = rng.random_range(0..self.height - height);

        Room { x, y, width, height }
    }

    fn validate_room(){

    }
    
}
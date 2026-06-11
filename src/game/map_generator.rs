use crate::game::{map::{Map, Tile}, position::Position};
use rand::prelude::*;

pub struct MapGenerationResult {
    pub map: Map,
    pub player_start: Position,
    pub room_space: Vec<Position>
}

pub struct MapGenerator{
    config: MapGeneratorConfig
}
#[derive(Clone)]
pub struct MapGeneratorConfig{
    width: usize,
    height: usize,
    room_max_width: usize,
    room_min_width: usize,
    room_max_height: usize,
    room_min_height: usize,
    max_rooms: usize,

}

impl MapGeneratorConfig {
    pub fn new(
        width: usize,
        height: usize,
        room_min_width: usize,
        room_max_width: usize,
        room_min_height: usize,
        room_max_height: usize,
        max_rooms: usize,
    ) -> Self {
        Self {
            width,
            height,
            room_min_width,
            room_max_width,
            room_min_height,
            room_max_height,
            max_rooms,
        }
    }
}

struct Room {
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

impl Room {
    fn overlaps_with_margin(&self, other: &Room) -> bool {
        self.x < other.x + other.width + 1 &&
        self.x + self.width + 1 > other.x &&
        self.y < other.y + other.height + 1 &&
        self.y + self.height + 1 > other.y
    }

    fn carve_room(&self, tiles: &mut Vec<Vec<Tile>>) {
        for y in self.y..self.y + self.height {
            for x in self.x..self.x + self.width {
                tiles[y][x] = Tile::Floor;
            }
        }
    }

    fn center(&self) -> (usize, usize) {
        (
            self.x + self.width / 2,
            self.y + self.height / 2,
        )
    }

    fn distance_squared_to(&self, other: &Room) -> usize {
    let (x1, y1) = self.center();
    let (x2, y2) = other.center();

    let dx = x1.abs_diff(x2);
    let dy = y1.abs_diff(y2);

    dx * dx + dy * dy
}

}


impl MapGenerator {
    pub fn new(config: MapGeneratorConfig) -> Self{
        MapGenerator{
            config
        }
    }

    pub fn generate_map(&self) -> MapGenerationResult{

        let width = self.config.width;
        let height = self.config.height;
        let mut tiles = vec![vec![Tile::Wall; width]; height];
        let mut rooms: Vec<Room> = Vec::new();

        for _ in 0.. self.config.max_rooms{
            let room = self.generate_room();

            if self.validate_room(&room, &rooms){
                rooms.push(room);
            }
        }
        for r in &rooms{
            r.carve_room(&mut tiles);
        }
        let room_space = self.get_room_space(&tiles);

        self.connect_all_rooms(&mut tiles, &rooms);

        let player_start = rooms[0].center();
        MapGenerationResult {
            map: Map::new(tiles),
            player_start: Position {
            x: player_start.0 as i32,
            y: player_start.1 as i32,
    },
    room_space: room_space
}
    }

    fn generate_room(&self) -> Room{
        let mut rng = rand::rng();
        let width = rng.random_range(self.config.room_min_width ..= self.config.room_max_width - 1);
        let height = rng.random_range(self.config.room_min_height ..= self.config.room_max_height - 1);
        let x = rng.random_range(1..self.config.width - width - 1);
        let y = rng.random_range(1..self.config.height - height - 1);

        Room { x, y, width, height }
    }

    fn validate_room(&self,room: &Room, rooms: &[Room]) -> bool{
        for r in rooms{
            if room.overlaps_with_margin(r){
                return false;
            }
        }
        true
    }

    fn connect_rooms(&self, tiles: &mut Vec<Vec<Tile>>, room1: &Room, room2: &Room) {
    let (start_x, start_y) = room1.center();
    let (end_x, end_y) = room2.center();

    for x in start_x.min(end_x)..=start_x.max(end_x) {
        tiles[start_y][x] = Tile::Floor;
    }

    for y in start_y.min(end_y)..=start_y.max(end_y) {
        tiles[y][end_x] = Tile::Floor;
    }
}

fn connect_all_rooms(&self, tiles: &mut Vec<Vec<Tile>>, rooms: &[Room]) {
    if rooms.len() < 2 {
        return;
    }

    let mut connected: Vec<usize> = vec![0];
    let mut not_connected: Vec<usize> = (1..rooms.len()).collect();

    while !not_connected.is_empty() {
        let mut best_connected_index = 0;
        let mut best_not_connected_pos = 0;
        let mut best_distance = usize::MAX;

        for &connected_index in &connected {
            for (pos, &not_connected_index) in not_connected.iter().enumerate() {
                let distance = rooms[connected_index]
                    .distance_squared_to(&rooms[not_connected_index]);

                if distance < best_distance {
                    best_distance = distance;
                    best_connected_index = connected_index;
                    best_not_connected_pos = pos;
                }
            }
        }

        let new_room_index = not_connected.remove(best_not_connected_pos);

        self.connect_rooms(
            tiles,
            &rooms[best_connected_index],
            &rooms[new_room_index],
        );

        connected.push(new_room_index);
    }
}
    
fn get_room_space(&self,tiles:&Vec<Vec<Tile>>)->Vec<Position>{
    let mut room_space: Vec<Position> = Vec::new();
    for y in 0..tiles.len(){
        for x in 0..tiles[0].len(){
            if tiles[y][x] == Tile::Floor{
                room_space.push(Position{x: x as i32 ,y: y as i32});
            }
        }
    }
    room_space
}
}
use crate::game::{
    map::{Map, Tile},
    position::Position,
    room::Room,
};
use rand::prelude::*;

pub struct MapGenerationResult {
    pub map: Map,
    pub player_start: Position,
    pub rooms: Vec<Room>,
    pub start_room_index: usize,
}

pub struct MapGenerator {
    config: MapGeneratorConfig,
}

#[derive(Clone)]
pub struct MapGeneratorConfig {
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

struct RawRoom {
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

impl RawRoom {
    fn overlaps_with_margin(&self, other: &RawRoom) -> bool {
        self.x < other.x + other.width + 1
            && self.x + self.width + 1 > other.x
            && self.y < other.y + other.height + 1
            && self.y + self.height + 1 > other.y
    }

    fn carve_room(&self, tiles: &mut Vec<Vec<Tile>>) {
        for y in self.y..self.y + self.height {
            for x in self.x..self.x + self.width {
                tiles[y][x] = Tile::Floor;
            }
        }
    }

    fn center(&self) -> (usize, usize) {
        (self.x + self.width / 2, self.y + self.height / 2)
    }

    fn distance_squared_to(&self, other: &RawRoom) -> usize {
        let (x1, y1) = self.center();
        let (x2, y2) = other.center();
        let dx = x1.abs_diff(x2);
        let dy = y1.abs_diff(y2);
        dx * dx + dy * dy
    }

    fn to_room(&self, index: usize) -> Room {
        Room::new(index, self.x, self.y, self.width, self.height)
    }
}

impl MapGenerator {
    pub fn new(config: MapGeneratorConfig) -> Self {
        MapGenerator { config }
    }

    pub fn generate_map(&self) -> MapGenerationResult {
        let width = self.config.width;
        let height = self.config.height;
        let mut tiles = vec![vec![Tile::Wall; width]; height];
        let mut raw_rooms: Vec<RawRoom> = Vec::new();

        for _ in 0..self.config.max_rooms {
            let room = self.generate_room();
            if self.validate_room(&room, &raw_rooms) {
                raw_rooms.push(room);
            }
        }

        for room in &raw_rooms {
            room.carve_room(&mut tiles);
        }

        self.connect_all_rooms(&mut tiles, &raw_rooms);

        let rooms: Vec<Room> = raw_rooms
            .iter()
            .enumerate()
            .map(|(index, room)| room.to_room(index))
            .collect();

        let player_start = rooms
            .first()
            .map(|room| room.center())
            .unwrap_or(Position { x: 0, y: 0 });

        MapGenerationResult {
            map: Map::new(tiles),
            player_start,
            rooms,
            start_room_index: 0,
        }
    }

    fn generate_room(&self) -> RawRoom {
        let mut rng = rand::rng();
        let width = rng.random_range(self.config.room_min_width..=self.config.room_max_width - 1);
        let height = rng.random_range(self.config.room_min_height..=self.config.room_max_height - 1);
        let x = rng.random_range(1..self.config.width - width - 1);
        let y = rng.random_range(1..self.config.height - height - 1);

        RawRoom {
            x,
            y,
            width,
            height,
        }
    }

    fn validate_room(&self, room: &RawRoom, rooms: &[RawRoom]) -> bool {
        rooms
            .iter()
            .all(|other| !room.overlaps_with_margin(other))
    }

    fn connect_rooms(&self, tiles: &mut Vec<Vec<Tile>>, room1: &RawRoom, room2: &RawRoom) {
        let (start_x, start_y) = room1.center();
        let (end_x, end_y) = room2.center();

        for x in start_x.min(end_x)..=start_x.max(end_x) {
            tiles[start_y][x] = Tile::Floor;
        }

        for y in start_y.min(end_y)..=start_y.max(end_y) {
            tiles[y][end_x] = Tile::Floor;
        }
    }

    fn connect_all_rooms(&self, tiles: &mut Vec<Vec<Tile>>, rooms: &[RawRoom]) {
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
                    let distance = rooms[connected_index].distance_squared_to(&rooms[not_connected_index]);
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
}

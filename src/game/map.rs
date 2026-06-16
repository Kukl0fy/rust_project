use crate::game::position::Position;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Void,
    Wall,
    Floor,
    Exit,
    Ladder,
}

#[derive(Clone)]
pub struct Map {
    tiles: Vec<Vec<Tile>>,
    width: usize,
    height: usize,
}

impl Map {
    pub fn validate_pos(&self, pos: Position) -> bool {
        return pos.x >= 0 && pos.y >= 0 && pos.x < self.width as i32 && pos.y < self.height as i32;
    }

    pub fn tile_at(&self, pos: Position) -> Option<Tile> {
    if !self.validate_pos(pos) {
        return None;
    }

    Some(self.tiles[pos.y as usize][pos.x as usize])
    }

    pub fn is_walkable(&self, pos: Position) -> bool {
        if !self.validate_pos(pos) {
            return false;
        }
        match self.tile_at(pos) {
            Some(Tile::Floor) | Some(Tile::Exit) | Some(Tile::Ladder) => true,
            _ => false,
        }
    }

    pub fn is_floor_neighbor(&self, pos: Position, dx: i32, dy: i32) -> bool {
        let neighbor = Position {
            x: pos.x + dx,
            y: pos.y + dy,
        };
        matches!(
            self.tile_at(neighbor),
            Some(Tile::Floor) | Some(Tile::Exit) | Some(Tile::Ladder)
        )
    }

    pub fn new(tiles: Vec<Vec<Tile>>) -> Self {
        let height = tiles.len();
        let width = tiles[0].len();

        Self {
            tiles,
            width,
            height,
        }
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn set_tile(&mut self, pos: Position, tile: Tile) {
        if self.validate_pos(pos) {
            self.tiles[pos.y as usize][pos.x as usize] = tile;
        }
    }
}

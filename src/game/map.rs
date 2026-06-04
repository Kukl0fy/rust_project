use crate::game::position::Position;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Wall,
    Floor,
    Exit,
}

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
            Some(Tile::Wall) => false,
            _ => true,
        }
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
}

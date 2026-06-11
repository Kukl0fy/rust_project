use crate::game::position::Position;

#[derive(Clone, Debug)]
pub struct Room {
    pub index: usize,
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
}

impl Room {
    pub fn new(index: usize, x: usize, y: usize, width: usize, height: usize) -> Self {
        Self {
            index,
            x,
            y,
            width,
            height,
        }
    }

    pub fn overlaps_with_margin(&self, other: &Room) -> bool {
        self.x < other.x + other.width + 1
            && self.x + self.width + 1 > other.x
            && self.y < other.y + other.height + 1
            && self.y + self.height + 1 > other.y
    }

    pub fn contains(&self, pos: Position) -> bool {
        pos.x >= self.x as i32
            && pos.y >= self.y as i32
            && pos.x < (self.x + self.width) as i32
            && pos.y < (self.y + self.height) as i32
    }

    pub fn center(&self) -> Position {
        Position {
            x: (self.x + self.width / 2) as i32,
            y: (self.y + self.height / 2) as i32,
        }
    }

    pub fn floor_positions(&self) -> Vec<Position> {
        let mut positions = Vec::with_capacity(self.width * self.height);
        for y in self.y..self.y + self.height {
            for x in self.x..self.x + self.width {
                positions.push(Position {
                    x: x as i32,
                    y: y as i32,
                });
            }
        }
        positions
    }
}

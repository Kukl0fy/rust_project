use crate::game::position::Position;

pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}
impl Direction {
    pub fn to_pos(&self) -> Position {
        match self {
            Direction::Left => Position { x: -1, y: 0 },
            Direction::Right => Position { x: 1, y: 0 },
            Direction::Up => Position { x: 0, y: -1 },
            Direction::Down => Position { x: 0, y: 1 },
        }
    }
}

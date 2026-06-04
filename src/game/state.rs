use crate::game::direction::Direction;
use crate::game::map::Map;
use crate::game::map::Tile;
use crate::game::monster::Monster;
use crate::game::object::Object;
use crate::game::player::Player;
use crate::game::position::Position;
pub struct State {
    map: Map,
    player: Player,
    //entities: Vec<Monster>,
}

impl State {
    pub fn new(map: Map, player: Player) -> Self {
        Self { map, player }
    }

    pub fn object_at(&self, pos: Position) -> Option<Object> {
        if self.player.pos() == pos {
            Some(Object::Player)
        } else {
            None
        }
    }

    pub fn char_at(&self, pos: Position) -> char {
        match self.object_at(pos) {
            Some(Object::Player) => '@',
            _ => match self.map.tile_at(pos) {
                Some(Tile::Wall) => '#',
                Some(Tile::Floor) => '.',
                Some(Tile::Exit) => 'D',
                None => ' ',
            },
        }
    }

    pub fn validate_placing(&self, pos: Position) -> bool {
        self.map.is_walkable(pos)
    }

    pub fn map(&self) -> &Map {
        &self.map
    }

    pub fn move_player(&mut self, direction: Direction) {
        let new_pos = self.player.pos() + direction.to_pos();
        if self.validate_placing(new_pos){
            self.player.set_pos(new_pos);
        }
    }
}

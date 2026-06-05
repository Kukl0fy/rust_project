use crate::game::direction::Direction;
use crate::game::map::Map;
use crate::game::map::Tile;
use crate::game::monster::Monster;
use crate::game::object::Object;
use crate::game::player::Player;
use crate::game::position::Position;
use crate::game::combat::{self,CombatOutcome};

pub enum GameMode{
    Exploration,
    Combat{
        monster_index: usize,
        is_player_turn: bool,
    },
}

pub struct State {
    map: Map,
    player: Player,
    mode: GameMode,
    entities: Vec<Monster>,
}

impl State {
    pub fn new(map: Map, player: Player) -> Self {
        Self { map, player, mode: GameMode::Exploration, entities: Vec::new() }
    }

    pub fn add_monster(&mut self, monster:Monster){
        self.entities.push(monster);
    }

    pub fn object_at(&self, pos: Position) -> Option<Object> {
        if self.player.pos() == pos {
            return Some(Object::Player);
        }

        for _monster in &self.entities {
            if _monster.pos() == pos {
                return Some(Object::Monster);
            }
        }
        None
    }

    pub fn char_at(&self, pos: Position) -> char {
        match self.object_at(pos) {
            Some(Object::Player) => '@',
            Some(Object::Monster) => 'M',
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
        match self.mode{
            GameMode::Exploration => {
                let new_pos = self.player.pos() + direction.to_pos();

                let mut hit_monster_index = None;
                for (index,monster) in self.entities.iter().enumerate(){
                    if monster.pos() == new_pos{
                        hit_monster_index = Some(index);
                        break;
                    }
                }

                if let Some(idx)= hit_monster_index {
                    self.mode = GameMode::Combat{
                        monster_index: idx,
                        is_player_turn: true,
                    };
                } else if self.validate_placing(new_pos){
                    self.player.set_pos(new_pos);
                }
            },
            GameMode::Combat{monster_index, ..} => {
                let outcome =combat::process_turn(
                    &mut self.player,
                    &mut self.entities[monster_index],
                    direction,
                );

                match outcome{
                    combat::CombatOutcome::MonsterDefeated => {
                        self.entities.remove(monster_index);
                        self.mode = GameMode::Exploration;
                    }
                    combat::CombatOutcome::PlayerDefeated => {
                        //nwm w sumie co tu robimy chyba wtedy koniec gry i guess
                        //narazie ożywiam do testów
                        self.player.stats.hp = self.player.stats.max_hp;
                        self.mode = GameMode::Exploration;
                    }
                    combat::CombatOutcome::Fled => {
                        self.mode = GameMode::Exploration;
                        //zastanawiam sie czy opcja ucieczki wgl ma sens ale narazie dałam
                    }
                    combat::CombatOutcome::Ongoing => {}
                }
            },
        }
    }
}

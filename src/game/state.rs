use crate::game::combat::{self, CombatOutcome};
use crate::game::direction::Direction;
use crate::game::map::Map;
use crate::game::map::Tile;
use crate::game::monster::Monster;
use crate::game::object::Object;
use crate::game::player::Player;
use crate::game::position::Position;

pub enum GameMode {
    Exploration,
    Combat {
        monster_index: usize,
        is_player_turn: bool,
    },
}

pub struct State {
    map: Map,
    player: Player,
    mode: GameMode,
    entities: Vec<Monster>,
    status_message: String,
}

impl State {
    pub fn new(map: Map, player: Player, entities: Vec<Monster>) -> Self {
        Self {
            map,
            player,
            mode: GameMode::Exploration,
            entities,
            status_message: "Witaj w lochu. Szukaj potworow (M).".to_string(),
        }
    }

    pub fn add_monster(&mut self, monster: Monster) {
        self.entities.push(monster);
    }

    pub fn map(&self) -> &Map {
        &self.map
    }

    pub fn player(&self) -> &Player {
        &self.player
    }

    pub fn entities(&self) -> &[Monster] {
        &self.entities
    }

    pub fn mode(&self) -> &GameMode {
        &self.mode
    }

    pub fn status_message(&self) -> &str {
        &self.status_message
    }

    pub fn object_at(&self, pos: Position) -> Option<Object> {
        if self.player.pos() == pos {
            return Some(Object::Player);
        }

        for monster in &self.entities {
            if monster.pos() == pos {
                return Some(Object::Monster);
            }
        }
        None
    }

    pub fn char_at(&self, pos: Position) -> char {
        if let GameMode::Combat { monster_index, .. } = &self.mode {
            if self.entities[*monster_index].pos() == pos {
                return 'X';
            }
        }

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

    pub fn move_player(&mut self, direction: Direction) {
        match &self.mode {
            GameMode::Exploration => {
                let new_pos = self.player.pos() + direction.to_pos();

                let mut hit_monster_index = None;
                for (index, monster) in self.entities.iter().enumerate() {
                    if monster.pos() == new_pos {
                        hit_monster_index = Some(index);
                        break;
                    }
                }

                if let Some(idx) = hit_monster_index {
                    let monster_name = self.entities[idx].monster_type.name();
                    self.mode = GameMode::Combat {
                        monster_index: idx,
                        is_player_turn: true,
                    };
                    self.status_message = format!(
                        "Walka z {}! W=atak, S=atak spec., A=ucieczka.",
                        monster_name
                    );
                } else if self.validate_placing(new_pos) {
                    self.player.set_pos(new_pos);
                }
            }
            GameMode::Combat { monster_index, .. } => {
                let monster_index = *monster_index;
                let result = combat::process_turn(
                    &mut self.player,
                    &mut self.entities[monster_index],
                    direction,
                );

                self.status_message = result.message;

                match result.outcome {
                    CombatOutcome::MonsterDefeated => {
                        self.entities.remove(monster_index);
                        self.mode = GameMode::Exploration;
                    }
                    CombatOutcome::PlayerDefeated => {
                        self.player.stats.hp = self.player.stats.max_hp;
                        self.mode = GameMode::Exploration;
                        self.status_message =
                            "Zostales pokonany! Odzyskujesz sily (tryb testowy).".to_string();
                    }
                    CombatOutcome::Fled => {
                        self.mode = GameMode::Exploration;
                    }
                    CombatOutcome::Ongoing => {}
                }
            }
        }
    }
}

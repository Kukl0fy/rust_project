use crate::game::chest::Chest;
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
    chests: Vec<Chest>,
    status_message: String,
}

impl State {
    pub fn new(
        map: Map,
        player: Player,
        entities: Vec<Monster>,
        chests: Vec<Chest>,
    ) -> Self {
        Self {
            map,
            player,
            mode: GameMode::Exploration,
            entities,
            chests,
            status_message: "Witaj w lochu. M = potwor, C = skrzynka.".to_string(),
        }
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

    pub fn chests(&self) -> &[Chest] {
        &self.chests
    }

    pub fn mode(&self) -> &GameMode {
        &self.mode
    }

    pub fn status_message(&self) -> &str {
        &self.status_message
    }

    pub fn is_room_cleared(&self, room_index: usize) -> bool {
        !self
            .entities
            .iter()
            .any(|monster| monster.room_index() == room_index)
    }

    fn chest_index_at(&self, pos: Position) -> Option<usize> {
        self.chests
            .iter()
            .position(|chest| chest.pos() == pos)
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

        if self.chest_index_at(pos).is_some() {
            return Some(Object::Chest);
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
            Some(Object::Chest) => {
                let chest = &self.chests[self.chest_index_at(pos).unwrap()];
                if chest.is_open() {
                    '.'
                } else {
                    'C'
                }
            }
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

    fn try_open_chest(&mut self, chest_index: usize) {
        let room_index = self.chests[chest_index].room_index();

        if !self.is_room_cleared(room_index) {
            self.status_message =
                "Skrzynka jest zablokowana! Pokonaj wszystkie potwory w pokoju.".to_string();
            return;
        }

        if self.chests[chest_index].is_open() {
            self.status_message = "Skrzynka jest juz pusta.".to_string();
            return;
        }

        if let Some(item) = self.chests[chest_index].open() {
            let item_name = item.name.to_string();
            self.player.apply_item(&item);
            self.status_message = format!("Otworzyles skrzynke: {item_name}.");
        }
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
                    if let Some(chest_index) = self.chest_index_at(new_pos) {
                        self.try_open_chest(chest_index);
                    }
                }
            }
            GameMode::Combat { monster_index, .. } => {
                let monster_index = *monster_index;
                let defeated_room = self.entities[monster_index].room_index();
                let result = combat::process_turn(
                    &mut self.player,
                    &mut self.entities[monster_index],
                    direction,
                );

                self.status_message = result.message.clone();

                match result.outcome {
                    CombatOutcome::MonsterDefeated => {
                        self.entities.remove(monster_index);
                        self.mode = GameMode::Exploration;
                        if self.is_room_cleared(defeated_room) {
                            self.status_message = format!(
                                "{} Pokoj oczyszczony - mozesz otworzyc skrzynke.",
                                result.message
                            );
                        }
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

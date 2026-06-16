use crate::game::chest::{Chest, ItemEffect};
use crate::game::combat::{self, CombatOutcome};
use crate::game::direction::Direction;
use crate::game::map::Map;
use crate::game::map::Tile;
use crate::game::monster::Monster;
use crate::game::object::Object;
use crate::game::player::Player;
use crate::game::position::Position;
use rand::RngExt;

const MONSTER_HEAL_DROP_CHANCE: f64 = 0.35;
const MONSTER_HEAL_AMOUNT: i32 = 20;

pub enum GameMode {
    Exploration,
    Combat {
        monster_index: usize,
        is_player_turn: bool,
    },
}

use crate::game::level_generator::Level;

pub enum MoveResult {
    Normal,
    DescendLevel,
}

pub struct State {
    map: Map,
    player: Player,
    mode: GameMode,
    entities: Vec<Monster>,
    chests: Vec<Chest>,
    status_message: String,
    last_loot_name: Option<String>,
    last_loot_effect: Option<String>,
    level_depth: u32,
    ladder_pos: Position,
    exit_room_index: usize,
}

impl State {
    pub fn new(
        map: Map,
        player: Player,
        entities: Vec<Monster>,
        chests: Vec<Chest>,
        ladder_pos: Position,
        exit_room_index: usize,
    ) -> Self {
        Self {
            map,
            player,
            mode: GameMode::Exploration,
            entities,
            chests,
            status_message: "Witaj w lochu. Pokonaj wszystkie potwory i znajdz drabine.".to_string(),
            last_loot_name: None,
            last_loot_effect: None,
            level_depth: 1,
            ladder_pos,
            exit_room_index,
        }
    }

    pub fn level_depth(&self) -> u32 {
        self.level_depth
    }

    pub fn ladder_pos(&self) -> Position {
        self.ladder_pos
    }

    pub fn load_next_level(&mut self, level: Level) {
        let stats = self.player.stats.clone();
        let class = self.player.class();
        let (map, monsters, chests, start, ladder_pos, exit_room_index) = level.into_parts();

        self.map = map;
        self.entities = monsters;
        self.chests = chests;
        self.ladder_pos = ladder_pos;
        self.exit_room_index = exit_room_index;
        self.level_depth += 1;
        self.mode = GameMode::Exploration;
        self.player = Player::with_stats(start, class, stats);
        self.status_message = format!(
            "Zszedles na poziom {}. Pokonaj wszystkie potwory na mapie.",
            self.level_depth
        );
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

    pub fn last_loot(&self) -> Option<(&str, &str)> {
        match (&self.last_loot_name, &self.last_loot_effect) {
            (Some(name), Some(effect)) => Some((name.as_str(), effect.as_str())),
            _ => None,
        }
    }

    pub fn is_in_combat(&self) -> bool {
        matches!(self.mode, GameMode::Combat { .. })
    }

    pub fn combat_monster(&self) -> Option<&Monster> {
        match &self.mode {
            GameMode::Combat { monster_index, .. } => self.entities.get(*monster_index),
            _ => None,
        }
    }

    pub fn is_room_cleared(&self, room_index: usize) -> bool {
        !self
            .entities
            .iter()
            .any(|monster| monster.room_index() == room_index)
    }

    pub fn is_level_cleared(&self) -> bool {
        self.entities.is_empty()
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
                Some(Tile::Ladder) => '>',
                Some(Tile::Void) | None => ' ',
            },
        }
    }

    pub fn validate_placing(&self, pos: Position) -> bool {
        self.map.is_walkable(pos)
    }

    fn try_monster_heal_drop(&mut self) -> Option<String> {
        let mut rng = rand::rng();
        if !rng.random_bool(MONSTER_HEAL_DROP_CHANCE) {
            return None;
        }

        ItemEffect::Heal(MONSTER_HEAL_AMOUNT).apply(&mut self.player.stats);
        self.last_loot_name = Some("Mikstura z potwora".to_string());
        self.last_loot_effect = Some(format!("+{MONSTER_HEAL_AMOUNT} HP"));
        Some(format!(
            "Drop: +{MONSTER_HEAL_AMOUNT} HP (teraz {}/{})",
            self.player.stats.hp,
            self.player.stats.max_hp
        ))
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
            let effect_text = item.effect.detail();
            self.last_loot_name = Some(item_name.clone());
            self.last_loot_effect = Some(effect_text);
            self.status_message = item.effect.result_message(&item_name, &self.player.stats);
        }
    }

    pub fn move_player(&mut self, direction: Direction) -> MoveResult {
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
                    return self.check_ladder_descent();
                }
                MoveResult::Normal
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

                        let mut msg = result.message.clone();
                        if let Some(heal) = self.try_monster_heal_drop() {
                            msg = format!("{msg} {heal}");
                        }

                        if self.is_room_cleared(defeated_room) {
                            msg = format!("{msg} Pokoj oczyszczony - mozesz otworzyc skrzynke.");
                        }
                        if self.is_level_cleared() {
                            msg = format!("{msg} Wszystkie potwory pokonane - idz na drabine!");
                        }
                        self.status_message = msg;
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
                MoveResult::Normal
            }
        }
    }

    fn check_ladder_descent(&mut self) -> MoveResult {
        if self.player.pos() != self.ladder_pos {
            return MoveResult::Normal;
        }

        if !self.is_level_cleared() {
            self.status_message =
                "Pokonaj wszystkie potwory na mapie, zanim zejdziesz drabina!".to_string();
            return MoveResult::Normal;
        }

        MoveResult::DescendLevel
    }
}

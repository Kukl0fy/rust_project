use crate::game::monster::MonsterType;

use super::sprite_map::TileCoord;

pub const CHARACTER_TILE_PX: f32 = 16.0;

pub fn player_sprite(class: crate::game::character_class::CharacterClass) -> TileCoord {
    match class {
        crate::game::character_class::CharacterClass::Warrior => TileCoord::new(0, 0),
        crate::game::character_class::CharacterClass::Mage => TileCoord::new(2, 0),
        crate::game::character_class::CharacterClass::Tank => TileCoord::new(4, 0),
    }
}

pub fn monster_sprite(monster_type: MonsterType) -> TileCoord {
    match monster_type {
        MonsterType::Goblin => TileCoord::new(2, 1),
        MonsterType::Spider => TileCoord::new(0, 1),
        MonsterType::MindFlayer => TileCoord::new(1, 1),
        MonsterType::Vampire => TileCoord::new(4, 1),
    }
}

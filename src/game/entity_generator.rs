use crate::game::{monster::{self, Monster}, position::Position};
use rand::seq::SliceRandom;
pub struct EntityGeneratorConfig{
    monster_count: usize,

}
impl EntityGeneratorConfig {
   pub fn new(monster_count: i32) -> Self {
        EntityGeneratorConfig { monster_count: monster_count as usize }
    }
}

pub struct Entities{
    monsters: Monster
}

pub struct EntitiesGenerator{
    config: EntityGeneratorConfig
}

impl EntitiesGenerator{
    pub fn generate_entities(&self, room_space:&Vec<Position>) -> Entities{
        let mut occupied_pos = Vec::new();
        let mut avilable_pos = room_space.clone();
        let mut rng = rand::rng();
        let (picked, _avilable_pos) = avilable_pos.partial_shuffle(&mut rng, self.config.monster_count);
        occupied_pos.extend(picked);

        Entities {
            monsters: Monster::new(Position { x: 0, y: 0 }, crate::game::monster::MonsterType::Goblin)
        }
    }
    
    fn place_monster(&self, _position: Position) {
        
    }

    fn choose_monster_type() {
        
    }
}

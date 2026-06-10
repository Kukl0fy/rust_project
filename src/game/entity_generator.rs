use crate::game::{monster::{self, Monster}, position::Position};
use rand::seq::SliceRandom;
pub struct EntityGeneratorConfig{
    monster_count: usize,

}
impl EntityGeneratorConfig {
   pub fn new(monster_count: i32) -> Self{
        EntityGeneratorConfig { monster_count }
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
        let mut avilable_pos = room_space;
        let mut rng = rand::rng();
        let (picked,avilable_pos) = avilable_pos.partial_shuffle(&mut rng, self.config.monster_count);
        occupied_pos.extend(picked);


    }
    fn place_monster(&self,position: Position){
        
    }

    fn choose_monster_type(){
        
    }
}

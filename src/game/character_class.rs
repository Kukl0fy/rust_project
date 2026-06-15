use crate::game::combat_stats::CombatStats;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CharacterClass {
    Warrior,
    Mage,
    Tank,
}

impl CharacterClass{
    pub fn default_stats(&self) -> CombatStats{
        match self{
            CharacterClass::Warrior => CombatStats {
                hp: 100,
                max_hp: 100,
                attack: 15,
                defense: 10,
                sp_attack: 18,
                sp_defense: 8,
            },
            CharacterClass::Mage => CombatStats {
                hp: 60,
                max_hp: 60,
                attack: 5,
                defense: 5,
                sp_attack: 22,
                sp_defense: 10,
            },
            CharacterClass::Tank => CombatStats {
                hp: 150,
                max_hp: 150,
                attack: 10,
                defense: 20,
                sp_attack: 12,
                sp_defense: 14,
            },
        }
    }
}
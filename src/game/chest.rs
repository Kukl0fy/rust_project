use crate::game::combat_stats::CombatStats;
use crate::game::position::Position;
use rand::prelude::IndexedRandom;
use rand::Rng;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ItemEffect {
    Heal(i32),
    BoostMaxHp(i32),
    BoostAttack(i32),
    BoostDefense(i32),
    BoostSpAttack(i32),
    BoostSpDefense(i32),
}

impl ItemEffect {
    pub fn apply(&self, stats: &mut CombatStats) {
        match self {
            ItemEffect::Heal(amount) => {
                stats.hp = (stats.hp + amount).min(stats.max_hp);
            }
            ItemEffect::BoostMaxHp(amount) => {
                stats.max_hp += amount;
                stats.hp += amount;
            }
            ItemEffect::BoostAttack(amount) => stats.attack += amount,
            ItemEffect::BoostDefense(amount) => stats.defense += amount,
            ItemEffect::BoostSpAttack(amount) => stats.sp_attack += amount,
            ItemEffect::BoostSpDefense(amount) => stats.sp_defense += amount,
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            ItemEffect::Heal(amount) => {
                if *amount >= 50 {
                    "duze leczenie"
                } else {
                    "leczenie"
                }
            }
            ItemEffect::BoostMaxHp(_) => "wiecej maks. HP",
            ItemEffect::BoostAttack(_) => "wiecej ataku",
            ItemEffect::BoostDefense(_) => "wiecej obrony",
            ItemEffect::BoostSpAttack(_) => "wiecej ataku spec.",
            ItemEffect::BoostSpDefense(_) => "wiecej obrony spec.",
        }
    }

    pub fn detail(&self) -> String {
        match self {
            ItemEffect::Heal(amount) => format!("+{amount} HP"),
            ItemEffect::BoostMaxHp(amount) => format!("+{amount} maks. HP"),
            ItemEffect::BoostAttack(amount) => format!("+{amount} ATK"),
            ItemEffect::BoostDefense(amount) => format!("+{amount} DEF"),
            ItemEffect::BoostSpAttack(amount) => format!("+{amount} SP ATK"),
            ItemEffect::BoostSpDefense(amount) => format!("+{amount} SP DEF"),
        }
    }

    pub fn result_message(&self, item_name: &str, stats: &CombatStats) -> String {
        match self {
            ItemEffect::Heal(_) => {
                format!("{item_name}: {} (HP: {}/{})", self.detail(), stats.hp, stats.max_hp)
            }
            ItemEffect::BoostMaxHp(_) => {
                format!(
                    "{item_name}: {} (HP: {}/{})",
                    self.detail(),
                    stats.hp,
                    stats.max_hp
                )
            }
            ItemEffect::BoostAttack(_) => {
                format!("{item_name}: {} (ATK: {})", self.detail(), stats.attack)
            }
            ItemEffect::BoostDefense(_) => {
                format!("{item_name}: {} (DEF: {})", self.detail(), stats.defense)
            }
            ItemEffect::BoostSpAttack(_) => {
                format!(
                    "{item_name}: {} (SP ATK: {})",
                    self.detail(),
                    stats.sp_attack
                )
            }
            ItemEffect::BoostSpDefense(_) => {
                format!(
                    "{item_name}: {} (SP DEF: {})",
                    self.detail(),
                    stats.sp_defense
                )
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ChestItem {
    pub name: &'static str,
    pub effect: ItemEffect,
}

impl ChestItem {
    pub const fn new(name: &'static str, effect: ItemEffect) -> Self {
        Self { name, effect }
    }

    pub fn apply_to(&self, stats: &mut CombatStats) {
        self.effect.apply(stats);
    }
}

pub const LOOT_POOL: [ChestItem; 7] = [
    ChestItem::new("Mikstura leczenia", ItemEffect::Heal(25)),
    ChestItem::new("Duza mikstura", ItemEffect::Heal(50)),
    ChestItem::new("Amulet witalnosci", ItemEffect::BoostMaxHp(15)),
    ChestItem::new("Ostrze wojownika", ItemEffect::BoostAttack(4)),
    ChestItem::new("Tarcza lochu", ItemEffect::BoostDefense(3)),
    ChestItem::new("Ksiega mocy", ItemEffect::BoostSpAttack(5)),
    ChestItem::new("Szata maga", ItemEffect::BoostSpDefense(4)),
];

pub fn random_loot(rng: &mut impl Rng) -> ChestItem {
    LOOT_POOL.choose(rng).unwrap().clone()
}

#[derive(Clone, Debug)]
pub struct Chest {
    pos: Position,
    item: ChestItem,
    opened: bool,
    room_index: usize,
}

impl Chest {
    pub fn new(pos: Position, item: ChestItem, room_index: usize) -> Self {
        Self {
            pos,
            item,
            opened: false,
            room_index,
        }
    }

    pub fn pos(&self) -> Position {
        self.pos
    }

    pub fn is_open(&self) -> bool {
        self.opened
    }

    pub fn item(&self) -> &ChestItem {
        &self.item
    }

    pub fn room_index(&self) -> usize {
        self.room_index
    }

    /// Zwraca przedmiot przy pierwszym otwarciu skrzynki.
    pub fn open(&mut self) -> Option<ChestItem> {
        if self.opened {
            return None;
        }
        self.opened = true;
        Some(self.item.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_stats() -> CombatStats {
        CombatStats {
            hp: 40,
            max_hp: 100,
            attack: 10,
            defense: 5,
            sp_attack: 8,
            sp_defense: 4,
        }
    }

    #[test]
    fn heal_does_not_exceed_max_hp() {
        let mut stats = sample_stats();
        ItemEffect::Heal(100).apply(&mut stats);
        assert_eq!(stats.hp, 100);
    }

    #[test]
    fn boost_max_hp_increases_current_hp() {
        let mut stats = sample_stats();
        ItemEffect::BoostMaxHp(20).apply(&mut stats);
        assert_eq!(stats.max_hp, 120);
        assert_eq!(stats.hp, 60);
    }

    #[test]
    fn boost_attack_increases_attack() {
        let mut stats = sample_stats();
        ItemEffect::BoostAttack(4).apply(&mut stats);
        assert_eq!(stats.attack, 14);
    }

    #[test]
    fn chest_gives_loot_only_once() {
        let mut chest = Chest::new(
            Position { x: 1, y: 2 },
            ChestItem::new("Test", ItemEffect::BoostAttack(2)),
            1,
        );

        assert!(chest.open().is_some());
        assert!(chest.open().is_none());
        assert!(chest.is_open());
    }
}

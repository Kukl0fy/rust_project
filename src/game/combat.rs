use crate::game::direction::Direction;
use crate::game::monster::Monster;
use crate::game::player::Player;

pub enum CombatOutcome {
    Ongoing,
    MonsterDefeated,
    PlayerDefeated,
    Fled,
}

pub struct CombatResult {
    pub outcome: CombatOutcome,
    pub message: String,
}

pub fn process_turn(player: &mut Player, monster: &mut Monster, input: Direction) -> CombatResult {
    match input {
        Direction::Up => {
            let damage = (player.stats.attack - monster.stats.defense).max(1);
            monster.stats.hp -= damage;

            if monster.stats.hp <= 0 {
                return CombatResult {
                    outcome: CombatOutcome::MonsterDefeated,
                    message: format!("Atak fizyczny! Zadajesz {} obrazen. Pokonales potwora!", damage),
                };
            }

            let counter = (monster.stats.attack - player.stats.defense).max(1);
            player.stats.hp -= counter;

            if player.stats.hp <= 0 {
                return CombatResult {
                    outcome: CombatOutcome::PlayerDefeated,
                    message: format!(
                        "Atak fizyczny! Zadajesz {} obrazen. Potwor odpowiada za {} obrazen.",
                        damage, counter
                    ),
                };
            }

            CombatResult {
                outcome: CombatOutcome::Ongoing,
                message: format!(
                    "Atak fizyczny! Zadajesz {} obrazen. Potwor odpowiada za {} obrazen.",
                    damage, counter
                ),
            }
        }
        Direction::Down => {
            let damage = (player.stats.sp_attack - monster.stats.sp_defense).max(1);
            monster.stats.hp -= damage;

            if monster.stats.hp <= 0 {
                return CombatResult {
                    outcome: CombatOutcome::MonsterDefeated,
                    message: format!("Atak specjalny! Zadajesz {} obrazen. Pokonales potwora!", damage),
                };
            }

            let counter = (monster.stats.attack - player.stats.defense).max(1);
            player.stats.hp -= counter;

            if player.stats.hp <= 0 {
                return CombatResult {
                    outcome: CombatOutcome::PlayerDefeated,
                    message: format!(
                        "Atak specjalny! Zadajesz {} obrazen. Potwor odpowiada za {} obrazen.",
                        damage, counter
                    ),
                };
            }

            CombatResult {
                outcome: CombatOutcome::Ongoing,
                message: format!(
                    "Atak specjalny! Zadajesz {} obrazen. Potwor odpowiada za {} obrazen.",
                    damage, counter
                ),
            }
        }
        Direction::Left => CombatResult {
            outcome: CombatOutcome::Fled,
            message: "Uciekasz z walki.".to_string(),
        },
        _ => CombatResult {
            outcome: CombatOutcome::Ongoing,
            message: "W walce: W=atak, S=atak spec., A=ucieczka.".to_string(),
        },
    }
}

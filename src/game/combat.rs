use crate::game::combat_stats::calculate_damage;
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
            let damage = calculate_damage(player.stats.attack, monster.stats.defense);
            monster.stats.hp -= damage;

            if monster.stats.hp <= 0 {
                return CombatResult {
                    outcome: CombatOutcome::MonsterDefeated,
                    message: format!("Atak fizyczny! Zadajesz {} obrazen. Pokonales potwora!", damage),
                };
            }

            let counter = calculate_damage(monster.stats.attack, player.stats.defense);
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
            let damage = calculate_damage(player.stats.sp_attack, monster.stats.sp_defense);
            monster.stats.hp -= damage;

            if monster.stats.hp <= 0 {
                return CombatResult {
                    outcome: CombatOutcome::MonsterDefeated,
                    message: format!("Atak specjalny! Zadajesz {} obrazen. Pokonales potwora!", damage),
                };
            }

            let counter = calculate_damage(monster.stats.attack, player.stats.defense);
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::character_class::CharacterClass;
    use crate::game::monster::MonsterType;
    use crate::game::position::Position;

    #[test]
    fn warrior_special_attack_can_defeat_mind_flayer() {
        let mut player = Player::new(Position { x: 0, y: 0 }, CharacterClass::Warrior);
        let mut monster = Monster::new(Position { x: 1, y: 0 }, MonsterType::MindFlayer, 0);

        let mut turns = 0;
        while monster.stats.hp > 0 && player.stats.hp > 0 && turns < 20 {
            let result = process_turn(&mut player, &mut monster, Direction::Down);
            turns += 1;
            if matches!(result.outcome, CombatOutcome::MonsterDefeated) {
                break;
            }
        }

        assert!(monster.stats.hp <= 0, "monster should be defeated");
        assert!(player.stats.hp > 0, "player should survive using special attacks");
    }

    #[test]
    fn warrior_special_deals_more_than_one_to_tough_enemies() {
        let player = Player::new(Position { x: 0, y: 0 }, CharacterClass::Warrior);
        let monster = Monster::new(Position { x: 1, y: 0 }, MonsterType::MindFlayer, 0);

        let damage = calculate_damage(player.stats.sp_attack, monster.stats.sp_defense);
        assert!(damage >= 10, "special attack should be viable, got {damage}");
    }
}

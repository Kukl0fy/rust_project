use crate::game::direction::Direction;
use crate::game::monster::Monster;
use crate::game::player::Player;

pub enum CombatOutcome {
    Ongoing,
    MonsterDefeated,
    PlayerDefeated,
    Fled,
}

pub fn process_turn(player: &mut Player, monster: &mut Monster, input: Direction) -> CombatOutcome {
    match input {
        Direction::Up => {
            // Atak fizyczny
            let damage = (player.stats.attack - monster.stats.defense).max(1);
            monster.stats.hp -= damage;
        }
        Direction::Down => {
            // Atak specjalny
            let damage = (player.stats.sp_attack - monster.stats.sp_defense).max(1);
            monster.stats.hp -= damage;
        }
        Direction::Left => {
            // Ucieczka
            return CombatOutcome::Fled;
        }
        _ => return CombatOutcome::Ongoing,
    }

    if monster.stats.hp <= 0 {
        return CombatOutcome::MonsterDefeated;
    }

    // atak potwora
    let monster_damage = (monster.stats.attack - player.stats.defense).max(1);
    player.stats.hp -= monster_damage;

    if player.stats.hp <= 0 {
        return CombatOutcome::PlayerDefeated;
    }

    CombatOutcome::Ongoing
}
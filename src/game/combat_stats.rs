#[derive(Clone, Debug)]
pub struct CombatStats {
    pub hp: i32,
    pub max_hp: i32,
    pub attack: i32,
    pub defense: i32,
    pub sp_attack: i32,
    pub sp_defense: i32,
}

/// Damage after defense. When defense exceeds attack, still deal at least 25% of attack.
pub fn calculate_damage(attack: i32, defense: i32) -> i32 {
    let raw = attack - defense;
    if raw >= 1 {
        raw
    } else if attack <= 0 {
        1
    } else {
        ((attack + 3) / 4).max(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normal_hit_subtracts_defense() {
        assert_eq!(calculate_damage(15, 5), 10);
        assert_eq!(calculate_damage(20, 10), 10);
    }

    #[test]
    fn weak_hit_against_high_defense_still_hurts() {
        assert_eq!(calculate_damage(5, 10), 2);
        assert_eq!(calculate_damage(8, 20), 2);
    }

    #[test]
    fn zero_attack_still_deals_one() {
        assert_eq!(calculate_damage(0, 5), 1);
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Effectiveness {
    NoEffect,
    NotVeryEffective,
    Neutral,
    SuperEffective,
}

#[allow(dead_code)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum PokemonType {
    Normal,
    Fighting,
    Flying,
    Poison,
    Ground,
    Rock,
    Bug,
    Ghost,
    Steel,
    Fire,
    Water,
    Grass,
    Electric,
    Psychic,
    Ice,
    Dragon,
    Dark,
}

// Source: [Generations II-V](https://bulbapedia.bulbagarden.net/wiki/Type/Type_chart)
const TYPE_TABLE: [[u8; 17]; 17] = [
    [2, 2, 2, 2, 2, 1, 2, 0, 1, 2, 2, 2, 2, 2, 2, 2, 2], // Normal
    [4, 2, 1, 1, 2, 4, 1, 0, 4, 2, 2, 2, 2, 1, 4, 2, 4], // Fighting
    [2, 4, 2, 2, 2, 1, 4, 2, 1, 2, 2, 4, 1, 2, 2, 2, 2], // Flying
    [2, 2, 2, 1, 1, 1, 2, 1, 0, 2, 2, 4, 2, 2, 2, 2, 2], // Poison
    [2, 2, 0, 4, 2, 4, 1, 2, 4, 4, 2, 1, 4, 2, 2, 2, 2], // Ground
    [2, 1, 4, 2, 1, 2, 4, 2, 1, 4, 2, 2, 2, 2, 4, 2, 2], // Rock
    [2, 1, 1, 1, 2, 2, 2, 1, 1, 1, 2, 4, 2, 4, 2, 2, 4], // Bug
    [0, 2, 2, 2, 2, 2, 2, 4, 2, 2, 2, 2, 2, 4, 2, 2, 1], // Ghost
    [2, 2, 2, 2, 2, 4, 2, 2, 1, 1, 1, 2, 1, 2, 4, 2, 2], // Steel
    [2, 2, 2, 2, 2, 1, 4, 2, 4, 1, 1, 4, 2, 2, 4, 1, 2], // Fire
    [2, 2, 2, 2, 4, 4, 2, 2, 2, 4, 1, 1, 2, 2, 2, 1, 2], // Water
    [2, 2, 1, 1, 4, 4, 1, 2, 1, 1, 4, 1, 2, 2, 2, 1, 2], // Grass
    [2, 2, 4, 2, 0, 2, 2, 2, 2, 2, 4, 1, 1, 2, 2, 1, 2], // Electric
    [2, 4, 2, 4, 2, 2, 2, 2, 1, 2, 2, 2, 2, 1, 2, 2, 0], // Psychic
    [2, 2, 4, 2, 4, 2, 2, 2, 1, 1, 1, 4, 2, 2, 1, 4, 2], // Ice
    [2, 2, 2, 2, 2, 2, 2, 2, 1, 2, 2, 2, 2, 2, 2, 4, 2], // Dragon
    [2, 1, 2, 2, 2, 2, 2, 4, 2, 2, 2, 2, 2, 4, 2, 2, 1], // Dark
];

impl PokemonType {
    pub fn get_effectiveness(attacker: PokemonType, defender: PokemonType) -> Effectiveness {
        match TYPE_TABLE[attacker as usize][defender as usize] {
            0 => Effectiveness::NoEffect,
            1 => Effectiveness::NotVeryEffective,
            2 => Effectiveness::Neutral,
            4 => Effectiveness::SuperEffective,
            _ => unreachable!(),
        }
    }

    fn _get_effectiveness_by_match(attacker: PokemonType, defender: PokemonType) -> Effectiveness {
        match (attacker, defender) {
            (PokemonType::Normal, PokemonType::Rock) => Effectiveness::NotVeryEffective,
            (PokemonType::Normal, PokemonType::Ghost) => Effectiveness::NoEffect,

            (PokemonType::Fire, PokemonType::Grass) => Effectiveness::SuperEffective,
            (PokemonType::Water, PokemonType::Grass) => Effectiveness::NotVeryEffective,
            (PokemonType::Fighting, PokemonType::Ghost) => Effectiveness::NoEffect,

            (PokemonType::Grass, PokemonType::Flying) => Effectiveness::NotVeryEffective,
            (PokemonType::Grass, PokemonType::Poison) => Effectiveness::NotVeryEffective,
            (PokemonType::Grass, PokemonType::Ground) => Effectiveness::SuperEffective,
            (PokemonType::Grass, PokemonType::Rock) => Effectiveness::SuperEffective,
            (PokemonType::Grass, PokemonType::Bug) => Effectiveness::NotVeryEffective,
            (PokemonType::Grass, PokemonType::Fire) => Effectiveness::NotVeryEffective,
            (PokemonType::Grass, PokemonType::Water) => Effectiveness::SuperEffective,
            (PokemonType::Grass, PokemonType::Grass) => Effectiveness::NotVeryEffective,
            (PokemonType::Grass, PokemonType::Dragon) => Effectiveness::NotVeryEffective,

            (PokemonType::Electric, PokemonType::Flying) => Effectiveness::SuperEffective,
            (PokemonType::Electric, PokemonType::Ground) => Effectiveness::NoEffect,
            (PokemonType::Electric, PokemonType::Water) => Effectiveness::SuperEffective,
            (PokemonType::Electric, PokemonType::Grass) => Effectiveness::NotVeryEffective,
            (PokemonType::Electric, PokemonType::Electric) => Effectiveness::NotVeryEffective,
            (PokemonType::Electric, PokemonType::Dragon) => Effectiveness::NotVeryEffective,

            _ => Effectiveness::Neutral,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_table() {
        assert_eq!(
            PokemonType::get_effectiveness(PokemonType::Fire, PokemonType::Grass),
            Effectiveness::SuperEffective
        );
        assert_eq!(
            PokemonType::get_effectiveness(PokemonType::Water, PokemonType::Grass),
            Effectiveness::NotVeryEffective
        );
        assert_eq!(
            PokemonType::get_effectiveness(PokemonType::Fighting, PokemonType::Ghost),
            Effectiveness::NoEffect
        );
        assert_eq!(
            PokemonType::get_effectiveness(PokemonType::Normal, PokemonType::Normal),
            Effectiveness::Neutral
        );
    }

    #[test]
    fn test_get_effectiveness_by_match() {
        assert_eq!(
            PokemonType::_get_effectiveness_by_match(PokemonType::Fire, PokemonType::Grass),
            Effectiveness::SuperEffective
        );
        assert_eq!(
            PokemonType::_get_effectiveness_by_match(PokemonType::Water, PokemonType::Grass),
            Effectiveness::NotVeryEffective
        );
        assert_eq!(
            PokemonType::_get_effectiveness_by_match(PokemonType::Fighting, PokemonType::Ghost),
            Effectiveness::NoEffect
        );
        assert_eq!(
            PokemonType::_get_effectiveness_by_match(PokemonType::Normal, PokemonType::Normal),
            Effectiveness::Neutral
        );
    }
}

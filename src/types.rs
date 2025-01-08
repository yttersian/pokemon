// Source: [Generations II-V](https://bulbapedia.bulbagarden.net/wiki/Type/Type_chart)
const TYPE_TABLE: [[u8; 17]; 17] = [
    // Fi Fl Po Gd Ro Bg Gh St Fi Wa Gs El Ps Ic Dr Da
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Effectiveness {
    NoEffect,
    NotVeryEffective,
    Neutral,
    SuperEffective,
}

#[allow(dead_code)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
pub enum PokemonType {
    #[default]
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

impl PokemonType {
    fn _get_effectiveness(attacker: PokemonType, defender: PokemonType) -> Effectiveness {
        use Effectiveness::*;
        use PokemonType::*;
        match (attacker, defender) {
            (Normal, Rock) => NotVeryEffective,
            (Normal, Rock) => NotVeryEffective,
            (Normal, Ghost) => NoEffect,
            (Normal, Steel) => NotVeryEffective,

            (Fighting, Normal) => SuperEffective,
            (Fighting, Flying) => NotVeryEffective,
            (Fighting, Poison) => NotVeryEffective,
            (Fighting, Rock) => SuperEffective,
            (Fighting, Ghost) => NoEffect,
            (Fighting, Steel) => SuperEffective,
            (Fighting, Psychic) => NotVeryEffective,
            (Fighting, Ice) => SuperEffective,
            (Fighting, Dark) => SuperEffective,

            (Flying, Fighting) => SuperEffective,
            (Flying, Rock) => NotVeryEffective,
            (Flying, Bug) => SuperEffective,
            (Flying, Steel) => NotVeryEffective,
            (Flying, Grass) => SuperEffective,
            (Flying, Electric) => NotVeryEffective,

            (Poison, Poison) => NotVeryEffective,
            (Poison, Ground) => NotVeryEffective,
            (Poison, Rock) => NotVeryEffective,
            (Poison, Ghost) => NotVeryEffective,
            (Poison, Steel) => NoEffect,
            (Poison, Grass) => SuperEffective,

            (Ground, Flying) => NoEffect,
            (Ground, Poison) => SuperEffective,
            (Ground, Rock) => SuperEffective,
            (Ground, Bug) => NotVeryEffective,
            (Ground, Steel) => SuperEffective,
            (Ground, Fire) => SuperEffective,
            (Ground, Grass) => NotVeryEffective,
            (Ground, Electric) => SuperEffective,

            (Fire, Grass) => SuperEffective,
            (Water, Grass) => NotVeryEffective,

            (Grass, Flying) => NotVeryEffective,
            (Grass, Poison) => NotVeryEffective,
            (Grass, Ground) => SuperEffective,
            (Grass, Rock) => SuperEffective,
            (Grass, Bug) => NotVeryEffective,
            (Grass, Fire) => NotVeryEffective,
            (Grass, Water) => SuperEffective,
            (Grass, Grass) => NotVeryEffective,
            (Grass, Dragon) => NotVeryEffective,

            (Electric, Flying) => SuperEffective,
            (Electric, Ground) => NoEffect,
            (Electric, Water) => SuperEffective,
            (Electric, Grass) => NotVeryEffective,
            (Electric, Electric) => NotVeryEffective,
            (Electric, Dragon) => NotVeryEffective,

            _ => Neutral,
        }
    }

    pub fn get_effectiveness(attacker: PokemonType, defender: PokemonType) -> Effectiveness {
        use Effectiveness::*;
        match TYPE_TABLE[attacker as usize][defender as usize] {
            0 => NoEffect,
            1 => NotVeryEffective,
            2 => Neutral,
            4 => SuperEffective,
            _ => unreachable!(),
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
            PokemonType::_get_effectiveness(PokemonType::Fire, PokemonType::Grass),
            Effectiveness::SuperEffective
        );
        assert_eq!(
            PokemonType::_get_effectiveness(PokemonType::Water, PokemonType::Grass),
            Effectiveness::NotVeryEffective
        );
        assert_eq!(
            PokemonType::_get_effectiveness(PokemonType::Fighting, PokemonType::Ghost),
            Effectiveness::NoEffect
        );
        assert_eq!(
            PokemonType::_get_effectiveness(PokemonType::Normal, PokemonType::Normal),
            Effectiveness::Neutral
        );
    }
}

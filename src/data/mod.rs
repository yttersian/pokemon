use crate::pokemon::{Pokemon, Stats};
use crate::types::PokemonType;

use crate::moves::{Move, MoveCategory};

#[non_exhaustive]
enum EvolutionItem {
    FireStone,
    WaterStone,
    ThunderStone,
}

enum EvolutionCondition {
    Level(u8),
    Item(EvolutionItem),
}

struct EvolutionPath {
    evolves_to: SpeciesId,
    condition: EvolutionCondition,
}

pub struct SpeciesId(u16);

pub struct Species {
    species_id: SpeciesId,
    base_stats: Stats,
    types: Vec<PokemonType>,
    learnset: Vec<(u8, Move)>,
    evolutions: Vec<EvolutionPath>,
}

impl Species {
    pub fn new(
        species_id: SpeciesId,
        base_stats: Stats,
        types: Vec<PokemonType>,
        learnset: Vec<(u8, Move)>,
        evolutions: Vec<EvolutionPath>,
    ) -> Self {
        Self {
            species_id,
            base_stats,
            types,
            learnset,
            evolutions,
        }
    }
}

pub fn load_pokemon_db() -> String {
    let data = std::fs::read_to_string("./src/data/pokemon_db.json")
        .expect("Unable to read Pokemon database");
    // let pokemon_db: HashMap<String, Pokemon> =

    data
}

pub fn get_pokemon(species: &str) -> Pokemon {
    let types: Vec<PokemonType>;
    let stats: Stats;
    let moves: Vec<Move>;
    let evolutions: Vec<EvolutionPath>;
    let learnset: Vec<(u8, &str)>;

    match species {
        "Bulbasaur" => {
            stats = Stats {
                max_hp: 45,
                attack: 49,
                defense: 49,
                special_attack: 65,
                special_defense: 65,
                speed: 45,
            };
            types = vec![PokemonType::Grass];
            moves = vec![get_move("Tackle"), get_move("Vine Whip")];
            evolutions = vec![EvolutionPath {
                evolves_to: SpeciesId(2), // Ivysaur
                condition: EvolutionCondition::Level(16),
            }];
        }

        "Pikachu" => {
            stats = Stats {
                max_hp: 35,
                attack: 55,
                defense: 30,
                special_attack: 50,
                special_defense: 40,
                speed: 90,
            };
            types = vec![PokemonType::Electric];
            moves = vec![get_move("Tackle"), get_move("Thunderbolt")];
            evolutions = vec![EvolutionPath {
                evolves_to: SpeciesId(26), // Raichu
                condition: EvolutionCondition::Item(EvolutionItem::ThunderStone),
            }];
            learnset = vec![
                (1, "Thunder Shock"),
                (1, "Growl"),
                (5, "Tail Whip"),
                (10, "Thunder Wave"),
                (13, "Quick Attack"),
                (18, "Double Team"),
                (21, "Slam"),
                (26, "Thunderbolt"),
                (29, "Feint"),
                (34, "Agility"),
                (35, "Discharge"),
                (42, "Light Screen"),
                (45, "Thunder"),
            ];
        }

        _ => unimplemented!("The Pokemon '{}' is not implemented.", species),
    }

    Pokemon::builder(species)
        .stats(stats)
        .types(types)
        .moves(moves)
        .build()
}

pub fn get_move(name: &str) -> Move {
    match name {
        "Tackle" => Move {
            name: name.to_string(),
            power: 40,
            pp: 35,
            max_pp: 35,
            accuracy: 100,
            move_type: PokemonType::Normal,
            category: MoveCategory::Physical,
        },

        "Thunderbolt" => Move {
            name: "Thunderbolt".to_string(),
            power: 90,
            pp: 15,
            max_pp: 15,
            accuracy: 100,
            move_type: PokemonType::Electric,
            category: MoveCategory::Special,
        },

        "Vine Whip" => Move {
            name: name.to_string(),
            power: 45,
            pp: 25,
            max_pp: 25,
            accuracy: 100,
            move_type: PokemonType::Grass,
            category: MoveCategory::Physical,
        },

        _ => unimplemented!("The Move '{}' is not implemented.", name),
    }
}

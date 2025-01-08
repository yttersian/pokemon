use crate::pokemon::{Pokemon, Stats};
use crate::types::PokemonType;

use crate::moves::{Move, MoveCategory};

fn load_pokemon_db() {
    todo!();
}

pub fn get_pokemon(species: &str) -> Pokemon {
    let types: Vec<PokemonType>;
    let stats: Stats;
    let moves: Vec<Move>;

    match species {
        "Bulbasaur" => {
            types = vec![PokemonType::Grass];
            stats = Stats {
                max_hp: 45,
                attack: 49,
                defense: 49,
                special_attack: 65,
                special_defense: 65,
                speed: 45,
            };
            moves = vec![get_move("Tackle"), get_move("Vine Whip")];
        }

        "Pikachu" => {
            types = vec![PokemonType::Electric];
            stats = Stats {
                max_hp: 35,
                attack: 55,
                defense: 30,
                special_attack: 50,
                special_defense: 40,
                speed: 90,
            };
            moves = vec![get_move("Tackle"), get_move("Thunderbolt")];
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

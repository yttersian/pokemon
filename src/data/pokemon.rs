use crate::data::moves::get_move;
use crate::moves::Move;
use crate::pokemon::{Pokemon, Stats};
use crate::types::PokemonType;

pub fn get_pokemon(name: &str) -> Pokemon {
    let pokemon_type: PokemonType;
    let stats: Stats;
    let moves: Vec<Move>;

    match name {
        "Bulbasaur" => {
            pokemon_type = PokemonType::Grass;
            stats = Stats {
                hp: 45,
                attack: 49,
                defense: 49,
                special_attack: 65,
                special_defense: 65,
                speed: 45,
            };
            moves = vec![get_move("Tackle"), get_move("Vine Whip")];
        }

        "Pikachu" => {
            pokemon_type = PokemonType::Electric;
            stats = Stats {
                hp: 35,
                attack: 55,
                defense: 30,
                special_attack: 50,
                special_defense: 40,
                speed: 90,
            };
            moves = vec![get_move("Tackle"), get_move("Thunderbolt")];
        }

        _ => unimplemented!("The Pokemon '{}' is not implemented.", name),
    }

    let pokemon = Pokemon::new(name, pokemon_type, stats, moves);

    pokemon
}

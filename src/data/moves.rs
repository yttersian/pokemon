use rand::Rng;

use crate::moves::{Move, MoveCategory};
use crate::pokemon::StatusCondition;
use crate::types::PokemonType;

pub fn get_move(name: &str) -> Move {
    match name {
        "Tackle" => Move {
            name: name.to_string(),
            move_type: PokemonType::Normal,
            category: MoveCategory::Physical,
            power_point: 35,
            power: 40,
            accuracy: 100,
            special_effect: None,
        },

        "Thunderbolt" => Move {
            name: "Thunderbolt".to_string(),
            move_type: PokemonType::Electric,
            power_point: 15,
            power: 90,
            accuracy: 100,
            category: MoveCategory::Special,
            special_effect: Some(Box::new(|_user, _target| {
                let mut rng = rand::thread_rng();
                if rng.gen_range(0..100) < 10 {
                    println!("{} is paralyzed!", _target.name);
                    _target.status = Some(StatusCondition::Paralyzed);
                }
            })),
        },

        "Vine Whip" => Move {
            name: name.to_string(),
            move_type: PokemonType::Grass,
            category: MoveCategory::Physical,
            power_point: 25,
            power: 45,
            accuracy: 100,
            special_effect: None,
        },

        _ => unimplemented!("The Move '{}' is not implemented.", name),
    }
}

use crate::moves::{Move, MoveCategory};
use crate::pokemon::Pokemon;
use crate::types::{Effectiveness, PokemonType};
use rand::Rng;

pub fn use_move(move_index: usize, user: &mut Pokemon, target: &mut Pokemon) {
    if !user.is_alive() {
        println!("{} cannot move because it has fainted.", user.name);
        return;
    }

    let selected_move = &user.moves[move_index];

    if selected_move.power > 0 {
        if !target.is_alive() {
            println!(
                "{} cannot attack {} because it has fainted.",
                user.name, target.name
            );
            return;
        }

        println!(
            "{} used {} on {}!",
            user.name, selected_move.name, target.name
        );
        // todo! add accuracy

        let damage = get_move_damage(selected_move, &user, &target);

        target.take_damage(damage);
    }
}

fn get_move_damage(selected_move: &Move, user: &Pokemon, target: &Pokemon) -> i32 {
    let effectiveness =
        match PokemonType::get_effectiveness(selected_move.move_type, target.pokemon_type) {
            Effectiveness::NoEffect => {
                println!("It has no effect");
                return 0;
            }

            Effectiveness::NotVeryEffective => {
                println!("It's not very effective...");
                0.5
            }

            Effectiveness::Neutral => 1.0,

            Effectiveness::SuperEffective => {
                println!("It's super effective!");
                2.0
            }
        };

    let stab = if selected_move.move_type == user.pokemon_type {
        1.5
    } else {
        1.0
    };

    let level_modifier = user.level as f32 * 2.0 / 5.0 + 2.0;
    let stat_ratio = match selected_move.category {
        MoveCategory::Physical => user.stats.attack as f32 / target.stats.defense as f32,
        MoveCategory::Special => {
            user.stats.special_attack as f32 / target.stats.special_defense as f32
        }
    };
    let base_damage = level_modifier * selected_move.power as f32 * stat_ratio / 50.0 + 2.0;

    (base_damage * stab * effectiveness).round() as i32
}

pub fn fight_once(pokemon1: &mut Pokemon, pokemon2: &mut Pokemon) {
    if pokemon1.stats.speed >= pokemon2.stats.speed {
        use_move(0, pokemon1, pokemon2);
        if pokemon2.is_alive() {
            use_move(0, pokemon2, pokemon1);
        }
    } else {
        use_move(0, pokemon2, pokemon1);
        if pokemon1.is_alive() {
            use_move(0, pokemon1, pokemon2);
        }
    }
}

pub fn fight(pokemon1: &mut Pokemon, pokemon2: &mut Pokemon) {
    println!("{} and {} are fighting!", pokemon1.name, pokemon2.name);
    while pokemon1.is_alive() && pokemon2.is_alive() {
        fight_once(pokemon1, pokemon2);
    }
}

pub fn fight_with_random_moves(pokemon1: &mut Pokemon, pokemon2: &mut Pokemon) {
    println!("{} and {} are fighting!", pokemon1.name, pokemon2.name);
    let mut rng = rand::thread_rng();
    while pokemon1.is_alive() && pokemon2.is_alive() {
        let pokemon1_move = rng.gen_range(0..pokemon1.moves.len());
        let pokemon2_move = rng.gen_range(0..pokemon2.moves.len());

        if pokemon1.stats.speed >= pokemon2.stats.speed {
            use_move(pokemon1_move, pokemon1, pokemon2);
            if pokemon2.is_alive() {
                use_move(pokemon2_move, pokemon2, pokemon1);
            }
        } else {
            use_move(pokemon2_move, pokemon2, pokemon1);
            if pokemon1.is_alive() {
                use_move(pokemon1_move, pokemon1, pokemon2);
            }
        }
    }
}

use crate::moves::{Move, MoveCategory};
use crate::pokemon::Pokemon;
use crate::trainer::Trainer;
use crate::types::{Effectiveness, PokemonType};
use rand::Rng;

struct StatModifiers {
    attack: i8,
    defense: i8,
    special_attack: i8,
    special_defense: i8,
    speed: i8,
}

struct BattlePokemon {
    pokemon: Pokemon,
    stat_modifiers: StatModifiers,
}

pub fn start_battle(player1: &mut Trainer, player2: &mut Trainer) {
    todo!()
}

pub fn perform_turn(pokemon1: &mut Pokemon, pokemon2: &mut Pokemon) {
    let pokemon1_mv = choose_move(pokemon1);
    let pokemon2_mv = choose_move(pokemon2);

    use_move(0, pokemon1, pokemon2);
    if !pokemon2.is_fainted() {
        use_move(0, pokemon2, pokemon1);
    } else {
        use_move(0, pokemon2, pokemon1);
        if !pokemon1.is_fainted() {
            use_move(0, pokemon1, pokemon2);
        }
    }
}

pub fn battle(pokemon1: &mut Pokemon, pokemon2: &mut Pokemon) {
    println!("{} and {} are fighting!", pokemon1.name, pokemon2.name);

    while !pokemon1.is_fainted() && !pokemon2.is_fainted() {
        perform_turn(pokemon1, pokemon2);
    }
}

pub fn battle_with_random_moves(pokemon1: &mut Pokemon, pokemon2: &mut Pokemon) {
    println!("{} and {} are fighting!", pokemon1.name, pokemon2.name);
    let mut rng = rand::thread_rng();
    while !pokemon1.is_fainted() && !pokemon2.is_fainted() {
        let pokemon1_move = rng.gen_range(0..pokemon1.moves.len());
        let pokemon2_move = rng.gen_range(0..pokemon2.moves.len());

        if pokemon1.stats.speed >= pokemon2.stats.speed {
            use_move(pokemon1_move, pokemon1, pokemon2);
            if !pokemon2.is_fainted() {
                use_move(pokemon2_move, pokemon2, pokemon1);
            }
        } else {
            use_move(pokemon2_move, pokemon2, pokemon1);
            if !pokemon1.is_fainted() {
                use_move(pokemon1_move, pokemon1, pokemon2);
            }
        }
    }
}

fn choose_move(pokemon: &Pokemon) {
    todo!()
}

pub fn use_move(move_index: usize, user: &mut Pokemon, target: &mut Pokemon) {
    if user.is_fainted() {
        println!("{} cannot move because it has fainted.", user.name);
        return;
    }

    let selected_move = &user.moves[move_index];

    if selected_move.power > 0 {
        if target.is_fainted() {
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

fn get_move_damage(selected_move: &Move, user: &Pokemon, target: &Pokemon) -> u32 {
    let effectiveness =
        match PokemonType::get_effectiveness(selected_move.move_type, PokemonType::Bug) {
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

    let stab = if selected_move.move_type == PokemonType::Bug {
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

    (base_damage * stab * effectiveness).round() as u32
}

#![allow(dead_code, unused)]

// mod battle;
mod constants;
mod data;
mod moves;
mod pokemon;
mod trainer;
mod types;
mod utils;

// use battle::*;
use data::get_pokemon;
use pokemon::Pokemon;
use trainer::Trainer;

fn main() {
    let mut leaf = Trainer::new("Leaf");
    let mut red = Trainer::new("Red");

    leaf.add_pokemon(get_pokemon("Bulbasaur"));
    red.add_pokemon(get_pokemon("Pikachu"));

    dbg!(leaf);
    dbg!(red);

    // start_battle(&mut leaf, &mut red);
}

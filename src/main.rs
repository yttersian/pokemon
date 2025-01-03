mod battle;
mod data;
mod moves;
mod pokemon;
mod types;

use battle::*;
use data::pokemon::get_pokemon;

fn main() {
    let mut pikachu = get_pokemon("Pikachu");
    pikachu.level = 10;

    let mut bulbasaur = get_pokemon("Bulbasaur");
    bulbasaur.level = 10;
    // let mut squirtle = get_pokemon("Squirtle");

    // fight(&mut pikachu, &mut bulbasaur);
    fight_with_random_moves(&mut pikachu, &mut bulbasaur);
    // dbg!(pikachu);
    // dbg!(bulbasaur);
}

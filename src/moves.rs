use crate::types::PokemonType;

#[derive(Debug)]
pub enum MoveCategory {
    Physical,
    Special,
}

#[derive(Debug)]
pub struct Move {
    pub name: String,
    pub power: u32,
    pub pp: u8,
    pub max_pp: u8,
    pub accuracy: u8,
    pub move_type: PokemonType,
    pub category: MoveCategory,
}

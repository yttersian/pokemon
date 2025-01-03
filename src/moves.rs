use std::fmt::Debug;

use crate::{pokemon::Pokemon, types::PokemonType};

#[derive(Debug)]
pub enum MoveCategory {
    Physical,
    Special,
}

pub struct Move {
    pub name: String,
    pub move_type: PokemonType,
    pub category: MoveCategory,
    pub power_point: u8,
    pub power: u8,
    pub accuracy: u8,
    pub special_effect: Option<Box<dyn FnMut(&mut Pokemon, &mut Pokemon)>>,
}

impl Debug for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Move")
            .field("name", &self.name)
            .field("move_type", &self.move_type)
            .field("category", &self.category)
            .field("power_point", &self.power_point)
            .field("power", &self.power)
            .field("accuracy", &self.accuracy)
            // Omitting special_effect from Debug output
            .finish()
    }
}

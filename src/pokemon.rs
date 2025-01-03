use crate::moves::Move;
use crate::types::PokemonType;

#[derive(Debug, Clone, Copy)]
pub struct Stats {
    pub hp: i32,
    pub attack: i32,
    pub defense: i32,
    pub special_attack: i32,
    pub special_defense: i32,
    pub speed: i32,
}

#[derive(Debug)]
pub enum StatusCondition {
    Paralyzed,
}

#[derive(Debug)]
pub struct Pokemon {
    pub name: String,
    pub pokemon_type: PokemonType,
    pub stats: Stats,
    pub level: u8,
    pub current_hp: i32,
    pub moves: Vec<Move>,
    pub status: Option<StatusCondition>,
}

impl Pokemon {
    pub fn new(name: &str, pokemon_type: PokemonType, stats: Stats, moves: Vec<Move>) -> Self {
        let pokemon = Self {
            name: name.to_string(),
            pokemon_type,
            stats,
            level: 1,
            current_hp: stats.hp,
            moves,
            status: None,
        };

        println!("{} spawned with {} health", name, pokemon.current_hp);

        pokemon
    }

    pub fn is_alive(&self) -> bool {
        self.current_hp > 0
    }

    pub fn take_damage(&mut self, amount: i32) {
        if !self.is_alive() {
            return;
        }
        println!("{} took {} damage!", self.name, amount);
        self.current_hp = (self.current_hp - amount).max(0);

        if self.is_alive() {
            println!("{}'s remaining health: {}", self.name, self.current_hp);
        } else {
            println!("{} fainted.", self.name);
        }
    }
}

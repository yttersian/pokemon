use crate::constants::{POKEMON_MAX_MOVES, POKEMON_MAX_TYPES};
use std::default;

use crate::moves::Move;
use crate::types::PokemonType;

////////////////////////////////////////////////////////////////////////////////
// Core Data Structures
////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
#[non_exhaustive]
pub enum StatusCondition {
    Paralyzed,
}

#[derive(Debug)]
pub struct Stats {
    pub max_hp: u32,
    pub attack: u32,
    pub defense: u32,
    pub special_attack: u32,
    pub special_defense: u32,
    pub speed: u32,
}

impl Default for Stats {
    fn default() -> Self {
        Self {
            max_hp: 10,
            attack: 10,
            defense: 10,
            special_attack: 10,
            special_defense: 10,
            speed: 10,
        }
    }
}

#[derive(Debug)]
pub struct Pokemon {
    pub name: String,
    species: String,
    level: u8,
    hp: u32,
    stats: Stats,
    types: Vec<PokemonType>,
    moves: Vec<Move>,
    status: Option<StatusCondition>,
}

impl Pokemon {
    pub fn new(species: &str) -> Pokemon {
        Self::builder(species).build()
    }

    pub fn builder(species: &str) -> PokemonBuilder {
        PokemonBuilder {
            species: species.into(),
            level: 1,
            stats: Stats::default(),
            types: vec![PokemonType::default()],
            moves: Vec::new(),
        }
    }

    pub fn from_species(name: &str) -> Self {
        todo!()
    }

    pub fn level_up(&mut self) {
        todo!()
    }

    pub fn is_fainted(&self) -> bool {
        self.hp == 0
    }

    pub fn heal(&mut self, amount: u32) {
        self.hp = (self.hp + amount).min(self.stats.max_hp);
    }

    pub fn full_heal(&mut self) {
        self.hp = self.stats.max_hp;
    }

    pub fn take_damage(&mut self, amount: u32) {
        self.hp = self.hp.saturating_sub(amount);
    }
}

////////////////////////////////////////////////////////////////////////////////
// Creation Logic
////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct PokemonBuilder {
    species: String,
    level: u8,
    stats: Stats,
    types: Vec<PokemonType>,
    moves: Vec<Move>,
}

impl PokemonBuilder {
    pub fn level(mut self, level: u8) -> Self {
        todo!()
    }

    pub fn stats(mut self, stats: Stats) -> Self {
        self.stats = stats;
        self
    }

    pub fn types(mut self, types: Vec<PokemonType>) -> Self {
        assert!(
            types.len() <= POKEMON_MAX_TYPES,
            "Cannot create a Pokemon with more than {} types.",
            POKEMON_MAX_TYPES
        );
        self.types = types;
        self
    }

    pub fn moves(mut self, moves: Vec<Move>) -> Self {
        assert!(
            moves.len() <= POKEMON_MAX_MOVES,
            "Cannot create a Pokemon with more than {} moves.",
            POKEMON_MAX_MOVES
        );
        self.moves = moves;
        self
    }

    pub fn build(self) -> Pokemon {
        Pokemon {
            name: self.species.clone(),
            species: self.species,
            level: self.level,
            hp: self.stats.max_hp,
            stats: self.stats,
            types: self.types,
            moves: self.moves,
            status: None,
        }
    }
}

use crate::constants::MAX_PARTY_SIZE;
use crate::pokemon::Pokemon;

#[derive(Debug)]
pub struct Party {
    members: Vec<Pokemon>,
}

impl Party {
    pub fn new() -> Self {
        Self {
            members: Vec::new(),
        }
    }

    pub fn from_pokemon(pokemon: Vec<Pokemon>) -> Self {
        assert!(
            pokemon.len() <= MAX_PARTY_SIZE,
            "Cannot create a party with more than {} Pokemon.",
            MAX_PARTY_SIZE
        );
        Self { members: pokemon }
    }

    pub fn size(&self) -> usize {
        self.members.len()
    }

    pub fn is_full(&self) -> bool {
        self.size() >= MAX_PARTY_SIZE
    }

    pub fn add(&mut self, pokemon: Pokemon) -> Result<(), String> {
        if self.is_full() {
            Err(format!("Team is full! Cannot add {}.", pokemon.name))
        } else {
            self.members.push(pokemon);
            Ok(())
        }
    }
}

#[derive(Debug)]
pub struct Trainer {
    pub name: String,
    pub party: Party,
}

impl Trainer {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            party: Party::new(),
        }
    }

    pub fn with_pokemon(trainer_name: &str, pokemon: Vec<Pokemon>) -> Self {
        Self {
            name: trainer_name.to_string(),
            party: Party::from_pokemon(pokemon),
        }
    }

    pub fn is_party_full(&self) -> bool {
        self.party.is_full()
    }

    pub fn add_pokemon(&mut self, pokemon: Pokemon) -> Result<(), String> {
        self.party.add(pokemon)
    }

    pub fn get_party(&self) -> &[Pokemon] {
        &self.party.members
    }
}

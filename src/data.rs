use std::error::Error;
use std::collections::HashMap;

use crate::common::pokedex::Species;
use crate::common::moves::Move;
use crate::common::types::Type;

pub struct Data {
    pokedex: HashMap<u32, Species>,
    moves: HashMap<u32, Move>,
    types: HashMap<u32, Type>,
}

impl Data {
    pub fn get_species(&self, id: u32) -> Option<&Species> {
        self.pokedex.get(&id)
    }

    pub fn get_move(&self, id: u32) -> Option<&Move> {
        self.moves.get(&id)
    }

    pub fn get_type(&self, id: u32) -> Option<&Type> {
        self.types.get(&id)
    }
}

pub fn setup() -> Result<Data, Box<dyn Error>> {
    let pokedex = HashMap::new();
    let moves = HashMap::new();
    let types = HashMap::new();

    Ok(Data { pokedex, moves, types })
}
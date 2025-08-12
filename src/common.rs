use serde::{Serialize, Deserialize};

#[derive(Deserialize)]
pub struct Move {
    accuracy: Option<u32>,
    category: String,
    name: String,
    power: Option<u32>,
    pp: u32,
    typ: String,
}

pub struct Stats {
    hp: u32,
    att: u32,
    def: u32,
    spa: u32,
    spd: u32,
    spe: u32,
}


// abstract pokemon struct
pub struct Species {
    id: u32,
    name: String,
    types: (String, Option<String>), // single type or double type
    stats: Stats,
    abilities: (String, Option<String>, Option<String>), // at least 1 ability, max 3
    gender: Option<(f64, f64)>, // male/female ratio or genderless
}

pub struct Pokedex {
    index: Vec<Species>,
}


pub fn read_pokedex(path: &str) -> Result<> {
    Serialize::serialize(&self, serializer)
}
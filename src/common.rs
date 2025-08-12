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

#[derive(PartialEq)]
pub enum Type {
    Normal,
    Grass,
    Fire,
    Water,
    Fighting,
    Flying,
    Poison,
    Electric,
    Ground,
    Psychic,
    Rock,
    Ice,
    Bug,
    Dragon,
    Ghost,
    Dark,
    Steel,
    Fairy,
}

pub struct Stats {
    hp: u32,
    attack: u32,
    defense: u32,
    
}


pub struct Species {
    id: u32,
    name: &'static str,
    types: (Type, Option<Type>)
    base_stats: 
}


pub fn file_to_json(path: &str) -> Result<> {
    Serialize::serialize(&self, serializer)
}
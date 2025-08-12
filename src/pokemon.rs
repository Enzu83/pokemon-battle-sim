use crate::common::Move;

pub struct Pokemon {
    name: String,
    level: u32,
    moves: [Option<&'static Move>; 4],

    // hp: u32,
    // att: u32,
    // def: u32,
    // spa: u32,
    // spd: u32,
    // spe: u32,
}

impl Pokemon {
    pub fn new(name: &str) -> Self {
        Self { 
            name: name.to_string(),
            level: 5,
            moves: [None; 4],
        }
    }
}
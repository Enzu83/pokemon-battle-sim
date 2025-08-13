use super::moves::Move;

enum Gender {
    Male,
    Female,
}

pub struct Pokemon {
    name: String,
    types: (String, Option<String>), // single type or double type
    abilities: String,
    gender: Option<Gender>,
    
    level: u32,
    hp: u32,
    att: u32,
    def: u32,
    spa: u32,
    spd: u32,
    spe: u32,
    moves: (&'static Move, Option<&'static Move>, Option<&'static Move>, Option<&'static Move>),
}
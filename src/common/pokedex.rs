use std::collections::HashMap;

// abstract pokemon struct
pub struct Species {
    name: String,
    types: (String, Option<String>), // single type or double type
    abilities: (String, Option<String>, Option<String>), // at least 1 ability, max 3
    gender_ratio: Option<(f64, f64)>, // male/female ratio or genderless
    hp: u32,
    att: u32,
    def: u32,
    spa: u32,
    spd: u32,
    spe: u32,
}
use super::types::{Type, string_to_type};

use std::collections::HashMap;
use std::error::Error;
use std::fs::read_to_string;

use serde::Deserialize;
use serde::de::Deserializer;

#[derive(Debug)]
enum Category {
    PHYSICAL,
    SPECIAL,
    STATUS,
}

#[derive(Debug)]
pub struct Move {
    name: String,
    power: Option<u32>,
    typ: Type,
    accuracy: Option<u32>,
    pp: Option<u32>,
    category: Category,
}

pub struct Moves {
    map: HashMap<String, Move>,
}

impl Moves {
    pub fn get(&self, name: &str) -> Option<&Move> {
        self.map.get(name)
    }
}

#[derive(Deserialize)]
struct MoveJsonRow {
    name: String,

    #[serde(rename = "type", deserialize_with = "string_to_type")]
    typ: Type,
    power: Option<u32>,
    accuracy: Option<u32>,
    pp: Option<u32>,

    #[serde(deserialize_with = "string_to_category")]
    category: Category,
    
}

fn string_to_category<'a, D>(deserializer: D) -> Result<Category, D::Error>
where 
    D: Deserializer<'a>
{
    let string = String::deserialize(deserializer)?;
    
    match string.as_str() {
        "physical" => Ok(Category::PHYSICAL),
        "special" => Ok(Category::SPECIAL),
        "status" => Ok(Category::STATUS),
        _ => Ok(Category::STATUS),
    }
}

pub fn read_moves(file_path: String) -> Result<Moves, Box<dyn Error>> {
    let file_content = read_to_string(file_path)?;
    let json: Vec<MoveJsonRow> = serde_json::from_str(&file_content)?;

    let mut moves_map = HashMap::new();

    for row in json {
        let mov = Move {
            name: row.name.clone(),
            power: row.power,
            typ: row.typ,
            accuracy: row.accuracy,
            pp: row.pp,
            category: row.category
        };
        moves_map.insert(row.name, mov);
    }

    Ok(Moves { map: moves_map })
}
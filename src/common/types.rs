use std::collections::HashMap;
use std::error::Error;
use std::fs::read_to_string;

use serde::Deserialize;
use serde::de::Deserializer;

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Type {
    UNKNOWN,
    NORMAL,
    FIRE,
    WATER,
    ELECTRIC,
    GRASS,
    ICE,
    FIGHTING,
    POISON,
    GROUND,
    FLYING,
    PSYCHIC,
    BUG,
    ROCK,
    GHOST,
    DRAGON,
}

#[derive(Deserialize)]
struct TypeChartJsonRow {
    #[serde(deserialize_with = "string_to_type")]
    attack: Type,

    #[serde(deserialize_with = "string_to_type")]
    defend: Type,
    effectiveness: f64,
}

fn string_to_type<'a, D>(deserializer: D) -> Result<Type, D::Error>
where 
    D: Deserializer<'a>
{
    let string = String::deserialize(deserializer)?;
    
    match string.as_str() {
        "normal" => Ok(Type::NORMAL),
        "fire" => Ok(Type::FIRE),
        "water" => Ok(Type::WATER),
        "electric" => Ok(Type::ELECTRIC),
        "grass" => Ok(Type::GRASS),
        "ice" => Ok(Type::ICE),
        "fighting" => Ok(Type::FIGHTING),
        "poison" => Ok(Type::POISON),
        "ground" => Ok(Type::GROUND),
        "flying" => Ok(Type::FLYING),
        "psychic" => Ok(Type::PSYCHIC),
        "bug" => Ok(Type::BUG),
        "rock" => Ok(Type::ROCK),
        "ghost" => Ok(Type::GHOST),
        "dragon" => Ok(Type::DRAGON),
        _ => Ok(Type::UNKNOWN),
    }
}

pub struct TypeChart {
    map: HashMap<(Type, Type), f64>,
}

impl TypeChart {
    pub fn add_relation(&mut self, attack_type: Type, defense_type: Type, effectiveness: f64) {
        self.map.insert((attack_type, defense_type), effectiveness);
    }

    pub fn get_effectiveness(&self, attack_type: &Type, defense_type: &Type) -> Option<&f64> {
        self.map.get(&(attack_type.clone(), defense_type.clone()))
    }
}

pub fn read_type_chart(file_path: String) -> Result<TypeChart, Box<dyn Error>> {
    let file_content = read_to_string(file_path)?;
    let json: Vec<TypeChartJsonRow> = serde_json::from_str(&file_content)?;

    let mut type_chart_map = HashMap::new();

    for row in json {
        type_chart_map.insert((row.attack, row.defend), row.effectiveness);
    }

    Ok(TypeChart { map: type_chart_map })
}
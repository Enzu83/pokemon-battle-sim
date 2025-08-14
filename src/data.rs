use std::error::Error;
use std::collections::HashMap;

use crate::common::pokedex::Species;
use crate::common::moves::Move;
use crate::common::types::{Type, TypeChart, read_type_chart};

pub struct Data {
    pokedex: HashMap<u32, Species>,
    moves: HashMap<u32, Move>,
    type_chart: TypeChart,
}

impl Data {
    pub fn get_species(&self, id: u32) -> Option<&Species> {
        self.pokedex.get(&id)
    }

    pub fn get_move(&self, id: u32) -> Option<&Move> {
        self.moves.get(&id)
    }

    pub fn get_type_effectiveness(&self, attack_type: &Type, defense_type: &Type) -> Option<&f64> {
        self.type_chart.get_effectiveness(attack_type, defense_type)
    }
}

pub fn setup(data_folder_path: &str) -> Result<Data, Box<dyn Error>> {
    let pokedex = read_pokedex(format!("{data_folder_path}/pokedex.json"))?;
    let moves = read_moves(format!("{data_folder_path}/moves.json"))?;
    let type_chart = read_type_chart(format!("{data_folder_path}/type_chart.json"))?;

    Ok(Data { pokedex, moves, type_chart })
}

fn read_pokedex(file_path: String) -> Result<HashMap<u32, Species>, Box<dyn Error>> {
    let pokedex = HashMap::new();

    Ok(pokedex)
}

fn read_moves(file_path: String) -> Result<HashMap<u32, Move>, Box<dyn Error>> {
    Ok(HashMap::new())
}

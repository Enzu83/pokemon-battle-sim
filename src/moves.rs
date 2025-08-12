use std::fs::read_to_string;

use crate::common::Move;

impl Move {
    pub fn new(name: &str) -> Result<Self, i32> {
        let file = read_to_string("data/moves.json")?;
        // let reader = BufReader::new(file);

        // let move_list: Vec<Move> = serde_json::from_reader(reader)?;
    }
}
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Move {
    accuracy: Option<u32>,
    category: String,
    name: String,
    power: Option<u32>,
    pp: u32,
    typ: String,
}

// impl Move {
//     pub fn new(name: &str) -> Result<Self, i32> {
//         let file = read_to_string("data/moves.json")?;
//         // let reader = BufReader::new(file);

//         // let move_list: Vec<Move> = serde_json::from_reader(reader)?;
//     }
// }
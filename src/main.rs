use std::error::Error;

use crate::common::types::Type;

pub mod common;
mod data;

fn main() -> Result<(), Box<dyn Error>> {
    let data = data::setup("data")?;

    // if let Some(charmender) = data.get_species(3) {
    //     println!("{}", charmender.name);
    // }

    let effectiveness = data.get_type_effectiveness(&Type::GROUND, &Type::FLYING).unwrap_or(&1.0);
    
    println!("Effectiveness: {}", effectiveness);

    Ok(())
}

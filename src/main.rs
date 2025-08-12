use std::error::Error;

mod pokemon;
mod moves;
mod types;
mod common;

// read data files to get pokemon, moves...
pub fn setup() -> Result<(), Box<dyn Error>> {
    setup_moves()?;

    Ok(())
}

fn setup_moves() -> Result<(), Box<dyn Error>> {
    Ok(())
}


fn main() -> Result<(), Box<dyn Error>> {
    setup()?;

    Ok(())
}

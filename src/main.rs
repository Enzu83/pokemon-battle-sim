use std::error::Error;

pub mod common;
mod data;

fn main() -> Result<(), Box<dyn Error>> {
    data::setup()?;

    Ok(())
}

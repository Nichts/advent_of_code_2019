use crate::part_1::calculate_fuel;
use std::error::Error;

mod part_1;

fn main() -> Result<(), Box<dyn Error>>{
    println!("Part 1: {}", calculate_fuel()?);
    Ok(())
}

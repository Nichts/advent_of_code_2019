use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter;

use anyhow::Result;

fn fuel_requirement(mass: u64) -> u64 {
    (mass / 3).saturating_sub(2)
}

fn full_fuel_requirement(mass: u64) -> u64 {
    let total_fuel_requirement = fuel_requirement(mass);
    let mut last_fuel_requirement = total_fuel_requirement;
    iter::from_fn(move || {
        last_fuel_requirement = fuel_requirement(last_fuel_requirement);
        match last_fuel_requirement {
            0 => None,
            _ => Some(last_fuel_requirement),
        }
    })
    .fold(total_fuel_requirement, |acc, x| acc + x)
}

fn get_modules() -> Result<Vec<u64>> {
    let file: File = File::open("input.txt")?;
    let buf_reader = BufReader::new(file);
    let iter = buf_reader.lines().map(|line| Ok(line?.parse::<u64>()?));
    let modules: Result<Vec<_>, _> = iter.collect();
    modules
}

pub(crate) fn calculate_fuel(modules: &[u64], fn_fuel: &dyn Fn(u64) -> u64) -> u64 {
    modules.iter().fold(0u64, |acc, &x| acc + fn_fuel(x))
}

fn main() -> Result<()> {
    let masses = get_modules()?;
    println!("Part 1: {}", calculate_fuel(&masses, &fuel_requirement));
    println!(
        "Part 2: {}",
        calculate_fuel(&masses, &full_fuel_requirement)
    );
    Ok(())
}

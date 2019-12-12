use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct Module {
    mass: u64,
}

fn fuel_requirement(mass: u64) -> u64 {
    (mass / 3).saturating_sub(2)
}

impl Module {
    fn fuel_requirement(&self) -> u64 {
        fuel_requirement(self.mass)
    }

    fn full_fuel_requirement(&self) -> u64 {
        let mut total_fuel_requirement = self.fuel_requirement();
        let mut last_fuel_requirement = total_fuel_requirement;
        loop {
            last_fuel_requirement = fuel_requirement(last_fuel_requirement);
            total_fuel_requirement += last_fuel_requirement;
            if last_fuel_requirement == 0 {
                return total_fuel_requirement;
            }
        }
    }

    fn new(mass: u64) -> Self {
        Self { mass }
    }
}

fn get_modules() -> Result<Vec<Module>, Box<dyn Error>> {
    let file: File = File::open("input.txt").map_err(|e| -> Box<dyn Error> { e.into() })?;
    let buf_reader = BufReader::new(file);
    let iter = buf_reader
        .lines()
        .map(|line| {
            Ok(line
                .map_err(|e| -> Box<dyn Error> { e.into() })?
                .parse::<u64>()
                .map_err(|e| -> Box<dyn Error> { e.into() })?)
        })
        .map(
            |val: Result<u64, Box<dyn Error>>| -> Result<Module, Box<dyn Error>> {
                Ok(Module::new(val?))
            },
        );
    let modules: Result<Vec<_>, _> = iter.collect();
    modules
}

pub(crate) fn calculate_fuel(modules: &Vec<Module>, fn_fuel: &dyn Fn(&Module) -> u64) -> u64 {
    modules
        .iter()
        .fold(0u64, |acc, x| acc + fn_fuel(x))
}

fn main() -> Result<(), Box<dyn Error>> {
    let modules = get_modules()?;
    println!("Part 1: {}", calculate_fuel(&modules, &Module::fuel_requirement));
    println!("Part 2: {}", calculate_fuel(&modules,&Module::full_fuel_requirement));
    Ok(())
}

use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct Module {
    mass: i32,
}

impl Module {
    fn fuel_requirement(&self) -> i32 {
        self.mass / 3 - 2
    }

    fn new(mass: i32) -> Self {
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
                .parse::<i32>()
                .map_err(|e| -> Box<dyn Error> { e.into() })?)
        })
        .map(
            |val: Result<i32, Box<dyn Error>>| -> Result<Module, Box<dyn Error>> {
                Ok(Module::new(val?))
            },
        );
    let modules: Result<Vec<_>, _> = iter.collect();
    modules
}

pub(crate) fn calculate_fuel() -> Result<i32, Box<dyn Error>> {
    let modules = get_modules()?;
    Ok(modules
        .iter()
        .fold(0i32, |acc, x| acc + x.fuel_requirement()))
}

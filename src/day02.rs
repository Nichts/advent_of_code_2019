use crate::vm::Computer;
use anyhow::{Error, Result};
use std::fs::read_to_string;

fn run(data: &[i64], noun: i64, verb: i64) -> Result<i64> {
    let mut data = data.to_owned();
    data[1] = noun;
    data[2] = verb;
    Ok(Computer::new(data).execute()?)
}

pub fn main() -> Result<()> {
    let input = read_to_string("data/day02.txt")?;
    let data = input
        .trim()
        .split(',')
        .map(|val| val.parse::<i64>().map_err(Error::from))
        .collect::<Result<Vec<_>>>()?;
    println!("Part 1: {}", run(&data, 12, 2)?);
    for noun in 0..99 {
        for verb in 0..99 {
            match run(&data, noun, verb) {
                Ok(result) if result == 19_690_720 => {
                    println!("Part 2: {}", 100 * noun + verb);
                    break;
                }
                _ => (),
            };
        }
    }
    Ok(())
}

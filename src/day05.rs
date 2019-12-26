use crate::vm::errors::Error;
use crate::vm::types::Value;
use crate::vm::Computer;
use anyhow::{anyhow, Result};
use std::fs::read_to_string;

fn run(data: &[i64], input: i64) -> Result<Value> {
    let data = data.to_owned();
    let mut out: Vec<Value> = Vec::new();
    let mut input = Some(input);
    let mut read = || input.take().ok_or(Error::ReadingNotSupported);
    let mut write = |value| {
        out.push(value);
        Ok(())
    };
    let mut vm = Computer::new(data);
    vm.run(&mut read, &mut write)?;
    out.iter()
        .fold(Ok(None), |acc, &val| {
            if val == 0 {
                match acc {
                    Ok(None) => Ok(None),
                    _ => Err(anyhow!("Invalid value")),
                }
            } else {
                match acc {
                    Ok(None) => Ok(Some(val)),
                    _ => Err(anyhow!("Invalid value")),
                }
            }
        })?
        .ok_or_else(|| anyhow!("No value"))
}

pub fn main() -> Result<()> {
    let input = read_to_string("data/day05.txt")?;
    let data = input
        .trim()
        .split(',')
        .map(|val| val.parse::<i64>().map_err(::anyhow::Error::from))
        .collect::<Result<Vec<_>>>()?;
    let res = run(&data, 1)?;
    println!("Part 1: {}", res);
    let res = run(&data, 5)?;
    println!("Part 2: {}", res);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() -> Result<()> {
        main()
    }
}

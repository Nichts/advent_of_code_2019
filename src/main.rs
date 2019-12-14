mod day01;
mod day02;
mod day03;
mod vm;

use anyhow::Result;

fn main() -> Result<()> {
    day01::main()?;
    day02::main()?;
    Ok(())
}

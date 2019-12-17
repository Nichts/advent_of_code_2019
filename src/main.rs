mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod vm;

use anyhow::Result;

macro_rules! days {
    ( $($day:ident),* ) => {
        fn main() -> Result<()> {
            $($day::main()?;)*
            Ok(())
        }
    }
}

days! {day01, day02, day03, day04, day05}

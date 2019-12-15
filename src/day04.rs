use anyhow::Result;
use regex::Regex;
use std::fs::read_to_string;

fn validate(num: u32) -> Option<u32> {
    if !(num >= 100_000 && num <= 999_999) {
        return None;
    }
    let mut last = num % 10;
    let mut num = num;
    let mut curr_cluster = 1;
    let mut shortest_cluster = None;
    while num > 0 {
        num /= 10;
        let curr = num % 10;
        if curr > last {
            return None;
        } else if curr == last {
            curr_cluster += 1;
        } else {
            if curr_cluster > 1 {
                match shortest_cluster {
                    None => shortest_cluster = Some(curr_cluster),
                    Some(sc) if curr_cluster < sc => shortest_cluster = Some(curr_cluster),
                    _ => (),
                };
            };
            curr_cluster = 1;
        }

        last = curr;
    }
    Some(shortest_cluster.unwrap_or(1))
}

pub fn main() -> Result<()> {
    let input = read_to_string("data/day04.txt")?;
    let matcher = Regex::new(r"^(\d{6})-(\d{6})$")?;
    let captures = matcher.captures(input.trim()).unwrap();
    let low = captures.get(1).unwrap().as_str().parse()?;
    let high = captures.get(2).unwrap().as_str().parse()?;
    let count = (low..=high)
        .filter(|num| {
            validate(*num)
                .and_then(|val| if val > 1 { Some(val) } else { None })
                .is_some()
        })
        .count();
    println!("Part 1: {}", count);
    let count = (low..=high)
        .filter(|num| {
            validate(*num)
                .and_then(|val| if val == 2 { Some(val) } else { None })
                .is_some()
        })
        .count();
    println!("Part 2: {}", count);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate() {
        assert_eq!(validate(122456).unwrap(), 2);
        assert_eq!(validate(123456).unwrap(), 1);

        assert_eq!(validate(111111).unwrap(), 6);
        assert!(validate(223450).is_none());
        assert_eq!(validate(123789).unwrap(), 1);
        assert!(validate(359288).is_none());
        assert_eq!(validate(111122).unwrap(), 2);
    }

    #[test]
    fn test_main() -> Result<()> {
        main()
    }
}

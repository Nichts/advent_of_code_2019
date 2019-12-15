use anyhow::Result;
use nalgebra::{Point2, Vector2};
use regex::Regex;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::fs::read_to_string;
use thiserror::Error;

use lazy_static::lazy_static;

type Value = i64;
type Point = Point2<Value>;
type Vector = Vector2<Value>;

#[derive(Clone, Error, Debug, PartialEq)]
pub enum Error {
    #[error("Invalid segment {0}")]
    InvalidSegment(String),
    #[error("Non intersections found")]
    NoIntersections,
}

#[derive(Debug, PartialEq)]
struct Segment {
    direction: Direction,
    length: Value,
}

struct StepIter<'a> {
    segment: &'a Segment,
    steps: Value,
}

impl<'a> StepIter<'a> {
    fn new(segment: &'a Segment) -> Self {
        Self { segment, steps: 0 }
    }
}

impl Segment {
    fn iter_steps(&self) -> StepIter {
        StepIter::new(&self)
    }
}
lazy_static! {
    static ref MATCHER: Regex = Regex::new(r"^(\w)(\d+)$").unwrap();
}

impl TryFrom<&str> for Segment {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let captures = MATCHER
            .captures(value)
            .ok_or_else(|| Error::InvalidSegment(value.to_owned()))?;
        let direction = match &captures[1] {
            "R" => Direction::Right,
            "L" => Direction::Left,
            "U" => Direction::Up,
            "D" => Direction::Down,
            _ => return Err(Error::InvalidSegment(value.to_owned())),
        };
        let length: Value = captures[2]
            .parse()
            .map_err(|_| Error::InvalidSegment(value.to_owned()))?;
        Ok(Self { direction, length })
    }
}

impl<'a> Iterator for StepIter<'a> {
    type Item = Vector;

    fn next(&mut self) -> Option<Self::Item> {
        if self.steps >= self.segment.length {
            None
        } else {
            self.steps += 1;
            Some(self.segment.direction.as_offset(1))
        }
    }
}

#[derive(Debug, PartialEq)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn as_offset(&self, length: Value) -> Vector {
        match self {
            Direction::Left => Vector::new(-length, 0),
            Direction::Right => Vector::new(length, 0),
            Direction::Up => Vector::new(0, length),
            Direction::Down => Vector::new(0, -length),
        }
    }
}

struct Wire {
    start: Point,
    segments: Vec<Segment>,
}

impl Wire {
    pub fn from_str(start: Point, segments: &[&str]) -> Result<Self, Error> {
        Ok(Self {
            start,
            segments: segments
                .iter()
                .map(|&seg| Segment::try_from(seg))
                .collect::<Result<Vec<_>, _>>()?,
        })
    }

    pub fn iter_points<'a>(&'a self) -> impl Iterator<Item = Point> + 'a {
        let mut curr = self.start;
        self.segments
            .iter()
            .flat_map(|segment| segment.iter_steps())
            .map(move |offset| {
                curr += offset;
                curr
            })
    }

    pub fn intersections<'a>(
        &'a self,
        other: &'a Wire,
    ) -> impl Iterator<Item = (Point, usize)> + 'a {
        let mut points: HashMap<Point, usize> = HashMap::new();
        self.iter_points().enumerate().for_each(|(dist, point)| {
            points
                .entry(point)
                .and_modify(|curr_dist| {
                    if *curr_dist > dist {
                        *curr_dist = dist
                    }
                })
                .or_insert(dist);
        });
        other
            .iter_points()
            .enumerate()
            .filter_map(move |(curr_dist, point)| {
                points
                    .get(&point)
                    .map(|dist| (point, curr_dist + *dist + 2))
            })
    }
}

pub fn main() -> Result<()> {
    let input = read_to_string("data/day03.txt")?;
    let lines = input.lines();
    let data = lines
        .map(|line| {
            let segments: Vec<&str> = line.split(',').collect();
            Wire::from_str(Point::new(0, 0), &segments).map_err(::anyhow::Error::from)
        })
        .collect::<Result<Vec<_>>>()?;
    assert_eq!(data.len(), 2);
    let intersections = data[0].intersections(&data[1]).collect::<Vec<_>>();
    let closest = intersections
        .iter()
        .fold(None, |acc, val| match acc {
            None => Some(val.0),
            Some(curr) => {
                if val.0.x.abs() + val.0.y.abs() < curr.x.abs() + curr.y.abs() {
                    Some(val.0)
                } else {
                    Some(curr)
                }
            }
        })
        .ok_or_else(|| ::anyhow::Error::from(Error::NoIntersections))?;
    println!("Part 1: {}", closest.x.abs() + closest.y.abs());
    let shortest = intersections
        .iter()
        .map(|(_, dist)| dist)
        .min()
        .ok_or_else(|| ::anyhow::Error::from(Error::NoIntersections))?;
    println!("Part 2: {}", shortest);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    impl Segment {
        fn new(direction: Direction, length: Value) -> Self {
            Self { direction, length }
        }
    }

    impl Wire {
        fn new(start: Point, segments: Vec<Segment>) -> Self {
            Self { start, segments }
        }
    }

    #[test]
    fn test_iters() {
        let wire = Wire::new(
            Point::new(1, 2),
            vec![
                Segment::new(Direction::Right, 3),
                Segment::new(Direction::Up, 2),
                Segment::new(Direction::Down, 1),
                Segment::new(Direction::Left, 2),
            ],
        );
        let expected = vec![
            Point::new(2, 2),
            Point::new(3, 2),
            Point::new(4, 2),
            Point::new(4, 3),
            Point::new(4, 4),
            Point::new(4, 3),
            Point::new(3, 3),
            Point::new(2, 3),
        ];
        assert_eq!(wire.iter_points().collect::<Vec<Point>>(), expected);
    }

    #[test]
    fn test_intersections() {
        let wire_a = Wire::new(
            Point::new(1, 1),
            vec![
                Segment::new(Direction::Right, 3),
                Segment::new(Direction::Up, 2),
            ],
        );
        let wire_b = Wire::new(
            Point::new(1, 1),
            vec![
                Segment::new(Direction::Up, 1),
                Segment::new(Direction::Right, 4),
            ],
        );
        let expected = vec![(Point::new(4, 2), 8)];
        assert_eq!(
            wire_a
                .intersections(&wire_b)
                .collect::<Vec<(Point, usize)>>(),
            expected
        );
    }

    #[test]
    fn test_from() {
        assert_eq!(
            Segment::try_from("R255").unwrap(),
            Segment::new(Direction::Right, 255)
        );
    }

    #[test]
    fn test_main() -> Result<()> {
        main()
    }
}

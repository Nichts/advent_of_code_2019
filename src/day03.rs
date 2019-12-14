use nalgebra::{Point2, Vector2};
use std::collections::HashSet;
use std::fs::read_to_string;
use anyhow::Result;
use thiserror::Error;
use std::convert::TryFrom;
use regex::Regex;

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
    fn new(direction: Direction, length: Value) -> Self {
        Self { direction, length }
    }
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
        let captures = MATCHER.captures(value).ok_or(Error::InvalidSegment(value.to_owned()))?;
        let direction = match &captures[1] {
            "R" => Direction::Right,
            "L" => Direction::Left,
            "U" => Direction::Up,
            "D" => Direction::Down,
            _ => Err(Error::InvalidSegment(value.to_owned()))?
        };
        let length: Value = captures[2].parse().map_err(|_| Error::InvalidSegment(value.to_owned()))?;
        Ok(Self {
            direction,
            length,
        })
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

    fn apply(&self, target: &mut Point) {
        let right = Vector2::new(1, 0);
        let up = Vector2::new(0, 1);
        match self {
            Direction::Left => *target -= right,
            Direction::Right => *target += right,
            Direction::Up => *target += up,
            Direction::Down => *target -= up,
        };
    }
}

struct Wire {
    start: Point,
    segments: Vec<Segment>,
}

impl Wire {
    pub fn new(start: Point, segments: Vec<Segment>) -> Self {
        Self { start, segments }
    }

    pub fn from_str(start: Point, segments: &[&str]) -> Result<Self, Error> {
        Ok(Self {
            start,
            segments: segments.iter().map(|&seg| Segment::try_from(seg)).collect::<Result<Vec<_>, _>>()?,
        })
    }

    pub fn iter_points<'a>(&'a self) -> impl Iterator<Item=Point> + 'a {
        let mut curr = self.start.clone();
        self.segments
            .iter()
            .flat_map(|segment| segment.iter_steps())
            .map(move |offset| {
                curr += offset;
                curr
            })
    }

    pub fn intersections<'a>(&'a self, other: &'a Wire) -> impl Iterator<Item=Point> + 'a {
        let points: HashSet<Point> = self.iter_points().collect();
        other.iter_points().filter_map(move |point| {
            if points.contains(&point) {
                Some(point)
            } else {
                None
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_directions() {
        let mut point = Point::new(1, 1);
        Direction::Left.apply(&mut point);
        assert_eq!(point, Point::new(0, 1));
        Direction::Right.apply(&mut point);
        Direction::Up.apply(&mut point);
        assert_eq!(point, Point::new(1, 2));
        Direction::Down.apply(&mut point);
        assert_eq!(point, Point::new(1, 1));
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
        let expected = vec![Point::new(4, 2)];
        assert_eq!(
            wire_a.intersections(&wire_b).collect::<Vec<Point>>(),
            expected
        );
    }

    #[test]
    fn test_from() {
        assert_eq!(Segment::try_from("R255").unwrap(), Segment::new(Direction::Right, 255));
    }

    #[test]
    fn test_main() {
        main();
    }
}

fn main() -> Result<()> {
    let input = read_to_string("data/day03.txt")?;
    let lines = input.lines();
    let mut data = lines
        .map(|line| {
            let segments: Vec<&str> = line.split(',').collect();
            Wire::from_str(Point::new(0, 0), &segments).map_err(::anyhow::Error::from)
        }).collect::<Result<Vec<_>>>()?;
    assert_eq!(data.len(), 2);
    let closest = data[0].intersections(&data[1]).fold(None, |acc, val|
        match acc {
            None => dbg!(Some(val)),
            Some(curr) => if val.x.abs() + val.y.abs() < curr.x.abs() + curr.y.abs() {
                Some(val)
            } else {
                Some(curr)
            }
        },
    ).ok_or(::anyhow::Error::from(Error::NoIntersections))?;
    println!("Part 1: {}", closest.x.abs() + closest.y.abs());
    Ok(())
}

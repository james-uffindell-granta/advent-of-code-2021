use std::{cmp::{max, min}, collections::HashMap};
use scan_fmt::scan_fmt;

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub struct Coord {
    x: u32,
    y: u32,
}

impl Coord {
    pub fn in_line_with(&self, other: &Coord) -> bool {
        self.x == other.x || self.y == other.y
    }

    pub fn points_to(&self, other: &Coord) -> Vec<Coord> {
        if self.x == other.x {
            (min(self.y, other.y)..=max(self.y, other.y)).map(|y| Coord { x: self.x, y }).collect()
        } else if self.y == other.y {
            (min(self.x, other.x)..=max(self.x, other.x)).map(|x| Coord { x, y: self.y }).collect()
        } else {
            // must be diagonal
            let x_diff = (self.x as i32) - (other.x as i32);
            let y_diff = (self.y as i32) - (other.y as i32);
            assert!(x_diff.abs() == y_diff.abs());

            let x_increment = x_diff / x_diff.abs();
            let y_increment = y_diff / y_diff.abs();

            (0..=x_diff.abs()).map(|i| Coord { x: (self.x as i32 - i * x_increment) as u32, y: (self.y as i32 - i * y_increment) as u32 }).collect()
        }
    }
}

#[derive(Copy, Clone)]
pub struct Line {
    start: Coord,
    end: Coord,
}

impl Line {
    pub fn points(&self) -> Vec<Coord> {
        self.start.points_to(&self.end)
    }
}

pub fn parse_input(input: &str) -> Vec<Line> {
    input.lines().map(|l| {
        let (x1, y1, x2, y2) = scan_fmt!(l, "{},{} -> {},{}", u32, u32, u32, u32).unwrap();
        Line { start: Coord { x: x1, y: y1 }, end: Coord { x: x2, y:y2 }}
    }).collect()
}

pub fn resolve_lines<T : Iterator<Item = Line>>(input: T) -> HashMap<Coord, usize> {
    let mut map = HashMap::new();

    for line in input {
        for p in line.points() {
            *map.entry(p).or_insert(0) += 1;
        }
    }
    map
}

pub fn part_1(input: &[Line]) -> usize {
    let map = resolve_lines(input.iter().copied().filter(|l| l.start.in_line_with(&l.end)));
    map.iter().filter(|(_, &count)| count >= 2).count()
}

pub fn part_2(input: &[Line]) -> usize {
    let map = resolve_lines(input.iter().copied());
    map.iter().filter(|(_, &count)| count >= 2).count()
}

fn main() {
    let input = parse_input(include_str!("../input.txt"));
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}


#[test]
pub fn test_diagonals() {
    let l1 = Line { start: Coord { x: 1, y: 1 }, end: Coord { x: 3, y: 3 } };
    assert_eq!(l1.points(), vec![ Coord { x: 1, y: 1}, Coord { x: 2, y: 2 }, Coord { x: 3, y: 3 }]);

    let l2 = Line { start: Coord { x: 9, y: 7 }, end: Coord { x: 7, y: 9 } };
    assert_eq!(l2.points(), vec![ Coord { x: 9, y: 7}, Coord { x: 8, y: 8}, Coord { x: 7, y: 9 }])
}
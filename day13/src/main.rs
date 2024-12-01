use std::collections::HashSet;

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub struct Coord {
    x: i64,
    y: i64
}

impl From<(i64, i64)> for Coord {
    fn from(value: (i64, i64)) -> Self {
        Self { x: value.0, y: value.1 }
    }
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum FoldType { Horizontal, Vertical }

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub struct Fold {
    fold_type: FoldType,
    location: i64,
}

#[derive(Debug, Clone)]
pub struct Input {
    dots: HashSet<Coord>,
    instructions: Vec<Fold>
}

impl std::fmt::Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let max_x = self.dots.iter().map(|c| c.x).max().unwrap();
        let max_y = self.dots.iter().map(|c| c.y).max().unwrap();

        for y in 0 ..= max_y {
            for x in 0 ..= max_x {
                let c = (x, y).into();
                if self.dots.contains(&c) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl Input {
    // refactor this a bit later maybe
    pub fn follow_fold(&self, fold: Fold) -> Input {
        let new_dots = self.dots.iter().map(|d| 
            match fold.fold_type {
                FoldType::Horizontal => {
                    if d.y < fold.location { *d } else { (d.x, fold.location + fold.location - d.y).into() }
                },
                FoldType::Vertical => {
                    if d.x < fold.location { *d } else { (fold.location + fold.location - d.x, d.y).into() }
                },
            }).collect();

        Input { dots: new_dots, ..self.clone() }
    }

    pub fn fold(&self) -> Input {
        let mut data = self.clone();
        for instruction in &self.instructions {
            data = data.follow_fold(*instruction);
        }

        data
    }
}

pub fn parse_input(input: &str) -> Input {
    let mut dots: HashSet<Coord> = HashSet::new();
    let mut instructions = Vec::new();
    let (dot_portion, instruction_portion) = input.split_once("\n\n").unwrap();
    for dot in dot_portion.lines() {
        let (x, y) = dot.split_once(',').unwrap();
        dots.insert((x.parse().unwrap(), y.parse().unwrap()).into());
    }

    for instruction in instruction_portion.lines() {
        let fold = instruction.replace("fold along ", "");
        let (direction, location) = fold.split_once('=').unwrap();
        if direction == "y" {
            instructions.push(Fold { fold_type: FoldType::Horizontal, location: location.parse().unwrap() })
        } else if direction == "x" {
            instructions.push(Fold { fold_type: FoldType::Vertical, location: location.parse().unwrap() })
        } else {
            unreachable!();
        }
    }

    Input { dots, instructions }
}

fn main() {
    let input = include_str!("../input.txt");
    let input = parse_input(input);
    println!("Part 1: {}", input.follow_fold(input.instructions[0]).dots.len());
    println!("Part 2:");
    println!("{}", input.fold());
}

#[test]
pub fn test() {
    let input = r"6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";

    let data = parse_input(input);
    println!("{}", data);
    let folded = data.follow_fold(data.instructions[0]);
    println!("{}", folded);
    assert_eq!(folded.dots.len(), 17);
}

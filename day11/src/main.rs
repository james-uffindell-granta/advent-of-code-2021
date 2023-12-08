use std::collections::{HashMap, HashSet};

#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug)]
pub struct Octopus { x: i8, y: i8 }

impl Octopus {
    pub fn neighbours(&self) -> HashSet<Octopus> {
        vec![
            Self { x: self.x, y: self.y - 1 },
            Self { x: self.x, y: self.y + 1 },
            Self { x: self.x - 1, y: self.y },
            Self { x: self.x + 1, y: self.y },
            Self { x: self.x - 1, y: self.y - 1 },
            Self { x: self.x - 1, y: self.y + 1 },
            Self { x: self.x + 1, y: self.y - 1 },
            Self { x: self.x + 1, y: self.y + 1 },
        ].into_iter().collect()
    }
}

impl From<(i8, i8)> for Octopus {
    fn from(value: (i8, i8)) -> Self {
        Self { x: value.0, y: value.1 }
    }
}

#[derive(Clone)]
pub struct Grid {
    grid: HashMap<Octopus, u32>
}

impl Grid {
    fn ready_to_flash(&self) -> HashSet<Octopus> {
        self.grid.iter()
            .filter(|(o, &e)| e > 9)
            .map(|(o, _)| *o)
            .collect::<HashSet<_>>()
    }

    pub fn step(&mut self) -> usize {
        let mut flashed_this_step = HashSet::new();
        // first, the energy level of each octopus increases by 1
        for (_, energy) in self.grid.iter_mut() {
            *energy += 1;
        }

        let mut ready_to_flash = self.ready_to_flash();

        // then, repeat this until we run out of octopuses that can flash
        while !ready_to_flash.is_empty() {
            for octopus in ready_to_flash {
                for neighbour in octopus.neighbours() {
                    if let Some(energy) = self.grid.get_mut(&neighbour) {
                        *energy += 1;
                    }
                }

                // then remember that this octopus flashed
                flashed_this_step.insert(octopus);
            }

            ready_to_flash = self.ready_to_flash().difference(&flashed_this_step).copied().collect();
        }

        // then, set the energy of any octopus that flashed this step to 0
        for o in flashed_this_step.iter() {
            self.grid.entry(*o).and_modify(|e| *e = 0);
        }

        flashed_this_step.len()
    }

pub fn print(&self) {
    for y in 0..10 {
        for x in 0..10 {
            let octopus = (x, y).into();
                match self.grid.get(&octopus) {
                    Some(e) => print!("{}", e),
                    None => unreachable!(),
                }
        }

        println!();
    }

    println!();
}

}

pub fn parse_input(input: &str) -> Grid {
    let mut map = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let value = c.to_digit(10).unwrap();
            let coord = (x as i8, y as i8).into();
            map.insert(coord, value);
        }
    }

    Grid { grid: map }
}

pub fn part_1(grid: &Grid) -> usize {
    let mut grid = grid.clone();
    let mut flashes = 0;
    for _ in 0..100 {
        flashes += grid.step();
    }

    flashes
}

pub fn part_2(grid: &Grid) -> usize {
    let mut grid = grid.clone();
    for step in 1.. {
        if grid.step() == 100 {
            return step;
        }
    }

    unreachable!()
}

fn main() {
    let grid = parse_input(include_str!("../input.txt"));
    println!("Part 1: {}", part_1(&grid));
    println!("Part 2: {}", part_2(&grid));
}

#[test]
pub fn test_evolve() {
    let input = r"5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

    let mut grid = parse_input(input);
    let mut flashes = 0;
    for _ in 0..10 {
        flashes += grid.step();
        // grid.print();
    }

    for _ in 0..90 {
        flashes += grid.step();
    }

    assert_eq!(flashes, 1656);

    assert_eq!(part_2(&parse_input(input)), 195);
}

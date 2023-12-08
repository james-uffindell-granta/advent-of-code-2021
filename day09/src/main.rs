use std::collections::{HashMap, HashSet};

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
pub struct Coord { x: i32, y: i32 }

impl Coord {
    pub fn neighbours(&self) -> HashSet<Coord> {
        vec![
            Self { x: self.x, y: self.y - 1 },
            Self { x: self.x, y: self.y + 1 },
            Self { x: self.x - 1, y: self.y },
            Self { x: self.x + 1, y: self.y }
        ].into_iter().collect()
    }
}

impl From<(i32, i32)> for Coord {
    fn from(value: (i32, i32)) -> Self {
        Self { x: value.0, y: value.1 }
    }
}

pub fn parse_input(input: &str) -> HashMap<Coord, u32> {
    let mut map = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let value: u32 = c.to_digit(10).unwrap();
            let coord = (x as i32, y as i32).into();
            map.insert(coord, value);
        }
    }

    map
}

pub fn part_1(input: &HashMap<Coord, u32>) -> u32 {
    let mut low_points = Vec::new();
    for (c, height) in input {
        if c.neighbours().into_iter().filter_map(|n| input.get(&n)).all(|h| h > height) {
            low_points.push(1 + height);
        }
    }

    low_points.into_iter().sum()
}

// pub fn print_state(max_x: i32, max_y: i32, basin_mappings: &HashMap<Coord, u32>, basins: &HashMap<u32, Vec<u32>>) {
//     for y in 0..=max_y {
//         for x in 0..=max_x {
//             let c = (x, y).into();
//                 match basin_mappings.get(&c) {
//                     Some(b) => print!("{}", b),
//                     None => print!("."),
//                 }
//         }

//         println!();
//     }

//     println!();
// }

pub fn calculate_basins(input: &HashMap<Coord, u32>) -> HashMap<u32, Vec<u32>> {
    let max_x = input.keys().map(|c| c.x).max().unwrap();
    let max_y = input.keys().map(|c| c.y).max().unwrap();
    // basin number -> collection of heights in the basin
    let mut basins = HashMap::new();
    // coordinate to current basin number
    let mut basin_mappings = HashMap::new();
    let mut next_basin: u32 = 1;
    for (c, height) in input {
        if *height == 9 {
            // we can't use this to join basins - carry on
            continue;
        }

        let existing_neighbour_basins = c.neighbours().into_iter()
            .filter_map(|n| basin_mappings.get(&n))
            .copied()
            .collect::<Vec<_>>();
        match existing_neighbour_basins.as_slice() {
            [] => {
                // no neighbours already in a basin - so this is a new basin
                basin_mappings.insert(*c, next_basin);
                basins.entry(next_basin).or_insert(Vec::new()).push(*height);
                // and now the next new basin needs a new number
                next_basin += 1;
            },
            [b] => {
                // all our neighbours are already in the same basin - join them
                basin_mappings.insert(*c, *b);
                basins.entry(*b).or_insert(Vec::new()).push(*height);
            },
            _ => {
                // we have multiple neighbours in basins, but different basins - merge them
                // pick the lowest basin number to be the new basin
                let mut basins_to_work_with = existing_neighbour_basins.clone();
                basins_to_work_with.sort_unstable();
                let (first, rest) = basins_to_work_with.split_first().unwrap();
                // pick the smallest basin as the new consolidated basin
                let consolidated_basin = *first;
                // make sure to remove the one we're consolidating too - it may be in there many times
                let other_basins = rest.iter().filter(|&b| b != &consolidated_basin).collect::<HashSet<_>>();
                // put the current coord in that basin
                basin_mappings.insert(*c, consolidated_basin);
                basins.entry(consolidated_basin).or_insert(Vec::new()).push(*height);
                // now go through all the other basins and move the coords over
                let coords_in_other_basins: Vec<_> = basin_mappings.clone().into_iter().filter(|(_, b)| other_basins.contains(b)).collect();
                for (c, _) in coords_in_other_basins {
                    basin_mappings.insert(c, consolidated_basin);
                    basins.entry(consolidated_basin).or_insert(Vec::new()).push(*input.get(&c).unwrap());
                }
                // now remove our knowledge of the other basins
                for b in other_basins {
                    basins.remove(b);
                }
            }
        }

    }

    basins
}

pub fn part_2(input: &HashMap<Coord, u32>) -> usize {
    let basins = calculate_basins(input);
    let mut sizes = basins.values().map(|v| v.len()).collect::<Vec<_>>();
    sizes.sort_unstable();
    sizes.into_iter().rev().take(3).product::<usize>()
}

fn main() {
    let depth_map = parse_input(include_str!("../input.txt"));
    println!("Part 1: {}", part_1(&depth_map));
    println!("Part 2: {}", part_2(&depth_map));
}

#[test]
pub fn test() {
    let input = r"2199943210
3987894921
9856789892
8767896789
9899965678";

    let input = parse_input(input);
    let risk = part_1(&input);
    let basins = calculate_basins(&input);
    assert_eq!(risk, 15);
}

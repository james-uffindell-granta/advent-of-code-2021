use std::{cmp::Reverse, collections::{BTreeMap, BinaryHeap, HashMap, HashSet}};

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Coord {
    x: i64,
    y: i64
}

impl From<(i64, i64)> for Coord {
    fn from(value: (i64, i64)) -> Self {
        Self { x: value.0, y: value.1 }
    }
}

impl Coord {
    pub fn neighbours(self) -> HashSet<Coord> {
        let Coord { x, y }= self;
        HashSet::from([
            (x - 1, y).into(),
            (x + 1, y).into(),
            (x, y - 1).into(),
            (x, y + 1).into(),
        ])
    }
}

#[derive(Copy, Clone)]
pub struct Risk(u32);

#[derive(Clone)]
pub struct Cavern {
    cells: HashMap<Coord, Risk>,
    end: Coord,
}

impl Cavern {
    pub fn to_expanded(&self) -> Cavern {
        let mut new_cells = HashMap::new();
        for row in 0i64..=4 {
            for column in 0i64..=4 {
                for (Coord { x, y }, Risk(risk)) in &self.cells {
                    let mut new_risk = *risk as i64 + row + column;
                    if new_risk > 9 {
                        new_risk -= 9;
                    }
                    new_cells.insert((x + column * (self.end.x + 1), y + row * (self.end.y + 1)).into(), Risk(new_risk as u32));
                }
            }
        }

        let new_end = (5 * (self.end.x + 1) - 1, 5 * (self.end.y + 1) - 1).into();

        Cavern { cells: new_cells, end: new_end }
    }

    pub fn lowest_risks(&self) -> HashMap<Coord, u32> {
        let mut risks = HashMap::new();
        let mut unvisited = self.cells.keys().copied().collect::<HashSet<_>>();
        // map from coordinate to lowest known risk so far
        let mut lowest_risks = HashMap::<Coord, _>::new();
        let start: Coord = (0, 0).into();

        let mut frontier = BinaryHeap::new();
        frontier.push(Reverse((0, start)));

        // distance to the start is 0
        lowest_risks.insert(start, 0);

        while let Some((distance, cell)) = frontier.pop().map(|Reverse(n)| n) {
            if risks.contains_key(&cell) {
                // already did this one
                continue;
            }

            let neighbours = cell.neighbours();

            let unvisited_neighbours = neighbours.intersection(&unvisited);
            for neighbour in unvisited_neighbours {
                let distance_to_neighbour_this_way = distance + self.cells.get(neighbour).unwrap().0;
                match lowest_risks.get(neighbour) {
                    Some(existing_distance) => {
                        if existing_distance > &distance_to_neighbour_this_way {
                            let minimum_distance = existing_distance.min(&distance_to_neighbour_this_way);
                            frontier.push(Reverse((*minimum_distance, *neighbour)));
                            lowest_risks.insert(*neighbour, *minimum_distance);
                        }
                        // otherwise the existing minimum distance is still the min
                        // no need to adjust anything
                    },
                    None => {
                        lowest_risks.insert(*neighbour, distance_to_neighbour_this_way);
                        frontier.push(Reverse((distance_to_neighbour_this_way, *neighbour)));

                    }
                }
            }

            unvisited.remove(&cell);
            risks.insert(cell, distance);
        }

        risks
    }
}

pub fn parse_input(input: &str) -> Cavern {
    let mut cells = HashMap::new();
    let mut end = (0, 0).into();

    for (y, line) in input.lines().enumerate() {
        for (x, cell) in line.chars().enumerate() {
            let current_coord = (x as i64, y as i64).into();
            let risk = cell.to_digit(10).unwrap();
            cells.insert(current_coord, Risk(risk));
            end = current_coord;
        }
    }

    Cavern { cells, end }
}

pub fn part_1(cavern: &Cavern) -> u32 {
    let risks = cavern.lowest_risks();
    *risks.get(&cavern.end).unwrap()
}

pub fn part_2(cavern: &Cavern) -> u32 {
    let bigger_cavern = cavern.to_expanded();
    part_1(&bigger_cavern)
}

fn main() {
    let input = include_str!("../input.txt");
    let cavern = parse_input(input);
    println!("Part 1: {}", part_1(&cavern));
    println!("Part 2: {}", part_2(&cavern));
}

#[test]
pub fn test() {
    let input = r#"1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581
"#;

    let cavern = parse_input(input);
    assert_eq!(part_1(&cavern), 40);
    assert_eq!(part_2(&cavern), 315);
}
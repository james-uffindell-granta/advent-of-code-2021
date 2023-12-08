use std::num::ParseIntError;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Measurement(i32);

impl TryFrom<&str> for Measurement {
    type Error = ParseIntError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(Measurement(value.parse()?))
    }
}

pub fn parse_input(input: &str) -> Vec<Measurement> {
    input.lines().map(|l| l.try_into().unwrap()).collect()
}

pub fn part1(measurements: &[Measurement]) -> usize {
    measurements
        .windows(2)
        .filter(|w| w[0] < w[1])
        .count()
}

pub fn part2(measurements: &[Measurement]) -> usize {
    let summed_windows = measurements
        .windows(3)
        .map(|w| w.iter().map(|m| m.0).sum())
        .collect::<Vec<i32>>();

    summed_windows
        .windows(2)
        .filter(|w| w[0] < w[1])
        .count()
}


fn main() {
    let input = include_str!("../input.txt");
    let measurements = parse_input(input);

    println!("Part 1: {}", part1(&measurements));
    println!("Part 2: {}", part2(&measurements));
}

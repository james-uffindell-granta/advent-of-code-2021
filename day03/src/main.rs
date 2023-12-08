use std::num::ParseIntError;

use thiserror::Error;

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum Bit {
    Zero,
    One
}

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Failed to parse char {0}")]
    InvalidChar(char)
}

impl TryFrom<char> for Bit {
    type Error = ParseError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '0' => Ok(Self::Zero),
            '1' => Ok(Self::One),
            c => Err(Self::Error::InvalidChar(c))
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct Entry(Vec<Bit>);

impl std::fmt::Display for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let as_string: String = self.0.iter().map(|b| match b {
            Bit::Zero => '0',
            Bit::One => '1',
        }).collect();

        write!(f, "{}", as_string)?;

        Ok(())
    }
}

impl TryFrom<&str> for Entry {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(Entry(value.chars().map(Bit::try_from).collect::<Result<_, _>>()?))
    }
}

pub fn parse_input(input: &str) -> Result<Vec<Entry>, ParseError> {
    input.lines().map(Entry::try_from).collect::<Result<_, _>>()
}

pub fn part1(entries: &[Entry]) -> Result<u32, ParseIntError> {
    let entry_length = entries[0].0.len();
    let mut gamma_rate = Vec::new();
    let mut epsilon_rate = Vec::new();
    for index in 0..entry_length {
        match most_common_value_in_index(entries, index) {
            Some(Bit::Zero) => {
                gamma_rate.push(Bit::Zero);
                epsilon_rate.push(Bit::One);
            },
            Some(Bit::One) => {
                gamma_rate.push(Bit::One);
                epsilon_rate.push(Bit::Zero);
            },
            None => unreachable!(),
        }
    }

    let gamma_rate = u32::from_str_radix(&Entry(gamma_rate).to_string(), 2)?;
    let epsilon_rate = u32::from_str_radix(&(Entry(epsilon_rate).to_string()), 2)?;

    Ok(gamma_rate * epsilon_rate)
}

pub fn most_common_value_in_index(entries: &[Entry], index: usize) -> Option<Bit> {
    let (zeros, ones) : (Vec<_>, Vec<_>) = entries.iter().map(|e| e.0[index]).partition(|b| *b == Bit::Zero);
    match zeros.len().cmp(&ones.len()) {
        std::cmp::Ordering::Less => Some(Bit::One),
        std::cmp::Ordering::Equal => None,
        std::cmp::Ordering::Greater => Some(Bit::Zero)
    }
}

pub fn part2(entries: &[Entry]) -> Result<u32, ParseIntError> {
    let entry_length = entries[0].0.len();
    let mut oxygen_generator_candidates = entries.to_vec();
    let mut co2_scrubber_candidates = entries.to_vec();
    let mut oxygen_generator_rating = Entry::default();
    let mut co2_scrubber_rating = Entry::default();
    for index in 0..entry_length {
        match most_common_value_in_index(&oxygen_generator_candidates, index) {
            Some(Bit::Zero) => {
                oxygen_generator_candidates.retain(|e| e.0[index] == Bit::Zero);
            },
            _ => {
                oxygen_generator_candidates.retain(|e| e.0[index] == Bit::One);
            }
        }

        if oxygen_generator_candidates.len() == 1 {
            oxygen_generator_rating = oxygen_generator_candidates[0].clone();
            break;
        }
    }

    for index in 0..entry_length {
        match most_common_value_in_index(&co2_scrubber_candidates, index) {
            Some(Bit::Zero) => {
                co2_scrubber_candidates.retain(|e| e.0[index] == Bit::One);
            },
            _ => {
                co2_scrubber_candidates.retain(|e| e.0[index] == Bit::Zero);
            }
        }

        if co2_scrubber_candidates.len() == 1 {
            co2_scrubber_rating = co2_scrubber_candidates[0].clone();
            break;
        }
    }

    let generator_rating = u32::from_str_radix(&oxygen_generator_rating.to_string(), 2)?;
    let scrubber_rating = u32::from_str_radix(&co2_scrubber_rating.to_string(), 2)?;

    Ok(generator_rating * scrubber_rating)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = include_str!("../input.txt");
    let entries = parse_input(input)?;

    println!("Part 1: {}", part1(&entries)?);
    println!("Part 2: {}", part2(&entries)?);

    Ok(())
}


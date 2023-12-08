use std::num::ParseIntError;
use thiserror::Error;

pub enum Command {
    Forward(u32),
    Up(u32),
    Down(u32),
}

#[derive(Error, Debug)]
pub enum CommandParseError {
    #[error("failed to parse command: {0}")]
    InvalidCommand(String),
    #[error("failed to parse number: {0}")]
    InvalidNumber(#[from] ParseIntError),
}

impl TryFrom<&str> for Command {
    type Error = CommandParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut components = value.split_whitespace();
        match (components.next(), components.next()) {
            (Some("forward"), Some(v)) => Ok(Self::Forward(v.parse()?)),
            (Some("up"), Some(v)) => Ok(Self::Up(v.parse()?)),
            (Some("down"), Some(v)) => Ok(Self::Down(v.parse()?)),
            (Some(s), _) => Err(Self::Error::InvalidCommand(s.to_owned())),
            (_, _) => Err(Self::Error::InvalidCommand(String::new()))
        }
    }
}

#[derive(PartialEq, Eq, Hash, Default, Debug)]
pub struct Position {
    horizontal : u32,
    depth: u32,
    aim: u32,
}

impl Position {
    pub fn follow_part1(&mut self, command: &Command) {
        match command {
            Command::Forward(v) => self.horizontal += v,
            Command::Up(v) => self.depth -= v,
            Command::Down(v) => self.depth += v
        }
    }

    pub fn follow_part2(&mut self, command: &Command) {
        match command {
            Command::Forward(v) => {
                self.horizontal += v;
                self.depth += self.aim * v;
            },
            Command::Up(v) => self.aim -= v,
            Command::Down(v) => self.aim += v,
        }
    }
}

pub fn parse_input(input: &str) -> Result<Vec<Command>, CommandParseError> {
    input
        .lines()
        .map(Command::try_from)
        .collect()
}

pub fn follow_commands(commands: &[Command], rule: fn(&mut Position, &Command) -> ()) -> u32 {
    let mut position = Position::default();
    for c in commands {
        rule(&mut position, c)
    }

    position.horizontal * position.depth
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = include_str!("../input.txt");
    let commands = parse_input(input)?;
    println!("Part 1: {}", follow_commands(&commands, Position::follow_part1));
    println!("Part 2: {}", follow_commands(&commands, Position::follow_part2));

    Ok(())
}

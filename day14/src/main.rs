use std::{collections::HashMap, thread::current};
use itertools::Itertools;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Instruction {
    input: [char; 2],
    new_char: char,
}

#[derive(Debug, Clone)]
pub struct Input {
    start: String,
    rules: HashMap<[char; 2], char>,
}

impl Input {
    pub fn to_state(&self) -> State {
        let mut current_pairs_counts = HashMap::new();
        let mut char_counts = HashMap::new();

        for (first, second) in self.start.chars().tuple_windows() {
            *current_pairs_counts.entry([first, second]).or_insert(0) += 1;
        }

        for c in self.start.chars() {
            *char_counts.entry(c).or_insert(0) += 1;
        }

        State { current_pairs_counts, char_counts }
    }
}

#[derive(Debug, Clone)]
pub struct State {
    current_pairs_counts: HashMap<[char; 2], usize>,
    char_counts: HashMap<char, usize>,
}

impl State {
    pub fn apply_rules(&self, rules: &HashMap<[char; 2], char>) -> State {
        let mut new_pairs_counts = HashMap::new();
        let mut new_char_counts = HashMap::new();

        for (c, count) in &self.char_counts {
            *new_char_counts.entry(*c).or_insert(0) += count;
        }

        for (pair, count) in &self.current_pairs_counts {
            match rules.get(pair) {
                Some(c) => {
                    // all of this pair become two new pairs, and we get the new character
                    *new_pairs_counts.entry([pair[0], *c]).or_insert(0) += count;
                    *new_pairs_counts.entry([*c, pair[1]]).or_insert(0) += count;
                    *new_char_counts.entry(*c).or_insert(0) += count;
                }
                None => {
                    // no rule for this pair, it just goes through
                    *new_pairs_counts.entry(*pair).or_insert(0) += count;
                }
            }
        }

        State { current_pairs_counts: new_pairs_counts, char_counts: new_char_counts }
    }
}

pub fn parse_input(input: &str) -> Input {
    let (start, replacements) = input.split_once("\n\n").unwrap();

    let mut rules = HashMap::new();
    for replacement in replacements.lines() {
        let (pair, new) = replacement.split_once(" -> ").unwrap();
        let lhs = [pair.chars().next().unwrap(), pair.chars().next_back().unwrap()];
        let rhs = new.chars().next().unwrap();
        if rules.insert(lhs, rhs).is_some() {
            unreachable!();
        }
    }

    Input { start: start.to_string(), rules }
}

pub fn solve(input: &Input, steps: usize) -> usize {
    let mut state = input.to_state();
    for _ in 0 .. steps {
        state = state.apply_rules(&input.rules);
    }

    let max_count = state.char_counts.values().max().unwrap();
    let min_count = state.char_counts.values().min().unwrap();
    max_count - min_count
}

fn main() {
    let input = include_str!("../input.txt");
    let input = parse_input(input);
    println!("Part 1: {}", solve(&input, 10));
    println!("Part 2: {}", solve(&input, 40));
}

#[test]
pub fn test() {
    let input = r"NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

    let input = parse_input(input);
    let state = input.to_state();
    assert_eq!(solve(&input, 10), 1588);
    assert_eq!(solve(&input, 40), 2188189693529);
}
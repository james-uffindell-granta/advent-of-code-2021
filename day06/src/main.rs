use std::collections::HashMap;

#[derive(Copy, Clone, Debug)]
pub struct Lanternfish(u8);

impl Lanternfish {
    pub fn number_from_fish_after(&self, days: u32, lookup: &HashMap<u32, usize>) -> usize {
        let days_till_next_spawn = self.0;
        if days < days_till_next_spawn as u32 {
            return 1;
        }

        let days = days - (days_till_next_spawn as u32);
        *lookup.get(&days).unwrap()
    }

    pub fn live(&self) -> Vec<Lanternfish> {
        if self.0 > 0 {
            vec![Lanternfish(self.0 - 1)]
        } else {
            vec![Lanternfish(6), Lanternfish(8)]
        }
    }
}

pub fn build_number_from_scratch_after(days: u32) -> HashMap<u32, usize> {
    let mut numbers = HashMap::new();
    // assume we are considering a single lanternfish with value 0
    numbers.insert(0, 1);
    for d in 1..=7 {
        numbers.insert(d, 2);
    }

    numbers.insert(8, 3);
    numbers.insert(9, 3);

    for d in 10..=days {
        let seven_days_ago = numbers.get(&(d - 7)).unwrap();
        let nine_days_ago = numbers.get(&(d - 9)).unwrap();
        numbers.insert(d, seven_days_ago + nine_days_ago);
    }

    numbers
}

pub fn fish_after(starting_fish: &[Lanternfish], days: u32) -> usize {
    let mut pool = starting_fish.to_vec();
    for _ in 0..days {
        pool = pool.iter().flat_map(|f| f.live()).collect();
    }

    pool.len()
}

pub fn fish_after_2(starting_fish: &[Lanternfish], days: u32) -> usize {
    let lookup = build_number_from_scratch_after(days);
    starting_fish.iter().map(|f| f.number_from_fish_after(days, &lookup)).sum()
}

pub fn parse_input(input: &str) -> Vec<Lanternfish> {
    input.trim().split(',').map(|i| Lanternfish(i.parse().unwrap())).collect()
}

fn main() {
    let fish = parse_input(include_str!("../input.txt"));

    println!("Part 1: {}", fish_after(&fish, 80));
    println!("Part 2: {}", fish_after_2(&fish, 256));
}

#[test]
pub fn test_fish() {
    let start = parse_input("3,4,3,1,2");
    assert_eq!(fish_after(&start, 18), 26);
    assert_eq!(fish_after(&start, 80), 5934);

    assert_eq!(fish_after_2(&start, 18), 26);
    assert_eq!(fish_after_2(&start, 80), 5934);
    assert_eq!(fish_after_2(&start, 256), 26984457539);
}

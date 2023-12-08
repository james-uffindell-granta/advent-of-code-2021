
pub struct Submarine(u32);

pub fn parse_input(input: &str) -> Vec<Submarine> {
    input.trim().split(',').map(|i| Submarine(i.parse().unwrap())).collect()
}

pub fn find_optimal_fuel_cost(submarines: &[Submarine], cost_function: fn(u32) -> u32) -> u32 {
    let min_coordinate = submarines.iter().map(|s| s.0).min().unwrap();
    let max_coordinate = submarines.iter().map(|s| s.0).max().unwrap();

    let fuel_costs: Vec<u32> = (min_coordinate..=max_coordinate)
        .map(|c| submarines.iter().map(|s| cost_function(c.abs_diff(s.0))).sum())
        .collect();

    *fuel_costs.iter().min().unwrap()
}

pub fn crab_cost(distance: u32) -> u32 {
    distance * (distance + 1) / 2
}


fn main() {
    let submarines = parse_input(include_str!("../input.txt"));
    println!("Part 1: {}", find_optimal_fuel_cost(&submarines, |d| d));
    println!("Part 2: {}", find_optimal_fuel_cost(&submarines, crab_cost));
}

#[test]
pub fn test_subs() {
    let submarines = parse_input("16,1,2,0,4,2,7,1,2,14");
    assert_eq!(find_optimal_fuel_cost(&submarines, |d| d), 37);
    assert_eq!(find_optimal_fuel_cost(&submarines, crab_cost), 168);
}

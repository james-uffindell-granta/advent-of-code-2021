use std::ops::RangeInclusive;
use std::collections::HashSet;

#[derive(Clone, Debug)]
pub struct TargetArea {
    x: RangeInclusive<i64>,
    y: RangeInclusive<i64>
}

impl TargetArea {
    pub fn viable_x_velocities(&self) -> Vec<i64> {
        // the x velocity (absolutely) decreases by one each step.
        // therefore, the x coordinate traces out 'inverse triangular numbers'.
        // for now assume that the target area has positive x and fully negative y - the input
        // and the test both satisfy this.

        // every x value in the target area is a viable starting velocity - we could always pick
        // a y value within the target area too.
        // otherwise a starting velocity is only good if one of the partials
        // v, v + v - 1, v + v - 1 + v - 2. ...
        // intersects the target area x region.
        let mut x_velocities = Vec::new();

        for starting_v in 0..=*self.x.end() {
            if (0..=starting_v).rev()
                .scan(0, |sum, i| { *sum += i; Some(*sum)})
                .any(|p| self.x.contains(&p)) {
                    x_velocities.push(starting_v);
                }
        }

        x_velocities
    }

    pub fn viable_y_velocities(&self) -> Vec<i64> {
        // the y velocity decreases by one each step until it hits 0 and then keeps going
        // so the y value will always hit 0 again at the other side of the parabola (aha!)
        // so the absolute biggest y value we could try would be the low y value + 1
        let mut y_velocities = Vec::new();

        // so there are two possibilities here.
        // we are checking if, starting with a move downwards of value starting_v, 
        // we eventually hit the target area.
        // a move downwards with value v from the horizon comes in one of two ways:
        // either we started with a y velocity of -v, and this is our first move
        // or we started with a y velocity of +(v-1), and this move is happening after
        // we come back down to the horizon again
        for starting_v in 0..=(-*self.y.start()) {
            if (starting_v..=(-*self.y.start()))
                .scan(0, |sum, i| { *sum += i; Some(*sum)})
                .any(|p| self.y.contains(&(-p))) {
                    y_velocities.push(-starting_v); // make sure to add in both possibiltiies here
                    if starting_v > 0 {
                        y_velocities.push(starting_v-1);
                    }
                }
        }

        y_velocities
    }
}

pub fn working_initial_velocities(target: &TargetArea) -> HashSet<(i64, i64)> {
    let mut result = HashSet::new();
    let x_velocities = target.viable_x_velocities();
    let y_velocities = target.viable_y_velocities();

    for y_v in &y_velocities {
        // check this velocity for all xs - can we hit the target?
        for x_v in &x_velocities {
            let x_v_steps = (0..=*x_v).rev().chain(std::iter::repeat(0));
            let y_v_steps = (i64::MIN..=*y_v).rev();
            let positions = x_v_steps.zip(y_v_steps)
                .scan((0, 0), |(pos_x, pos_y), (v_x, v_y)| { *pos_x += v_x; *pos_y += v_y; Some((*pos_x, *pos_y))})
                .take_while(|(x, y)| x <= target.x.end() && y >= target.y.start())
                .collect::<Vec<_>>();
            if positions.iter().any(|p| target.x.contains(&p.0) && target.y.contains(&p.1)) {
                result.insert((*x_v, *y_v));
            }
        }
    }

    result
}

pub fn part_1(target: &TargetArea) -> i64 {
    let best_y = *working_initial_velocities(target).iter().map(|(_, y)| y).max().unwrap();
    best_y * (best_y + 1) / 2
}

pub fn part_2(target: &TargetArea) -> usize {
    working_initial_velocities(target).len()
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum ShootResult {
    Inconclusive,
    Hit(i64, i64),
    CannotHit,
}

pub fn parse_input(input: &str) -> TargetArea {
    let shorter = input.strip_prefix("target area: x=").unwrap();
    let (x_range, y_range) = shorter.split_once(", y=").unwrap();
    let (x_min, x_max) = x_range.split_once("..").unwrap();
    let (x_min, x_max) = (x_min.parse().unwrap(), x_max.parse().unwrap());
    let (y_min, y_max) = y_range.split_once("..").unwrap();
    let (y_min, y_max) = (y_min.parse().unwrap(), y_max.parse().unwrap());

    TargetArea { x: x_min..=x_max, y: y_min..=y_max }
}

fn main() {
    let input = include_str!("../input.txt");
    let target = parse_input(input.trim());
    println!("Part 1: {}", part_1(&target));
    println!("Part 2: {}", part_2(&target));
}

#[test]
pub fn test() {
    let target = parse_input("target area: x=20..30, y=-10..-5");
    assert_eq!(part_1(&target), 45);
    assert_eq!(part_2(&target), 112);
}

use std::iter::Sum;
use std::ops::Add;
use std::str::FromStr;
use itertools::Itertools;

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum SnailfishDigit {
    OpenBrace,
    CloseBrace,
    Number(u64),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SnailfishNumber {
    digits: Vec<SnailfishDigit>,
}

impl Add<SnailfishNumber> for SnailfishNumber {
    type Output = SnailfishNumber;

    fn add(self, rhs: SnailfishNumber) -> Self::Output {
        let digits = std::iter::once(SnailfishDigit::OpenBrace).chain(self.digits.clone()).chain(rhs.digits.clone()).chain(std::iter::once(SnailfishDigit::CloseBrace)).collect();
        Self::Output { digits }.simplify()
    }
}

impl Sum for SnailfishNumber {
    fn sum<I: Iterator<Item = SnailfishNumber>>(iter: I) -> Self {
        iter.reduce(|acc, i| acc + i).unwrap()
    }
}

impl FromStr for SnailfishNumber {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            digits: s
                .chars()
                .filter_map(|c| match c {
                    '[' => Some(SnailfishDigit::OpenBrace),
                    ']' => Some(SnailfishDigit::CloseBrace),
                    c if c.is_ascii_digit() => {
                        Some(SnailfishDigit::Number(c.to_digit(10).unwrap() as u64))
                    }
                    _ => None,
                })
                .collect(),
        })
    }
}

impl SnailfishNumber {
    pub fn simplify(&self) -> Self {
        let mut last_step = self.clone();
        'logic: loop {
            let mut next_step = last_step.clone();
            // find first nested four deep
            let mut nest_level = 0;
            for (index, digit) in last_step.digits.iter().enumerate() {
                match digit {
                    SnailfishDigit::OpenBrace => nest_level += 1,
                    SnailfishDigit::CloseBrace => nest_level -= 1,
                    SnailfishDigit::Number(x) if nest_level >= 5 => {
                        let SnailfishDigit::Number(y) = last_step.digits[index + 1] else {
                            unreachable!();
                        };

                        if let Some((i, e)) = next_step.digits[..index]
                            .iter()
                            .enumerate()
                            .rfind(|(_, e)| matches!(e, SnailfishDigit::Number(_)))
                        {
                            let SnailfishDigit::Number(old) = e else {
                                unreachable!();
                            };
                            next_step.digits[i] = SnailfishDigit::Number(old + x);
                        }

                        if let Some((i, e)) = next_step.digits[index + 2..]
                            .iter()
                            .enumerate()
                            .find(|(_, e)| matches!(e, SnailfishDigit::Number(_)))
                        {
                            let SnailfishDigit::Number(old) = e else {
                                unreachable!();
                            };
                            next_step.digits[i + index + 2] = SnailfishDigit::Number(old + y);
                        }

                        next_step.digits[index - 1] = SnailfishDigit::Number(0);
                        next_step.digits.remove(index);
                        next_step.digits.remove(index);
                        next_step.digits.remove(index);

                        last_step = next_step;
                        continue 'logic;
                    }
                    _ => {}
                }
            }

            // next check if any need to split
            for (index, digit) in last_step.digits.iter().enumerate() {
                match digit {
                    SnailfishDigit::Number(x) if x >= &10 => {
                        let new_left = x / 2;
                        let new_right = x / 2 + x % 2;
                        next_step.digits[index] = SnailfishDigit::CloseBrace;
                        next_step.digits.insert(index, SnailfishDigit::Number(new_right));
                        next_step.digits.insert(index, SnailfishDigit::Number(new_left));
                        next_step.digits.insert(index, SnailfishDigit::OpenBrace);

                        last_step = next_step;
                        continue 'logic;
                    }
                    _ => {}
                }
            }

            break;
        }

        last_step
    }

    pub fn magnitude(&self) -> u64 {
        let mut result = 0;
        let mut digits = self.digits.clone();
        while let Some(close) = digits.iter().position(|d| d == &SnailfishDigit::CloseBrace) {
            let SnailfishDigit::Number(left) = digits[close - 2] else { unreachable!(); };
            let SnailfishDigit::Number(right) = digits[close - 1] else { unreachable!(); };
            let magnitude = 3 * left + 2 * right;
            result = magnitude;
            digits[close - 3] = SnailfishDigit::Number(magnitude);
            digits.remove(close - 2);
            digits.remove(close - 2);
            digits.remove(close - 2);
        }

        result
    }
}

pub fn parse_input(input: &str) -> Vec<SnailfishNumber>  {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

pub fn part_1(input: &[SnailfishNumber]) -> u64 {
    input.iter().cloned().sum::<SnailfishNumber>().magnitude()
}

pub fn part_2(input: &[SnailfishNumber]) -> u64 {
    let mut result = 0;
    for (left, right) in input.iter().tuple_combinations() {
        result = result.max((left.clone() + right.clone()).magnitude());
        result = result.max((right.clone() + left.clone()).magnitude());
    }

    result
}

fn main() {
    let input = include_str!("../input.txt");
    let input = parse_input(input.trim());
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}


#[test]
pub fn test() {
    let input = r#"[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]
"#;
    let input = parse_input(input);
    assert_eq!(part_1(&input), 4140);
    assert_eq!(part_2(&input), 3993);
}


#[test]
pub fn test_simplify() {
    assert_eq!(
        SnailfishNumber::from_str("[[[[[9,8],1],2],3],4]")
            .unwrap()
            .simplify(),
        SnailfishNumber::from_str("[[[[0, 9],2],3],4]").unwrap()
    );

    assert_eq!(
        SnailfishNumber::from_str("[7,[6,[5,[4,[3,2]]]]]")
            .unwrap()
            .simplify(),
        SnailfishNumber::from_str("[7,[6,[5,[7,0]]]]").unwrap()
    );

    assert_eq!(
        SnailfishNumber::from_str("[[6,[5,[4,[3,2]]]],1]")
            .unwrap()
            .simplify(),
        SnailfishNumber::from_str("[[6,[5,[7,0]]],3]").unwrap()
    );

    assert_eq!(
        SnailfishNumber::from_str("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]")
            .unwrap()
            .simplify(),
        SnailfishNumber::from_str("[[3,[2,[8,0]]],[9,[5,[7,0]]]]").unwrap()
    );

    assert_eq!(
        SnailfishNumber::from_str("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]")
            .unwrap()
            .simplify(),
        SnailfishNumber::from_str("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]").unwrap()
    );

}

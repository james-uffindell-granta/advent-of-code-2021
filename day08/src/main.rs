use std::collections::{HashSet, HashMap};

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum Segment {
    A,B,C,D,E,F,G,
}

impl Segment {
    pub fn all_segments() -> HashSet<Segment> {
        vec![Segment::A, Segment::B, Segment::C, Segment::D, Segment::E, Segment::F, Segment::G].into_iter().collect()
    }

    pub fn zero() -> HashSet<Segment> {
        vec![Segment::A, Segment::B, Segment::C, Segment::E, Segment::F, Segment::G].into_iter().collect()
    }

    pub fn one() -> HashSet<Segment> {
        vec![Segment::C, Segment::F].into_iter().collect()
    }

    pub fn two() -> HashSet<Segment> {
        vec![Segment::A, Segment::C, Segment::D, Segment::E, Segment::G].into_iter().collect()
    }

    pub fn three() -> HashSet<Segment> {
        vec![Segment::A, Segment::C, Segment::D, Segment::F, Segment::G].into_iter().collect()
    }

    pub fn four() -> HashSet<Segment> {
        vec![Segment::B, Segment::C, Segment::D, Segment::F].into_iter().collect()
    }

    pub fn five() -> HashSet<Segment> {
        vec![Segment::A, Segment::B, Segment::D, Segment::F, Segment::G].into_iter().collect()
    }

    pub fn six() -> HashSet<Segment> {
        vec![Segment::A, Segment::B, Segment::D, Segment::E, Segment::F, Segment::G].into_iter().collect()
    }

    pub fn seven() -> HashSet<Segment> {
        vec![Segment::A, Segment::C, Segment::F].into_iter().collect()
    }

    pub fn eight() -> HashSet<Segment> {
        Self::all_segments()
    }

    pub fn nine() -> HashSet<Segment> {
        vec![Segment::A, Segment::B, Segment::C, Segment::D, Segment::F, Segment::G].into_iter().collect()
    }
}

impl TryFrom<char> for Segment {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'a' => Ok(Self::A),
            'b' => Ok(Self::B),
            'c' => Ok(Self::C),
            'd' => Ok(Self::D),
            'e' => Ok(Self::E),
            'f' => Ok(Self::F),
            'g' => Ok(Self::G),
            _ => Err(())
        }
    }
}

#[derive(Clone, Debug)]
pub struct SignalPattern {
    pattern: HashSet<Segment>
}

impl SignalPattern {
    pub fn is_easy_digit(&self) -> bool {
        self.pattern.len() == 2
        || self.pattern.len() == 3
        || self.pattern.len() == 4
        || self.pattern.len() == 7
    }
}

impl TryFrom<&str> for SignalPattern {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(Self { pattern: value.chars().map(|c| Segment::try_from(c).unwrap()).collect() })
    }
}

#[derive(Clone, Debug)]
pub struct Entry {
    // ten of these
    patterns: Vec<SignalPattern>,
    // four of these
    values: Vec<SignalPattern>,
}

impl Entry {
    pub fn decode_output(&self) -> u32 {
        let mut mappings: HashMap<_, _> = Segment::all_segments()
        .into_iter()
            .map(|s| (s, Segment::all_segments()))
            .collect();

        let all_patterns: Vec<_> = self.patterns.iter().chain(self.values.iter()).collect();

        for p in all_patterns {
            if p.pattern.len() == 2 {
                // this is a 1
                for s in p.pattern.iter() {
                    mappings.entry(*s).and_modify(|options| {
                        let intersection: HashSet<Segment> = options.intersection(&Segment::one()).copied().collect();
                        *options = intersection;
                    });
                }
            }

            if p.pattern.len() == 3 {
                // this is a 7
                for s in p.pattern.iter() {
                    mappings.entry(*s).and_modify(|options| {
                        let intersection: HashSet<Segment> = options.intersection(&Segment::seven()).copied().collect();
                        *options = intersection;
                    });
                }
            }

            if p.pattern.len() == 4 {
                // this is a 4
                for s in p.pattern.iter() {
                    mappings.entry(*s).and_modify(|options| {
                        let intersection: HashSet<Segment> = options.intersection(&Segment::four()).copied().collect();
                        *options = intersection;
                    });
                }
            }

            if p.pattern.len() == 5 {
                // this is a 2, 3, or 5 - must have A/D/G in the mapping somewhere
                // so remove A, D, and G from the options of what's not in this pattern
                for s in Segment::all_segments().difference(&p.pattern) {
                    mappings.entry(*s).and_modify(|options| {
                        options.remove(&Segment::A);
                        options.remove(&Segment::D);
                        options.remove(&Segment::G);
                    });
                }
            }

            if p.pattern.len() == 6 {
                // this is a 0, 6, or 9, so the missing segment must be either D, C, or E
                let missing_segments = vec![Segment::C, Segment::D, Segment::E].into_iter().collect::<HashSet<_>>();
                for s in Segment::all_segments().difference(&p.pattern) {
                    mappings.entry(*s).and_modify(|options| {
                        let intersection: HashSet<Segment> = options.intersection(&missing_segments).copied().collect();
                        *options = intersection;
                    });
                }
            }
        }

        let mut progressed = false;
        loop {
            if mappings.iter().all(|(_, v)| v.len() == 1) {
                // found our mappings
                break;
            }

            let resolved: Vec<_> = mappings.clone().into_iter().filter(|(_, v)| v.len() == 1).collect();
            for (k, v) in resolved {
                for (k2, v2) in mappings.iter_mut() {
                    if *k2 != k {
                        progressed |= v2.remove(v.iter().next().unwrap());
                    }
                }
            }

            if !progressed {
                panic!()
            }
        }

        let mut digits = String::new();
        for pattern in self.values.iter() {
            let number = pattern.pattern.iter().map(|s| *mappings.get(s).unwrap().iter().next().unwrap()).collect::<HashSet<_>>();
            if number == Segment::one() {
                digits.push('1');
            } else if number == Segment::two() {
                digits.push('2');
            } else if number == Segment::three() {
                digits.push('3');
            } else if number == Segment::four() {
                digits.push('4');
            } else if number == Segment::five() {
                digits.push('5');
            } else if number == Segment::six() {
                digits.push('6');
            } else if number == Segment::seven() {
                digits.push('7');
            } else if number == Segment::eight() {
                digits.push('8');
            } else if number == Segment::nine() {
                digits.push('9');
            } else if number == Segment::zero() {
                digits.push('0')
            }
        }

        digits.parse().unwrap()
    }
}

pub fn parse_input(input: &str) -> Vec<Entry> {
    let mut entries = Vec::new();
    for l in input.lines() {
        let mut components = l.split('|');
        let patterns = components.next().unwrap();
        let outputs = components.next().unwrap();
        entries.push(
            Entry {
                patterns: patterns.split_whitespace().map(SignalPattern::try_from).collect::<Result<_,_>>().unwrap(),
                values: outputs.split_whitespace().map(SignalPattern::try_from).collect::<Result<_, _>>().unwrap()
            });
    }

    entries
}

pub fn part_1(entries: &[Entry]) -> usize {
    entries.iter().map(|e| e.values.iter().filter(|p| p.is_easy_digit()).count()).sum()
}

pub fn part_2(entries: &[Entry]) -> u32 {
    entries.iter().map(|e| e.decode_output()).sum()
}

fn main() {
    let entries = parse_input(include_str!("../input.txt"));

    println!("Part 1: {}", part_1(&entries));
    println!("Part 2: {}", part_2(&entries));
}

#[test]
pub fn test() {
    let input = r"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb |    fdgacbe cefdb cefbgd gcbe
    edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec |    fcgedb cgb dgebacf gc
    fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef |    cg cg fdcagb cbg
    fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega |    efabcd cedba gadfec cb
    aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga |    gecf egdcabf bgf bfgea
    fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf |    gebdcfa ecba ca fadegcb
    dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf |    cefg dcbef fcge gbcadfe
    bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd |    ed bcgafe cdgba cbgef
    egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg |    gbdfcae bgc cg cgb
    gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc |    fgae cfgab fg bagce";

    let entries = parse_input(input);

    assert_eq!(part_1(&entries), 26);

    let single_entry = parse_input("acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab |    cdfeb fcadb cdfeb cdbaf");

    assert_eq!(single_entry.first().unwrap().decode_output(), 5353);
    assert_eq!(entries.iter().map(|e| e.decode_output()).sum::<u32>(), 61229);
}

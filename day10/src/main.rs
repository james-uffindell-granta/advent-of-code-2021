#[derive(PartialEq, Eq, Debug, Hash, Copy, Clone)]
pub enum Symbol {
    OpenParen,
    CloseParen,
    OpenBracket,
    CloseBracket,
    OpenBrace,
    CloseBrace,
    OpenAngle,
    CloseAngle
}

impl Symbol {
    pub fn score(&self) -> u32 {
        match self {
            Symbol::OpenParen => todo!(),
            Symbol::CloseParen => 3,
            Symbol::OpenBracket => todo!(),
            Symbol::CloseBracket => 57,
            Symbol::OpenBrace => todo!(),
            Symbol::CloseBrace => 1197,
            Symbol::OpenAngle => todo!(),
            Symbol::CloseAngle => 25137,
        }
    }

    pub fn points(&self) -> u32 {
        match self {
            Symbol::OpenParen => todo!(),
            Symbol::CloseParen => 1,
            Symbol::OpenBracket => todo!(),
            Symbol::CloseBracket => 2,
            Symbol::OpenBrace => todo!(),
            Symbol::CloseBrace => 3,
            Symbol::OpenAngle => todo!(),
            Symbol::CloseAngle => 4,
        }
    }

    pub fn pair(&self) -> Symbol {
        match self {
            Symbol::OpenParen => Symbol::CloseParen,
            Symbol::CloseParen => Symbol::OpenParen,
            Symbol::OpenBracket => Symbol::CloseBracket,
            Symbol::CloseBracket => Symbol::OpenBracket,
            Symbol::OpenBrace => Symbol::CloseBrace,
            Symbol::CloseBrace => Symbol::OpenBrace,
            Symbol::OpenAngle => Symbol::CloseAngle,
            Symbol::CloseAngle => Symbol::OpenAngle,
        }
    }
}

impl TryFrom<char> for Symbol {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '(' => Ok(Self::OpenParen),
            ')' => Ok(Self::CloseParen),
            '[' => Ok(Self::OpenBracket),
            ']' => Ok(Self::CloseBracket),
            '{' => Ok(Self::OpenBrace),
            '}' => Ok(Self::CloseBrace),
            '<' => Ok(Self::OpenAngle),
            '>' => Ok(Self::CloseAngle),
            _ => Err(()),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Hash, Debug)]
pub enum LineStatus {
    Okay,
    Incomplete { completion: Vec<Symbol> },
    Corrupt { unexpected_symbol: Symbol }
}

pub fn process_line(line: &str) -> LineStatus {
    let mut stack = Vec::new();
    for c in line.chars() {
        let symbol = Symbol::try_from(c).unwrap();
        match symbol {
            Symbol::OpenParen 
            | Symbol::OpenBracket 
            | Symbol::OpenBrace 
            | Symbol::OpenAngle => stack.push(symbol),
            Symbol::CloseParen 
            | Symbol::CloseBracket
            | Symbol::CloseBrace
            | Symbol::CloseAngle => match stack.pop() {
                    Some(s) if s == symbol.pair() => (),
                    _ => return LineStatus::Corrupt { unexpected_symbol: symbol },
                },
        }
    }

    if stack.is_empty() {
        LineStatus::Okay
    } else {
        let mut completion = Vec::new();
        while let Some(s) = stack.pop() {
            completion.push(s.pair());
        }

        LineStatus::Incomplete { completion }
    }
}

pub fn line_score(line: &str) -> u64 {
    if let LineStatus::Incomplete { completion: c } = process_line(line) {
        let mut score = 0;
        for s in c {
            score *= 5;
            score += s.points() as u64;
        }

        return score;
    }

    0
}

pub fn part_1(input: &str) -> u32 {
    let lines = input.lines().map(process_line);
    let mut result = 0;
    for l in lines {
        if let LineStatus::Corrupt { unexpected_symbol: s } = l {
            result += s.score();
        }
    }

    result
}

pub fn part_2(input: &str) -> u64 {
    // think it's safe to ignore the zeros?
    let mut lines = input.lines()
        .map(line_score)
        .filter(|s| *s != 0)
        .collect::<Vec<_>>();
    lines.sort();
    *lines.get(lines.len() / 2).unwrap()
}

fn main() {
    let input = include_str!("../input.txt");

    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

#[test]
pub fn test_line_status() {
    assert_eq!(process_line("{([(<{}[<>[]}>{[]{[(<()>"), LineStatus::Corrupt { unexpected_symbol: Symbol::CloseBrace });
    assert_eq!(process_line("[[<[([]))<([[{}[[()]]]"), LineStatus::Corrupt { unexpected_symbol: Symbol::CloseParen });
    assert_eq!(process_line("[{[{({}]{}}([{[{{{}}([]"), LineStatus::Corrupt { unexpected_symbol: Symbol::CloseBracket });
    assert_eq!(process_line("[<(<(<(<{}))><([]([]()"), LineStatus::Corrupt { unexpected_symbol: Symbol::CloseParen });
    assert_eq!(process_line("<{([([[(<>()){}]>(<<{{"), LineStatus::Corrupt { unexpected_symbol: Symbol::CloseAngle });

    assert_eq!(line_score("[({(<(())[]>[[{[]{<()<>>"), 288_957);
    assert_eq!(line_score("[(()[<>])]({[<{<<[]>>("), 5566);
    assert_eq!(line_score("(((({<>}<{<{<>}{[]{[]{}"), 1_480_781);
    assert_eq!(line_score("{<[[]]>}<{[{[{[]{()[[[]"), 995_444);
    assert_eq!(line_score("<{([{{}}[<[[[<>{}]]]>[]]"), 294);
}

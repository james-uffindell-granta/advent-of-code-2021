use std::num::ParseIntError;

use thiserror::Error;

#[derive(PartialEq, Eq, Debug, Hash, Copy, Clone)]
pub enum Status {
    Unmarked,
    Marked,
}

#[derive(PartialEq, Eq, Debug, Hash, Copy, Clone)]
pub struct Entry {
    status: Status,
    value: u32,
}

#[derive(PartialEq, Eq, Debug, Hash, Clone)]
pub struct Board<T> {
    board: Vec<T>
}

impl<T> Board<T> where T : Copy + Clone {
    pub fn rows(&self) -> Vec<Vec<T>>
    {
        (0..=4)
            .map(|r| self
                .board[(r*5) ..= ((r*5) + 4)].to_vec())
            .collect()
    }

    pub fn columns(&self) -> Vec<Vec<T>>
    {
        (0..=4)
            .map(|c| (0..=4).map(|r| self.board[(r*5) + c]).collect())
            .collect()
    }
}

impl Board<Entry> {
    pub fn call_number(&mut self, number: u32) {
        if let Some(entry) = self
            .board
            .iter_mut()
            .find(|e| e.value == number) {
                entry.status = Status::Marked;
            }
        
    }

    pub fn unmarked_sum(&self) -> u32 {
        self.board.iter().filter(|e| e.status == Status::Unmarked).map(|e| e.value).sum()
    }

    pub fn is_win(&self) -> bool {
        self.rows().iter().any(|r| r.iter().all(|e| e.status == Status::Marked))
        || self.columns().iter().any(|c| c.iter().all(|e| e.status == Status::Marked))
    }
}

#[derive(Error, Debug)]
pub enum ParseBoardError {
    #[error("Parser error")]
    InvalidNumberError(#[from] ParseIntError),
}

impl TryFrom<&str> for Board<Entry> {
    type Error = ParseBoardError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let numbers : Vec<_> = value.split_whitespace().map(|n| n.parse::<u32>()).collect::<Result<_, _>>()?;
        Ok(Board { board: numbers.iter().map(|n| Entry { value: *n, status: Status::Unmarked }).collect() })
    }
}

#[derive(Clone)]
pub struct Input {
    numbers: Vec<u32>,
    boards: Vec<Board<Entry>>,
}

pub fn parse_input(input: &str) -> Input {
    let mut chunks = input.split("\n\n");
    let numbers: Vec<u32> = chunks.next().unwrap().split(",").map(|s| s.parse().unwrap()).collect();
    let mut boards = Vec::new();
    for c in chunks {
        boards.push(Board::try_from(c).unwrap());
    }

    Input { numbers, boards }
}

pub fn winning_boards(input: &Input) -> Vec<u32> {
    let mut input = input.clone();
    let mut boards = Vec::new();
    for number in input.numbers {
        for board in input.boards.iter_mut() {
            if !board.is_win() {
                // this board's already won; never do it again
                board.call_number(number);
                if board.is_win() {
                    boards.push(board.unmarked_sum() * number);
                }
            }
        }
    }

    boards
}

pub fn part_1(input: &Input) -> u32 {
    *winning_boards(input).first().unwrap()
}

pub fn part_2(input: &Input) -> u32 {
    *winning_boards(input).last().unwrap()
}

fn main()
{
    let input = parse_input(include_str!("../input.txt"));
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));

}

#[test]
pub fn test_board() {
    let board = Board { board: vec![22, 13, 17, 11, 0, 8, 2, 23, 4, 24, 21, 9, 14, 16, 7, 6, 10, 3, 18, 5, 1, 12, 20, 15, 19] };

    let expected_rows = vec![
        vec![22, 13, 17, 11, 0],
        vec![8, 2, 23, 4, 24],
        vec![21, 9, 14, 16, 7],
        vec![6, 10, 3, 18, 5],
        vec![1, 12, 20, 15, 19]
    ];

    let expected_columns = vec![
        vec![22, 8, 21, 6, 1],
        vec![13, 2, 9, 10, 12],
        vec![17, 23, 14, 3, 20],
        vec![11, 4, 16, 18, 15],
        vec![0, 24, 7, 5, 19]
    ];

    assert_eq!(board.columns(), expected_columns);
    assert_eq!(board.rows(), expected_rows);
}

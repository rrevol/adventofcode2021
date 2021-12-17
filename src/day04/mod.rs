use std::fmt::{Debug, Formatter};
use crate::utils::read_lines;
use std::fmt;
use itertools::Itertools;

pub fn day() {
    part_1();
    part_2();
}

fn parse_input() -> (Vec<i32>, Vec<Board>) {
    let file = read_lines("./src/day04/input.txt");
    let numbers = file.first()
        .unwrap_or(&"".to_string())
        .split(",")
        .map(|n| n.parse::<i32>().unwrap())
        .collect();

    let lines: Vec<&str> = file.iter().skip(1).map(|line| line.as_str()).collect();
    let mut boards: Vec<Board> = Vec::new();
    lines.chunks(6)
        .map(|chunk| [chunk[1], chunk[2], chunk[3], chunk[4], chunk[5]])
        .for_each(|rows| boards.push(Board::parse(rows)));
    (numbers, boards)
}



fn part_1() {
 let (numbers, mut boards) = parse_input();
 println!("day4 part1 : {:?}", winner(numbers, &mut boards));
}

fn winner(numbers: Vec<i32>, boards: &mut Vec<Board>) -> i32 {
    for num in numbers {
        let total = boards.iter_mut()
        .map(|board| board.mark(num).map(|sum| sum * num))
        .filter(|opt| opt.is_some())
        .next()
        .and_then(|o| o);
        if total.is_some() { return total.unwrap() }
    }
    0
}

fn part_2(){
    let (numbers, mut boards) = parse_input();
    println!("day4 part2 : {:?}", loser(numbers, &mut boards));
}

fn loser(numbers: Vec<i32>, boards: &mut Vec<Board>) -> i32 {
    for num in numbers {
        let mut winner = None;
        let losers: Vec<_> = boards
        .iter_mut()
        .map(|board| {
            if board.winner_num.is_none() {
                let score = board.mark(num);
                if score.is_some() { winner = score; }
            }
            board
        })
        .filter(|board| board.winner_num.is_none())
        .collect();
        if losers.is_empty() {
            return winner.unwrap() * num
        }
    }
    0
}

#[derive(Debug, Clone, Copy)]
struct Board {
    nums: [[Num; 5]; 5],
    winner_num: Option<(usize, usize)>
}

impl Board {
    fn parse(lines: [&str; 5]) -> Self {
        let rows: Vec<[Num; 5]> = lines.iter()
        .enumerate()
        .map(|(row, &line)| {
            let nbs: Vec<i32> = line.split_whitespace()
            .take(5)
            .map(|nb| nb.parse::<i32>().unwrap())
            .collect();
            [
                Num::new(nbs[0], row, 0),
                Num::new(nbs[1], row, 1),
                Num::new(nbs[2], row, 2),
                Num::new(nbs[3], row, 3),
                Num::new(nbs[4], row, 4),
            ]
        }).collect();

        Board {
            nums: [rows[0], rows[1], rows[2], rows[3], rows[4]],
            winner_num: None
        }
    }

    fn mark(&mut self, number: i32) -> Option<i32> {
        let mut found = false;
        let mut result: Option<i32>  = None;
        for row in 0..5 {
            for col in 0..5 {
                if self.nums[row][col].value == number {
                    self.nums[row][col].marked = true;
                    found = true;
                    result = self.won(row, col);
                    break
                }
                if found { break }
            } 
        }
        result
    }

    fn won(&mut self, row: usize, col: usize) -> Option<i32> {
        let mut marked = 0;
        for r in 0..5 {
            if self.nums[r][col].marked {
                marked += 1;
            } else {
                break
            }
        }
        
        if marked < 5 { 
            marked = 0;
            for c in 0..5 {
                if self.nums[row][c].marked {
                    marked += 1;
                } else {
                    break
                }
            } 
        }

        if marked == 5 { 
            self.winner_num = Some((row, col));
            return Some(self.unmarked_sum()); 
        }
        None
    }

    fn unmarked_sum(&self) -> i32 {
        self.nums.iter()
        .map(|a| a.iter()
            .filter(|n| !n.marked)
            .map(|n| n.value)
            .sum::<i32>())
        .sum()
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let representation = self.nums.iter()
            .map(|row| row.iter().join(" "))
            .join("\n");
        write!(f, "{}", representation)
    }
}

#[derive(Debug, Clone, Copy)]
struct Num {
    value: i32,
    marked: bool,
    row: usize,
    col: usize
}

impl Num {
    fn new(value: i32, row: usize, col: usize) -> Self {
        Num {
            value,
            marked: false,
            row,
            col
        }
    }
}

impl fmt::Display for Num {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:>2}{}", self.value, if self.marked {"."} else {" "})
    }
}

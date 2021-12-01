use crate::utils::read_lines;

pub fn day() {
    part_1();
    part_2();
}

fn parse_input() -> Vec<i32> {
    let file = read_lines("./src/day01/input.txt");
    file.into_iter().map(|line| line.parse::<i32>().unwrap()).collect()
}

fn part_1() {
    let values = parse_input();
    println!("Total increases = {}", count_increases(values));
}

fn part_2() {
    let values = parse_input();
    let sums: Vec<i32> = values.iter()
    .zip(&values[1..])
    .map(|(&a, &b)| a+b)
    .zip(&values[2..])
    .map(|(a, &b)| a+b)
    .collect();
    println!("Total sliding increases = {}", count_increases(sums));
}

fn count_increases(values: Vec<i32>) -> i32 {
    values.iter()
    .zip(&values[1..])
    .filter(|(&a, &b)| a < b)
    .count() as i32
}

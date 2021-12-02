use crate::utils::read_lines;

pub fn day() {
    part_1();
    part_2();
}

fn parse_input() -> Vec<(String, i32)> {
    let file = read_lines("./src/day02/input.txt");
    file.into_iter()
    .filter(|line| !line.is_empty())
    .map(|line| {
        let mut split = line.split_whitespace();
        let command = split.next().unwrap();
        let nb = split.next().unwrap();
        let amount = nb.parse::<i32>().unwrap();
        (String::from(command), amount)
    }).collect()
}

fn part_1(){
    let values = parse_input();
    let (x, depth) = values.into_iter()
    .fold((0, 0), |(x,y), (command, amount)| match command.as_str() {
        "forward" => (x+amount, y),
        "down" => (x, y + amount),
        "up" => (x, y - amount),
        _ => (x,y)
    });
    println!("day2 part1 = {}", x*depth);
}

fn part_2(){
    let values = parse_input();
    let (x, depth, _) = values.into_iter()
    .fold((0, 0, 0), |(x,depth, aim), (command, amount)| match command.as_str() {
        "forward" => (x+amount, depth+amount*aim, aim),
        "down" => (x, depth, aim + amount),
        "up" => (x, depth, aim - amount),
        _ => (x,depth, aim)
    });
    println!("day2 part2 = {}", x*depth);
}

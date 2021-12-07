use crate::utils::read_lines;

pub fn day() {
    part_1();
    part_2();
}

fn parse_input() -> Vec<Bits> {
    let file = read_lines("./src/day03/input.txt");
    file.into_iter()
        .filter(|line| !line.is_empty())
        .map(|line| Bits::parse(line.as_str()))
        .collect()
}

fn part_1() {
    let values = parse_input();
    let total = values
        .iter()
        .fold(Bits::parse("000000000000"), |b1, b2| b1.pile_up(b2))
        .to_canonic();
    let gamma = total.to_int();
    let epsilon = total.reversed().to_int();
    println!("day3 part1 : {}*{}={}", gamma, epsilon, gamma * epsilon);
}

fn part_2() {
    let values: Vec<Bits> = parse_input()
    .iter()
    .map(|bits| bits.to_canonic())
    .collect();

    let oxygen_setting = compute_rate(values.clone(), 1);
    let co2_setting = compute_rate(values.clone(), 0);
    
    println!(
        "day3 part2 : {}*{}={}",
        oxygen_setting,
        co2_setting,
        oxygen_setting * co2_setting
    );
}

fn compute_rate(mut remaining: Vec<Bits>, default_bit: i32) -> i32 {
    let mut index = 0;
    while remaining.len() > 1 {
        let (ones, zeros): (Vec<Bits>, Vec<Bits>) = remaining.into_iter()
        .partition(|bits| bits.values[index] == 1);
        let multiplier: f64 = default_bit as f64 - 0.5;
        let ones_length = multiplier * ones.len() as f64;
        let zeros_length = multiplier * zeros.len() as f64;
        remaining = if ones_length > zeros_length { ones }
        else if ones_length < zeros_length { zeros }
        else if default_bit == 1 { ones }
        else { zeros };
        index += 1;
    }
    remaining.first().unwrap().to_int()
}

#[derive(Debug, Clone)]
struct Bits {
    values: Vec<i32>,
    cannonic: bool,
}

impl Bits {
    fn to_int(&self) -> i32 {
        self.to_canonic()
            .values
            .iter()
            .enumerate()
            .map(|(i, &bit)| &bit << (self.values.len() - 1 - i))
            .sum()
    }

    fn parse(s: &str) -> Self {
        let v = s
            .chars()
            .map(|c| match c {
                '0' => -1,
                _ => 1,
            })
            .collect();
        Bits {
            values: v,
            cannonic: false,
        }
    }

    fn pile_up(&self, bits: &Bits) -> Bits {
        Bits {
            values: self
                .values
                .iter()
                .zip(bits.values.iter())
                .map(|(a, b)| a + b)
                .collect(),
            cannonic: false,
        }
    }

    fn to_canonic(&self) -> Bits {
        if self.cannonic {
            Bits {
                values: self.values.clone(),
                cannonic: true,
            }
        } else {
            Bits {
                values: self
                    .values
                    .iter()
                    .map(|i| { if *i >= 0 { 1 } else { 0 }})
                    .collect(),
                cannonic: true,
            }
        }
    }

    fn reversed(&self) -> Bits {
        let cannonic = self.to_canonic();
        Bits {
            values: cannonic.values.iter().map(|i| (i + 1) % 2).collect(),
            cannonic: true,
        }
    }
}

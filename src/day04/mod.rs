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
        .to_canonic(1);
    let gamma = total.to_int();
    let epsilon = total.reversed().to_int();
    println!("day3 part1 : {}*{}={}", gamma, epsilon, gamma * epsilon);
}

fn part_2() {
    let values = parse_input();
    let oxygen_bits = values
        .iter()
        .fold(Bits::parse("0000000000000"), |b1, b2| b1.pile_up(b2))
        .to_canonic(1);
    let co2_bits = oxygen_bits.reversed();

    let mut remaining = values
        .iter()
        .map(|parsed| parsed.to_canonic(1))
        .collect::<Vec<Bits>>();
    for (i, b) in oxygen_bits.values.iter().enumerate() {
        println!("{}", i);
        let (ok, _) = remaining.into_iter()
        .partition(|bits| bits.values[i] == *b);
        remaining = ok;
        if remaining.len() == 1 {
            break;
        }
    }
    let oxygen_setting = remaining.first().unwrap().to_int().clone();

    remaining = values
        .iter()
        .map(|parsed| parsed.to_canonic(1))
        .collect::<Vec<Bits>>();
        for (i, b) in co2_bits.values.iter().enumerate() {
            let (ok, _) = remaining.into_iter()
            .partition(|bits| bits.values[i] == *b);
            remaining = ok;
            if remaining.len() == 1 {
                break;
            }
        }
    let co2_setting = remaining.first().unwrap().to_int();
    
    println!(
        "day3 part2 : {}*{}={}",
        oxygen_setting,
        co2_setting,
        oxygen_setting * co2_setting
    );
}

#[derive(Debug, Clone)]
struct Bits {
    values: Vec<i32>,
    cannonic: bool,
}

impl Bits {
    fn to_int(&self) -> i32 {
        self.to_canonic(1)
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

    fn to_canonic(&self, default: i32) -> Bits {
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
                    .map(|i| {
                        if *i > 0 {
                            1
                        } else if *i < 0 {
                            0
                        } else {
                            default
                        }
                    })
                    .collect(),
                cannonic: true,
            }
        }
    }

    fn reversed(&self) -> Bits {
        let cannonic = self.to_canonic(1);
        Bits {
            values: cannonic.values.iter().map(|i| (i + 1) % 2).collect(),
            cannonic: true,
        }
    }
}

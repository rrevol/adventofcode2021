use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

pub fn read_lines<P>(filename: P) -> Vec<String>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).unwrap();
    io::BufReader::new(file)
        .lines()
        .into_iter()
        .map(|line| line.unwrap())
        .collect()
}

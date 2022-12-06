use std::{
    fs::File,
    io::{self, BufRead},
};

fn main() {
    // get input file line
    let file = File::open("inputs/6.txt").unwrap();
    let line = io::BufReader::new(file).lines().next().unwrap().unwrap();

    let group_size = 4;
    line.chars()
        .collect::<Vec<_>>()
        .windows(group_size)
        .for_each(|window| {});
}

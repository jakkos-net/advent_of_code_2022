use std::{
    fs::File,
    io::{self, BufRead},
};

fn main() {
      // get input file lines
      let file = File::open("inputs/7.txt").unwrap();
      let lines = io::BufReader::new(file)
          .lines()
          .map(|line| line.unwrap())
          .collect::<Vec<_>>();

    
}
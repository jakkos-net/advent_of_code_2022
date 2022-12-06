use std::{
    fs::File,
    io::{self, BufRead},
};

fn main() {
    // get input file line
    let file = File::open("inputs/6.txt").unwrap();
    let line = io::BufReader::new(file).lines().next().unwrap().unwrap();

    let char_vec = line.chars().collect::<Vec<_>>();

    let a_marker_end = unique_window_end(&char_vec, 4);
    let b_marker_end = unique_window_end(&char_vec, 14);

    println!("Day 6 - answer a: {a_marker_end}, answer b: {b_marker_end}");
}

pub fn unique_window_end(char_vec: &Vec<char>, window_size: usize) -> usize {
    char_vec
        .windows(window_size) // create windows e.g. [a,b,c,d,e,f,...] => [a,b,c,d], [b,c,d,e], [c,d,e,f] ...
        .enumerate() // label them with starting position e.g (0, [a,b,c,d]), (1, [b,c,d,e]), ...
        .filter(|(_, window)| window_is_unique(window)) // only keep the windows that have all unique characters
        .map(|(index, _)| index + window_size) // calculate the end position
        .next() // get the first one
        .unwrap()
}

fn window_is_unique(window: &&[char]) -> bool {
    // for each character, compare equality with all characters occuring after it
    for (i, char_a) in window.iter().enumerate() {
        for char_b in window.iter().skip(i + 1) {
            if char_a == char_b {
                return false;
            }
        }
    }
    true
}

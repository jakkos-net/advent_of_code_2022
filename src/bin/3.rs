use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufRead},
};

fn main() {
    // get input file lines
    let file = File::open("inputs/3.txt").unwrap();
    let lines = io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<_>>();

    let a_total = lines
        .iter()
        .map(|line| {
            let compart1 = HashSet::<char>::from_iter(line.chars().take(line.len() / 2)); // set of items in compartment 1
            let compart2 = HashSet::<char>::from_iter(line.chars().skip(line.len() / 2)); // set of items in compartment 2
            let shared_item = compart1.intersection(&compart2).next().unwrap(); // item in both
            priority(shared_item)
        })
        .sum::<u32>();

    let group_size = 3;
    let b_total = lines
        .chunks(group_size)
        .map(|elf_group| {
            let badge = elf_group
                .iter()
                .map(|elf| HashSet::<char>::from_iter(elf.chars())) // set of items for each elf in group
                .reduce(|a, b| a.intersection(&b).cloned().collect()) // intersection of all the sets
                .unwrap()
                .into_iter() // iterator of items in the all-intersected set
                .next() // get the item in all
                .unwrap();
            priority(&badge)
        })
        .sum::<u32>();

    println!("Day 3 - answer a: {a_total}, answer b: {b_total}");
}

fn priority(item: &char) -> u32 {
    let mut priority = item.to_ascii_lowercase() as u32 - 'a' as u32 + 1; //a -> 1, b -> 2, .. ignore upper/lower case
    if item.is_ascii_uppercase() { // if its uppercase, add 26. E.g., a is 1, A is 27, b is 2, B is 28
        priority += 26
    };
    priority
}

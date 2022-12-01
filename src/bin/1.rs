use std::{
    fs::File,
    io::{self, BufRead},
};

fn main() {
    // get input file lines
    let file = File::open("inputs/1.txt").unwrap();
    let lines = io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<_>>();

    // split the flat list of strings into a list of lists of strings (one list per elf)
    let string_list_per_elf = lines.split(|s| s.is_empty());

    // convert all the strings to numbers
    let num_list_per_elf = string_list_per_elf.map(|string_list| {
        string_list
            .iter()
            .map(|string| string.parse::<usize>().unwrap())
    });

    // sum each list of numbers
    let total_per_elf = num_list_per_elf.map(|num_list| num_list.sum::<usize>());

    // sort the totals
    let mut totals_sorted_desc = total_per_elf.collect::<Vec<_>>();
    totals_sorted_desc.sort_by(|a, b| a.cmp(b).reverse());

    let top_total = totals_sorted_desc.first().unwrap();
    let top_3_combined_total = totals_sorted_desc.iter().take(3).sum::<usize>();

    println!("Day 1 - answer a: {top_total}, answer b: {top_3_combined_total}")
}

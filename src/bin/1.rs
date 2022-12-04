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

    let total_per_elf = lines.split(|s| s.is_empty()).map(|elf_items| {
        elf_items // for each item an elf is carrying
            .iter()
            .map(|item| item.parse::<u32>().unwrap()) // convert strings to numbers, e.g. "6900" => 6900
            .sum::<u32>() // add all the numbers together
    });

    // sort the totals
    let mut totals_sorted_desc = total_per_elf.collect::<Vec<_>>();
    totals_sorted_desc.sort_by(|a, b| a.cmp(b).reverse());

    let top_total = totals_sorted_desc.first().unwrap();
    let top_3_combined_total = totals_sorted_desc.iter().take(3).sum::<u32>();

    println!("Day 1 - answer a: {top_total}, answer b: {top_3_combined_total}")
}

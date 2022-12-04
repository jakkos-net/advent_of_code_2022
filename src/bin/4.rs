use std::{
    fs::File,
    io::{self, BufRead}
};

fn main() {
    // get input file lines
    let file = File::open("inputs/4.txt").unwrap();
    let lines = io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<_>>();

    // create an iteartor of ((first_elf_start_section, first_elf_stop_section), (second_elf_start_section, second_elf_stop_section))
    let elf_pair_sections = lines
        .iter()
        .map(|line| {
            let mut elf_pair = line.split(",").map(|sections| {
                let mut sec_nums = sections
                    .split("-")
                    .map(|section| section.parse::<u32>().unwrap());
                (sec_nums.next().unwrap(), sec_nums.next().unwrap())
            });

            (elf_pair.next().unwrap(), elf_pair.next().unwrap())
        });

    let a_count = elf_pair_sections.clone()
        // filter this iterator to only contain pairs where one entirely encompasses another
        .filter(|((a_start, a_end), (b_start, b_end))| {
            (a_start <= b_start && a_end >= b_end) || (b_start <= a_start && b_end >= a_end)
        })
        .count();

    let b_count = elf_pair_sections
        // filter if the start of one elf occurs between the start and end of the other
        .filter(|((a_start, a_end), (b_start, b_end))|{
            (a_start <= b_start && b_start <= a_end) || (b_start <= a_start && a_start <= b_end)
        })
        .count();

    print!("Day 4 - answer a: {a_count}, answer b: {b_count}");
}

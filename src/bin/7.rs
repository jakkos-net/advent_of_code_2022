use std::{
    fs::File,
    io::{self, BufRead}, collections::HashMap,
};

fn main() {
    // get input file lines
    let file = File::open("inputs/7.txt").unwrap();
    let lines = io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<_>>();

    let mut folder_sizes = HashMap::<String, u32>::default();
    let mut current_folders = vec!["root".to_string()]; // store the current directory and all parent directories
    for line in lines.iter().skip(1)
    {
        let mut parts = line.split(" ");
        let part1 = parts.next().unwrap();
        let part2 = parts.next().unwrap();

        if let Ok(size) = part1.parse::<u32>(){ // if the line is a file
            for folder in &current_folders{  // add its size to the current directory and its parents
                *folder_sizes.entry(folder.to_string()).or_insert(0) += size;
            }
        }
        else if part1 == "$" && part2 =="cd"
        {
            let part3 = parts.next().unwrap();

            if part3 == ".."
            {
                current_folders.pop(); // move up a folder
            }
            else{ // part 3 is a folder name
                current_folders.push(format!("{}/{}", current_folders.last().unwrap(), part3)); // move down a folder
            }
        }
    }

    let a_total = folder_sizes.values().filter(|size| **size <= 100000).sum::<u32>();

    let total_space = 70000000;
    let needed_space = 30000000;
    let used_space = folder_sizes["root"];
    let free_space = total_space - used_space;
    let need_to_free = needed_space - free_space;

    let b_folder_size = folder_sizes.values().filter(|size| **size >= need_to_free).min().unwrap();

    println!("Day 7 - answer a: {a_total}, answer b: {b_folder_size}")
}

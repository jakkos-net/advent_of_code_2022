use std::{
    fs::File,
    io::{self, BufRead},
};

fn main() {
    // get input file lines
    let file = File::open("inputs/5.txt").unwrap();
    let lines = io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<_>>();

    let mut split = lines.split(|s| s.is_empty());
    let stack_lines = split.next().unwrap(); // lines containing information about the starting stack
    let move_lines = split.next().unwrap(); // lines containing the move instructions

    // grab the stack numbers line and make a vec for each stack number
    let mut stacks = stack_lines
        .iter()
        .rev()
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .map(|_| vec![])
        .collect::<Vec<_>>();

    // push the items onto each stack (bottom first)
    stack_lines.iter().rev().skip(1).for_each(|line| {
        line.chars()
            //The item line is formatted "[A] [B] [C]..."
            .skip(1) // so if we skip the first char
            .step_by(4) // and take every 4th, we get the item names
            .enumerate()
            .for_each(|(stack, item)| {
                if item != ' ' {
                    stacks[stack].push(item)
                }
            });
    });

    // create an iterator of (the number of items to move, which stack from, which stack to)
    let move_instructions = move_lines.iter().map(|move_line| {
        let mut nums = move_line.split_ascii_whitespace().filter_map(|s| {
            if let Ok(num) = s.parse::<usize>() {
                Some(num)
            } else {
                None
            }
        });

        let num_items_to_move = nums.next().unwrap();
        let start_stack = nums.next().unwrap() - 1; // stacks stack counting from 1
        let end_stack = nums.next().unwrap() - 1; // but we start from 0
        (num_items_to_move, start_stack, end_stack)
    });

    // need separate stacks to do the a and b parts
    let mut a_stacks = stacks.clone();
    // grab and move one piece at a time
    move_instructions
        .clone()
        .for_each(|(num_items_to_move, start_stack, end_stack)| {
            (0..num_items_to_move).for_each(|_| {
                let item = a_stacks[start_stack].pop().unwrap();
                a_stacks[end_stack].push(item);
            });
        });

    let mut b_stacks = stacks;
    // grab and move multiple pieces at a time
    move_instructions.for_each(|(num_items_to_move, start_stack, end_stack)| {
        // grab all the items at once
        let mut grabbed_items = vec![];
        (0..num_items_to_move).for_each(|_| {
            grabbed_items.push(b_stacks[start_stack].pop().unwrap());
        });
        // then push all the items at once
        grabbed_items.into_iter().rev().for_each(|item| {
            b_stacks[end_stack].push(item);
        });
    });

    let a_tops = top_items_string(&a_stacks);
    let b_tops = top_items_string(&b_stacks);

    println!("Day 5 - answer a: {a_tops}, answer b: {b_tops}");
}

fn top_items_string(stacks: &Vec<Vec<char>>) -> String {
    stacks
        .iter()
        .map(|stack| stack.last().unwrap())
        .fold(String::default(), |a, b| format!("{a}{b}"))
}

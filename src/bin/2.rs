use std::{
    fs::File,
    io::{self, BufRead},
};

fn main() {
    // get input file lines
    let file = File::open("inputs/2.txt").unwrap();
    let lines = io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<_>>();

    // iterator of (opponent move, my move)
    let rps_iter = lines.iter().map(|line| line.split(" ")).map(|mut split| {
        (
            match split.nth(0).unwrap() {
                "A" => Rps::Rock,
                "B" => Rps::Paper,
                "C" => Rps::Scissors,
                _ => panic!("Wrong opponent input!"),
            },
            match split.nth(0).unwrap() {
                "X" => Rps::Rock,
                "Y" => Rps::Paper,
                "Z" => Rps::Scissors,
                _ => panic!("Wrong mine input!"),
            },
        )
    });

    let a_score_total = rps_iter
        .map(|(opponent, mine)| {
            mine.select_score() + mine.outcome_against(&opponent).outcome_score()
        })
        .sum::<u32>();

    // iterator of (opponent move, outcome)
    let outcome_iter = lines.iter().map(|line| line.split(" ")).map(|mut split| {
        (
            match split.nth(0).unwrap() {
                "A" => Rps::Rock,
                "B" => Rps::Paper,
                "C" => Rps::Scissors,
                _ => panic!("Wrong opponent input!"),
            },
            match split.nth(0).unwrap() {
                "X" => Outcome::Lose,
                "Y" => Outcome::Draw,
                "Z" => Outcome::Win,
                _ => panic!("Wrong outcome input!"),
            },
        )
    });

    let b_score_total = outcome_iter
        .map(|(opponent, outcome)| {
            // pick my move based on the desired outcome and what the opponent did
            let mine = match outcome {
                Outcome::Lose => opponent.wins_against(),
                Outcome::Draw => opponent,
                Outcome::Win => opponent.loses_against(),
            };

            outcome.outcome_score() + mine.select_score()
        })
        .sum::<u32>();

    println!("Day 2 - answer a: {a_score_total}, answer b: {b_score_total}");
}

#[derive(PartialEq)]
enum Rps {
    Rock,
    Paper,
    Scissors,
}

impl Rps {
    fn outcome_against(&self, other: &Self) -> Outcome {
        if &self.loses_against() == other {
            Outcome::Lose
        } else if &self.wins_against() == other {
            Outcome::Win
        } else {
            Outcome::Draw
        }
    }

    fn loses_against(&self) -> Self {
        match self {
            Rps::Rock => Rps::Paper,
            Rps::Paper => Rps::Scissors,
            Rps::Scissors => Rps::Rock,
        }
    }

    fn wins_against(&self) -> Self {
        match self {
            Rps::Rock => Rps::Scissors,
            Rps::Paper => Rps::Rock,
            Rps::Scissors => Rps::Paper,
        }
    }

    fn select_score(&self) -> u32 {
        match self {
            Rps::Rock => 1,
            Rps::Paper => 2,
            Rps::Scissors => 3,
        }
    }
}

enum Outcome {
    Lose,
    Draw,
    Win,
}

impl Outcome {
    fn outcome_score(&self) -> u32 {
        match self {
            Outcome::Lose => 0,
            Outcome::Draw => 3,
            Outcome::Win => 6,
        }
    }
}

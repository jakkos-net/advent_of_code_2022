# Advent of Code 2022 solutions in Rust

## Usage
1. Clone the project
2. Run `cargo run --bin <DAYNUMBER>` in the root directory to output the two answers for that day

## Notes
* As we assume that each question input is correctly formatted, there's a lot of use of `unwrap`. In a real scenario propagating errors through `Result`s should be done instead.
* Starting out, I'm trying to restrict myself to just using the standard library, there are many useful crates I would use otherwise (e.g. itertools). It's possible this restriction may change as I get fed up with writing `collect::<Vec<_>>()` over and over again :P
* Solutions tend to be a mix of valuing code readability while maintaining reasonable performance.
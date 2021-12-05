use std::io::BufRead;
use std::str::FromStr;

use aoc2021::sliding_window;


/// Split lines in input and return the result parsed as T.
/// 
/// Perfect to read lines read from stdin and parse each line as some sort of
/// data.
/// 
/// ## Arguments
/// 
/// * `input` - something implementing BufRead
fn parse_to_vec<T: FromStr>(input: impl BufRead) -> Vec<T> {
    input
        .lines()
        .filter_map(|s| match s.unwrap().parse::<T>() {
            Ok(value) => Some(value),
            _ => None,
        })
        .collect()
}

fn main() {
    let stdin = std::io::stdin();
    let numbers: Vec<i32> = parse_to_vec(stdin.lock());

    let result = sliding_window::increases(&numbers, 1);

    println!("Result: {}", result);
}
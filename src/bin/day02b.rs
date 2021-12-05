use std::io::BufRead;

use aoc2021::submarine::Instruction;

#[derive(Debug, Clone)]
struct State {
    horizontal: i32,
    depth: i32,
    aim: i32,
}

fn main() {
    let stdin = std::io::stdin();
    let position = stdin
        .lock()
        .lines()
        .filter_map(|s| match s.unwrap().parse::<Instruction>() {
            Ok(instruction) => Some(instruction),
            _ => None,
        })
        .fold(
            State {
                horizontal: 0,
                depth: 0,
                aim: 0,
            },
            |acc, i| match i {
                Instruction::Forward(value) => State {
                    horizontal: acc.horizontal + (value as i32),
                    depth: acc.depth + acc.aim * (value as i32),
                    aim: acc.aim
                },
                Instruction::Down(value) => State {
                    horizontal: acc.horizontal,
                    depth: acc.depth,
                    aim: acc.aim + (value as i32),
                },
                Instruction::Up(value) => State {
                    horizontal: acc.horizontal,
                    depth: acc.depth,
                    aim: acc.aim - (value as i32),
                },
            },
        );

    println!("{:?}", position);
    println!("{}", position.horizontal * position.depth);
}
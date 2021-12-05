use std::io::BufRead;

use aoc2021::submarine::Instruction;

#[derive(Debug, Clone)]
struct Position {
	horizontal: i32,
	depth: i32,
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
			Position {
				horizontal: 0,
				depth: 0,
			},
			|acc, i| match i {
				Instruction::Forward(value) => Position {
					horizontal: acc.horizontal + (value as i32),
					depth: acc.depth,
				},
				Instruction::Down(value) => Position {
					horizontal: acc.horizontal,
					depth: acc.depth + (value as i32),
				},
				Instruction::Up(value) => Position {
					horizontal: acc.horizontal,
					depth: acc.depth - (value as i32),
				},
			},
		);

	println!("{:?}", position);
	println!("{}", position.horizontal * position.depth);
}

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

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_example_input_should_result_in_7() {
		// Given
		let input = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

		// When
		let result = sliding_window::increases(&input, 1);

		// Then
		assert_eq!(7, result);
	}
}

use std::io::BufRead;
use std::iter;

fn gamma_filter(n: &usize, word_count: &usize) -> char {
	match n > &(word_count / 2) {
		true => '1',
		false => '0',
	}
}

fn sigma_filter(n: &usize, word_count: &usize) -> char {
	match n < &(word_count / 2) {
		true => '1',
		false => '0',
	}
}

fn main() {
	let stdin = std::io::stdin();

	let diagnostic_report: Vec<String> = stdin
		.lock()
		.lines()
		.filter_map(|s| Some(s.unwrap()))
		.collect();
	//println!("{:?}", diagnostic_report);

	let word_count = diagnostic_report.len();

	let mut report_iterator = diagnostic_report.iter();
	let first_word = report_iterator.next().expect("report is empty");
	let word_length = first_word.len();

	let sums = iter::once(first_word).chain(report_iterator).fold(
		vec![0; word_length],
		|acc: Vec<usize>, e: &String| {
			e.chars()
				.enumerate()
				.map(|(i, c)| match c {
					'1' => acc[i] + 1,
					_ => acc[i],
				})
				.collect()
		},
	);
	println!("{:?}, {:?}", sums, word_count);

	let gamma_rate_string: String = sums.iter().map(|n| gamma_filter(n, &word_count)).collect();
	let gamma_rate = usize::from_str_radix(&gamma_rate_string, 2).expect("foo");
	println!("{:?}, {:?}", gamma_rate_string, gamma_rate);

	let sigma_rate_string: String = sums.iter().map(|n| sigma_filter(n, &word_count)).collect();
	let sigma_rate = usize::from_str_radix(&sigma_rate_string, 2).expect("foo");
	println!("{:?}, {:?}", sigma_rate_string, sigma_rate);

	println!("{}", gamma_rate * sigma_rate);
}

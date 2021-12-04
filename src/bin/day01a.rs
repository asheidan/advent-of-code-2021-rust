use std::io::BufRead;
use std::str::FromStr;

/// Returns how many "times" the input increases
/// 
/// This is done by comparing sliding windows of `size`.
///
/// ## Arguments
///
/// * `input` - an i32 slice that should be examined
/// * `size` - an usize that determines the size of the window
fn increases(input: &[i32], size: usize) -> i32 {
    input.windows(size + 1).filter(|w| w[0] < w[size]).count() as i32
}

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

    let result = increases(&numbers, 1);

    println!("Result: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_increasing_pair_should_work() {
        // Given
        let data = [0, 1];

        // When
        let result = increases(&data, 1);

        // Then
        assert_eq!(1, result);
    }

    #[test]
    fn test_single_non_increasing_pair_should_work() {
        // Given
        let data = [1, 1];

        // When
        let result = increases(&data, 1);

        // Then
        assert_eq!(0, result);
    }

    #[test]
    fn test_empty_slice_should_result_in_0() {
        // Given
        let data = [];

        // When
        let result = increases(&data, 1);

        // Then
        assert_eq!(0, result);
    }

    #[test]
    fn test_given_example() {
        // Given
        let data = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

        // When
        let result = increases(&data, 1);

        // Then
        assert_eq!(7, result);
    }
}

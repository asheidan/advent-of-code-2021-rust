
/// Returns how many "times" the input increases
///
/// ## Arguments
///
/// * `input` - an integer slice that should be examined
fn increases(input: &[i32]) -> i32 {
    input.windows(2).filter(|w| w[0] < w[1]).count() as i32
}

fn main() {
    let stdin = std::io::stdin();
    let numbers: Vec<i32> = stdin
        .lines()
        .filter_map(|s| s.unwrap().parse::<i32>())
        .collect()
        .unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_increasing_pair_should_work() {
        // Given
        let data = [0, 1];

        // When
        let result = increases(&data);

        // Then
        assert_eq!(1, result);
    }

    #[test]
    fn test_single_non_increasing_pair_should_work() {
        // Given
        let data = [1, 1];

        // When
        let result = increases(&data);

        // Then
        assert_eq!(0, result);
    }

    #[test]
    fn test_empty_slice_should_result_in_0() {
        // Given
        let data = [];

        // When
        let result = increases(&data);

        // Then
        assert_eq!(0, result);
    }

    #[test]
    fn test_given_example() {
        // Given
        let data = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

        // When
        let result = increases(&data);

        // Then
        assert_eq!(7, result);
    }
}

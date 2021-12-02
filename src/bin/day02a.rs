#[derive(Debug,PartialEq)]
enum Instruction {
    Forward(u32),
    Down(u32),
    Up(u32),
}

impl std::str::FromStr for Instruction {
    // Err should probably be a combination of multiple errors
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data: Vec<&str> = s.trim()
                               .split_whitespace()
                               .collect();

        match data[..] {
            ["forward", value] => Ok(Instruction::Forward(value.parse::<u32>()?)),
            ["down", value] => Ok(Instruction::Down(value.parse::<u32>()?)),
            ["up", value] => Ok(Instruction::Up(value.parse::<u32>()?)),
            // TODO: Fix proper error handling
            _ => Err(()),
        }
    }

}

fn main() {
    let stdin = std::io::stdin();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn instruction_from_valid_forward_string() {
        // Given
        let input = String::from("forward 5");

        // When
        let result = input.parse::<Instruction>();

        // Then
        assert_eq!(Ok(Instruction::Forward(5)), result);
    }
}
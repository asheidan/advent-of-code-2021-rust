use std::io::BufRead;

#[derive(Debug, PartialEq)]
enum Instruction {
    Forward(u32),
    Down(u32),
    Up(u32),
}

#[derive(Debug, Clone, PartialEq)]
struct InstructionError;
impl std::fmt::Display for InstructionError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "invalid instruction")
    }
}
impl std::convert::From<std::num::ParseIntError> for InstructionError {
    fn from(_: std::num::ParseIntError) -> Self {
        Self
    }
}

impl std::str::FromStr for Instruction {
    // Err should probably be a combination of multiple errors
    type Err = InstructionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data: Vec<&str> = s.trim().split_whitespace().collect();

        match data[..] {
            ["forward", value] => Ok(Instruction::Forward(value.parse::<u32>()?)),
            ["down", value] => Ok(Instruction::Down(value.parse::<u32>()?)),
            ["up", value] => Ok(Instruction::Up(value.parse::<u32>()?)),
            _ => Err(InstructionError),
        }
    }
}

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

    #[test]
    fn instruction_from_valid_down_string() {
        // Given
        let input = String::from("down 42");

        // When
        let result = input.parse::<Instruction>();

        // Then
        assert_eq!(Ok(Instruction::Down(42)), result);
    }

    #[test]
    fn instruction_from_valid_up_string() {
        // Given
        let input = String::from("up 100");

        // When
        let result = input.parse::<Instruction>();

        // Then
        assert_eq!(Ok(Instruction::Up(100)), result);
    }
}

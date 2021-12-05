#[derive(Debug, PartialEq)]
pub enum Instruction {
	Forward(u32),
	Down(u32),
	Up(u32),
}

#[derive(Debug, Clone, PartialEq)]
pub struct InstructionError;
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

#[cfg(test)]
mod tests {
	use super::*;

	mod test_fromstr {
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
}

use std::fmt;
use std::io::BufRead;
use std::vec::Vec;

const CAVE: &str = 
"#############
#...........#
###.#.#.#.###
  #.#.#.#.#
  #########";

#[derive(PartialEq, PartialOrd)]
enum Color {
    Amber,
    Bronze,
    Copper,
    Desert,
}

impl Color {
    fn energy_cost(&self) -> i64 {
        match self {
            Color::Amber => 1,
            Color::Bronze => 10,
            Color::Copper => 100,
            Color::Desert => 1000,
        }
    }

    fn marker(&self) -> &str {
        match self {
            Color::Amber => "A",
            Color::Bronze => "B",
            Color::Copper => "C",
            Color::Desert => "D",
        }
    }

}

#[derive(PartialEq, PartialOrd)]
struct Position {
    y: usize,
    x: usize,
}

impl Position {
    fn distance(&self, other: &Position) -> i64 {
        ((self.y as i64) - (other.y as i64)).abs()
            + ((self.x as i64) - (other.x as i64)).abs()
    }
}

#[derive(PartialEq, PartialOrd)]
struct Amphipod {
    position: Position,
    color: Color,
}

impl Amphipod {
    fn energy_cost(&self) -> i64 {
        self.color.energy_cost()
    }
}

struct Map {
    //map: Vec<String>,
    amphipods: Vec<Amphipod>,
}

impl FromIterator<String> for Map {
    fn from_iter<T: IntoIterator<Item = String>>(iter: T) -> Self {

        let amphipods: Vec<Amphipod> = iter.into_iter().enumerate()
            .map(|(y, line)| {
                line.chars().enumerate()
                    .filter_map(|(x, c)| {
                        let position = Position { x, y };

                        match c {
                            'A' => Some(Amphipod { position, color: Color::Amber }),
                            'B' => Some(Amphipod { position, color: Color::Bronze }),
                            'C' => Some(Amphipod { position, color: Color::Copper }),
                            'D' => Some(Amphipod { position, color: Color::Desert }),
                            _ => None,
                        }
                    }).collect::<Vec<Amphipod>>()
            })
            .flatten()
            .collect();

        Self { amphipods }
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let mut data = String::from(CAVE);

        for amphipod in &self.amphipods[..] {
            let position = &amphipod.position;
            let offset = position.y * 14 + position.x;

            data.replace_range(offset..=offset, amphipod.color.marker());
        }

        writeln!(f, "{}", data)
    }
}

fn main() {
    let stdin = std::io::stdin();
    let map: Map = stdin.lock()
        .lines()
        .filter_map(|s| Some(s.unwrap()))
        .collect();

    println!("{}", map);
}

#[cfg(test)]
mod test {
    use super::*;

    mod position {
        use super::*;

        #[test]
        fn test_distance_to_self_should_be_zero() {
            // Given
            let input = Position { x: 42, y: 13 };

            // When
            let result = input.distance(&input);

            // Then
            assert_eq!(0, result);
        }

        #[test]
        fn test_distance_to_right_position_should_be_positive() {
            // Given
            let left = Position { x: 2, y: 2 };
            let right = Position { x: 5, y: 2 };

            // When
            let result = left.distance(&right);

            // Then
            assert_eq!(3, result);
        }

        #[test]
        fn test_distance_to_left_position_should_be_positive() {
            // Given
            let right = Position { x: 5, y: 2 };
            let left = Position { x: 2, y: 2 };

            // When
            let result = right.distance(&left);

            // Then
            assert_eq!(3, result);
        }

    }

}

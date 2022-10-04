use std::fmt;
use std::io::BufRead;
use std::vec::Vec;

const CAVE: &str = 
"#############
#...........#
###.#.#.#.###
  #.#.#.#.#
  #########";
const POSSIBLE_SPOTS: [i64; 7] = [1, 2, 4, 6, 8, 10, 11];

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

#[derive(PartialEq, PartialOrd, Debug)]
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

    fn possible_moves(&self, map: &Map) -> Vec<Position> {
        match self.position.y {
            1 => match self.color {
                Color::Amber  => vec![ Position { x: 3, y: 3 } ],
                Color::Bronze => vec![ Position { x: 5, y: 3 } ],
                Color::Copper => vec![ Position { x: 7, y: 3 } ],
                Color::Desert => vec![ Position { x: 9, y: 3 } ],
            },
            _ => vec![],
        }
    }
}

/// Stores the state of the Map
///
/// This should probably also be easily cloned so I can use this as the "job token" if I want to
/// distribute the work between workers.
struct Map {
    amphipods: Vec<Amphipod>,
}

impl Map {
    /// Determine if it is possible to move from start to goal
    ///
    /// This needs to take into account if there is another Amphipod in the way.
    /// Since this is used to find the way for a particular Ampipod some simplification can
    /// probably be made by ignoring if there's actually a 'pod at the starting point.
    fn path_is_open(&self, start: &Position, goal: &Position) -> bool {
        // TODO: This is probably broken
        // The initial idea was to find all the Positions in the traveled path and then try to se
        // if there's an Amphipod in the way.
        // It might be better for performance to create a set of the existing occupied positions
        // and then try to "travel the path" (try each position in turn) to see if any is occupied
        // (exists already in the set of positions).

        /*
        let vertical_travel = match (start.y < goal.y) {
            true  => ((start.y + 1)..=goal.y).map(|y| Position { x: goal.x, y: y } ),  // travel down
            false => (goal.y..=start.y).map(|y| Position { x: start.x, y: y } ),  // travel up
        };

        let horizontal_travel = match (start.x < goal.x) {
            true  => ((start.x + 1)..goal.x).map(|x| Position { x, y: 1 } ),
            false => (goal.x..start.x).map(|x| Position { x, y: 1 } ),
        };
        */

        false
    }

    fn path(&self, start: &Position, goal: &Position) -> impl Iterator<Item = &Position> + '_ {
        let foo: Vec<Position> = vec![];

        foo.iter()
    }
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

    mod map {
        use super::*;

        #[test]
        fn test_same_start_and_goal_should_whatever() {
            // Given
            let map = Map { amphipods: vec![] };

            let start = Position { x: 1, y: 1 };

            // When
            let result: Vec<&Position> = map.path(&start, &start).collect();

            // Then
            let expected: Vec<&Position> = vec![];
            assert_eq!(expected, result);
        }
    }

}

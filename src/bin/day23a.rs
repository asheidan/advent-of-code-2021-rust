use std::fmt;
use std::iter::Iterator;
use std::io::BufRead;
use std::vec::Vec;

const CAVE: &str = 
"#############
#...........#
###.#.#.#.###
  #.#.#.#.#
  #########";
const POSSIBLE_SPOTS: [i32; 7] = [1, 2, 4, 6, 8, 10, 11];

#[derive(Clone, Copy, Hash, Eq, PartialEq, PartialOrd)]
enum Color {
    Amber = 0,
    Bronze = 1,
    Copper = 2,
    Desert = 3,
}

impl Color {
    fn marker(&self) -> &str {
        match self {
            Color::Amber => "A",
            Color::Bronze => "B",
            Color::Copper => "C",
            Color::Desert => "D",
        }
    }
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
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

#[derive(Clone, Copy, PartialEq, PartialOrd)]
struct Amphipod {
    position: Position,
    color: Color,
    has_moved: bool,
}

impl Amphipod {
    fn energy_cost(&self) -> i64 {
        10_i64.pow(self.color as u32)
    }

    fn home_column(&self) -> usize {
        3 + 2 * (self.color as usize)
    }

    fn possible_moves(&self, map: &Map) -> Vec<Position> {
        let moves: Vec<Position> = match (self.position.y, self.has_moved) {
            (1, _)     => (2..=3).map(|y| Position { x: self.home_column(), y } ).collect(),
            (_, false) => POSSIBLE_SPOTS.iter().map(|x| Position { x: *x as usize, y: 1} ).collect(),
            (_, _)     => vec![],
        };

        let valid_moves = moves.into_iter()
            .filter(|p| {
                map.path_is_open(&self.position, p)
            })
            .collect();

        valid_moves
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
        // The initial idea was to find all the Positions in the traveled path and then try to se
        // if there's an Amphipod in the way.
        // It might be better for performance to create a set of the existing occupied positions
        // and then try to "travel the path" (try each position in turn) to see if any is occupied
        // (exists already in the set of positions).

        let path_position = self.path(start, goal);

        ! self.amphipods.iter().any(|amphipod| { path_position.contains(&amphipod.position) })
    }

    fn path(&self, start: &Position, goal: &Position) -> Vec<Position> {
        let horisontal_positions = match start.x < goal.x {
            true  => (start.x+1)..=(goal.x),
            false => (goal.x)..=(start.x-1),
        }.map(|x| Position { x, y: 1 } );

        let (x, ys) = match start.y > goal.y {
            true  => (start.x, (goal.y)..=(start.y-1)),
            false => (goal.x, (start.y+1)..=(goal.y)),
        };

        println!("{:?}", ys);

        let vertical_positions = ys.map(|y| Position { x, y } );

        horisontal_positions.chain(vertical_positions).collect()
    }

    fn amphipods_organized(&self) -> bool {
        let mut cache: [bool; 8] = [false; 8];

        self.amphipods.iter().for_each(|amphipod| {
            if amphipod.home_column() == amphipod.position.x && (2..=3).contains(&amphipod.position.y)  {
                let index: usize = 2 * amphipod.color as usize + (amphipod.position.y - 2);
                cache[index] = true;
            }
        });

        cache.iter().all(|b| *b)
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
                            'A' => Some(Amphipod { position, color: Color::Amber, has_moved: false }),
                            'B' => Some(Amphipod { position, color: Color::Bronze, has_moved: false }),
                            'C' => Some(Amphipod { position, color: Color::Copper, has_moved: false }),
                            'D' => Some(Amphipod { position, color: Color::Desert, has_moved: false }),
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

/*
fn easiest_moves(map: &Map, current_minimum: i64) -> i64 {
    if map.amphipods_organized() {
        return current_minimum;
    }

    let mut local_minimum = current_minimum;
    let mut map_copy: Map = Map { amphipods: [ Amphipod{}; 8 ] }

    for n in 0..map.amphipods.len() {
        let moves = map.amphipods.get(n).expect("Missing amphipod").possible_moves(map);
        for goal in moves {
            let mut map_copy = map.clone_into();
            let mut amphipod = map_copy.amphipods.get_mut(n).expect("Missing amphipod");
            let move_cost = amphipod.position.distance(&goal) * amphipod.energy_cost();

            local_minimum = easiest_moves(map_copy, current_minimum + move_cost);
        }

    }

    return local_minimum;
}
*/


fn main() {
    let stdin = std::io::stdin();
    let map: Map = stdin.lock()
        .lines()
        .filter_map(|s| Some(s.unwrap()))
        .collect();

    //let result = easiest_moves(&map, std::i64::MAX);

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

        #[test]
        fn test_diagonal_distance_should_use_manhattan_distance() {
            // Given
            let right = Position { x: 5, y: 1 };
            let left = Position { x: 2, y: 3 };

            // When
            let result = right.distance(&left);

            // Then
            assert_eq!(5, result);
        }

    }

    mod map {
        use super::*;

        mod distance {
            use super::*;

            #[test]
            fn test_same_start_and_goal_should_generate_empty_path() {
                // Given
                let map = Map { amphipods: vec![] };

                let start = Position { x: 1, y: 1 };

                // When
                let result: Vec<Position> = map.path(&start, &start);

                // Then
                let expected: Vec<Position> = vec![];
                assert_eq!(expected, result);
            }

            #[test]
            fn test_start_single_step_left_of_goal_should_have_single_position() {
                // Given
                let map = Map { amphipods: vec![] };

                let start = Position { x: 1, y: 1 };
                let goal = Position { x: 2, y: 1 };

                // When
                let result: Vec<Position> = map.path(&start, &goal);

                // Then
                let expected: Vec<Position> = vec![ Position { x: 2, y: 1} ];
                assert_eq!(expected, result);
            }

            #[test]
            fn test_start_single_step_right_of_goal_should_have_single_position() {
                // Given
                let map = Map { amphipods: vec![] };

                let start = Position { x: 2, y: 1 };
                let goal = Position { x: 1, y: 1 };

                // When
                let result: Vec<Position> = map.path(&start, &goal);

                // Then
                let expected: Vec<Position> = vec![ Position { x: 1, y: 1} ];
                assert_eq!(expected, result);
            }

            #[test]
            fn test_start_single_step_over_goal_should_have_single_position() {
                // Given
                let map = Map { amphipods: vec![] };

                let start = Position { x: 3, y: 1 };
                let goal = Position { x: 3, y: 2 };

                // When
                let result: Vec<Position> = map.path(&start, &goal);

                // Then
                let expected: Vec<Position> = vec![ Position { x: 3, y: 2} ];
                assert_eq!(expected, result);
            }

            #[test]
            fn test_start_single_step_under_goal_should_have_single_position() {
                // Given
                let map = Map { amphipods: vec![] };

                let start = Position { x: 3, y: 2 };
                let goal = Position { x: 3, y: 1 };

                // When
                let result: Vec<Position> = map.path(&start, &goal);

                // Then
                let expected: Vec<Position> = vec![ Position { x: 3, y: 1} ];
                assert_eq!(expected, result);
            }

            #[test]
            fn test_up_up_then_left_should_have_three_positions() {
                //! #############
                //! #.Gx........#
                //! ###x#.#.#.###
                //! ###S#.#.#.###
                //! #############

                // Given
                let map = Map { amphipods: vec![] };

                let start = Position { x: 3, y: 3 };
                let goal = Position { x: 2, y: 1 };

                // When
                let result: Vec<Position> = map.path(&start, &goal);

                // Then
                let expected: Vec<Position> = vec![
                    Position { x: 2, y: 1},
                    Position { x: 3, y: 1},
                    Position { x: 3, y: 2},
                ];
                assert_eq!(expected, result);
            }

            #[test]
            fn test_left_left_then_down_down_should_have_four_positions() {
                //! #############
                //! #........xxS#
                //! ###.#.#.#x###
                //! ###.#.#.#G###
                //! #############

                // Given
                let map = Map { amphipods: vec![] };

                let start = Position { x: 12, y: 1 };
                let goal = Position { x: 10, y: 3 };

                // When
                let result: Vec<Position> = map.path(&start, &goal);

                // Then
                let expected: Vec<Position> = vec![
                    Position { x: 10, y: 1},
                    Position { x: 11, y: 1},
                    Position { x: 10, y: 2},
                    Position { x: 10, y: 3},
                ];
                assert_eq!(expected, result);
            }
        }

        mod path_is_open {
            use super::*;

            #[test]
            fn test_open_path_should_be_true() {
                //! #############
                //! #Sxxxx..B...#
                //! ###.#x#.#.###
                //! ###.#G#.#.###
                //! #############

                // Given
                let map = Map {
                    amphipods: vec![
                        Amphipod {
                            position: Position { x: 9, y: 1 },
                            color: Color::Bronze,
                            has_moved: true,
                        }
                    ]
                };

                let start = Position { x: 1, y: 1 };
                let goal = Position { x: 6, y: 3 };

                // When
                let result = map.path_is_open(&start, &goal);

                // Then
                assert_eq!(true, result)
            }

            #[test]
            fn test_blocked_path_should_be_false() {
                //! #############
                //! #SxxBx......#
                //! ###.#x#.#.###
                //! ###.#G#.#.###
                //! #############

                // Given
                let map = Map {
                    amphipods: vec![
                        Amphipod {
                            position: Position { x: 5, y: 1 },
                            color: Color::Bronze,
                            has_moved: true,
                        }
                    ]
                };

                let start = Position { x: 1, y: 1 };
                let goal = Position { x: 6, y: 3 };

                // When
                let result = map.path_is_open(&start, &goal);

                // Then
                assert_eq!(false, result)
            }
        }
        
        mod amphipods_organized {
            use super::*;

            #[test]
            fn test_all_amphipods_in_their_correct_place_should_be_organized() {
                // Given
                let map: Map = vec![
                    "#############",
                    "#...........#",
                    "###A#B#C#D###",
                    "  #A#B#C#D#",
                    "  #########",
                ].into_iter()
                    .map(String::from)
                    .collect();

                // When
                let result = map.amphipods_organized();

                // Then
                assert_eq!(true, result);
            }

            #[test]
            fn test_all_misplaced_amphipod_should_be_not_organized() {
                // Given
                let map: Map = vec![
                    "#############",
                    "#...........#",
                    "###B#A#C#D###",
                    "  #A#B#C#D#",
                    "  #########",
                ].into_iter()
                    .map(String::from)
                    .collect();

                // When
                let result = map.amphipods_organized();

                // Then
                assert_eq!(false, result);
            }
        }
    }

}

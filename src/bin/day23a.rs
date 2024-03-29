use std::cmp::Reverse;
use std::collections::HashSet;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::Iterator;
use std::vec::Vec;

use priority_queue::PriorityQueue;


const CAVE: &str = "#############
#...........#
###.#.#.#.###
  #.#.#.#.#
  #########";
const POSSIBLE_SPOTS: [i32; 7] = [1, 2, 4, 6, 8, 10, 11];

#[derive(Clone, Copy, Eq, PartialEq, PartialOrd, Debug, Hash)]
struct Position {
    y: u8,
    x: u8,
}

impl Position {
    fn distance(&self, other: &Position) -> i32 {
        ((self.y as i32) - (other.y as i32)).abs() + ((self.x as i32) - (other.x as i32)).abs()
    }
}

#[derive(Clone, Copy, Eq, PartialEq, PartialOrd, Hash)]
struct Amphipod {
    position: Position,
    color: char,
    has_moved: bool,
}

impl Amphipod {
    fn energy_cost(&self) -> i32 {
        10_i32.pow(self.color as u32 - 'A' as u32)
    }

    fn home_column(&self) -> u8 {
        3 + 2 * (self.color as u8 - 'A' as u8)
    }

    fn possible_moves(&self, map: &Map) -> Vec<Position> {
        let moves: Vec<Position> = match (self.position.y, self.has_moved) {
            (1, _) => (2..=3)
                .map(|y| Position {
                    x: self.home_column(),
                    y,
                })
                .collect(),
            (_, false) => POSSIBLE_SPOTS
                .iter()
                .map(|x| Position {
                    x: *x as u8,
                    y: 1,
                })
                .collect(),
            (_, _) => vec![],
        };

        let valid_moves = moves
            .into_iter()
            .filter(|p| map.path_is_open(&self.position, p))
            .collect();

        valid_moves
    }
}

/// Stores the state of the Map
///
/// This should probably also be easily cloned so I can use this as the "job token" if I want to
/// distribute the work between workers.
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
struct Map {
    amphipods: [Amphipod; 8],
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

        !self
            .amphipods
            .iter()
            .any(|amphipod| path_position.contains(&amphipod.position))
    }

    fn path(&self, start: &Position, goal: &Position) -> Vec<Position> {
        let horisontal_positions = match start.x < goal.x {
            true => (start.x + 1)..=(goal.x),
            false => (goal.x)..=(start.x - 1),
        }
        .map(|x| Position { x, y: 1 });

        let (x, ys) = match start.y > goal.y {
            true => (start.x, (goal.y)..=(start.y - 1)),
            false => (goal.x, (start.y + 1)..=(goal.y)),
        };

        let vertical_positions = ys.map(|y| Position { x, y });

        horisontal_positions.chain(vertical_positions).collect()
    }

    fn amphipods_organized(&self) -> bool {
        let mut cache: [bool; 8] = [false; 8];

        self.amphipods.iter().for_each(|amphipod| {
            if amphipod.home_column() == amphipod.position.x
                && (2..=3).contains(&amphipod.position.y)
            {
                let index: usize = (2 * (amphipod.color as usize - 'A' as usize) + (amphipod.position.y as usize - 2)) as usize;
                cache[index] = true;
            }
        });

        cache.iter().all(|b| *b)
    }

    fn empty() -> Self {
        Map {
            amphipods: [Amphipod {
                color: 'A',
                position: Position { y: 0, x: 0 },
                has_moved: false,
            }; 8],
        }
    }
}

impl FromIterator<String> for Map {
    fn from_iter<T: IntoIterator<Item = String>>(iter: T) -> Self {
        let amphipods = iter
            .into_iter()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .filter_map(|(x, c)| {
                        let position = Position { x: x as u8, y: y as u8 };

                        match c {
                            'A' => Some(Amphipod {
                                position,
                                color: 'A',
                                has_moved: false,
                            }),
                            'B' => Some(Amphipod {
                                position,
                                color: 'B',
                                has_moved: false,
                            }),
                            'C' => Some(Amphipod {
                                position,
                                color: 'C',
                                has_moved: false,
                            }),
                            'D' => Some(Amphipod {
                                position,
                                color: 'D',
                                has_moved: false,
                            }),
                            _ => None,
                        }
                    })
                    .collect::<Vec<Amphipod>>()
            })
            .flatten();

        let mut array = [Amphipod {
            color: 'A',
            position: Position { y: 0, x: 0 },
            has_moved: false,
        }; 8];

        for (i, a) in amphipods.enumerate() {
            array[i] = a;
        }

        Self { amphipods: array }
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let mut data = String::from(CAVE);

        for amphipod in &self.amphipods[..] {
            let position = &amphipod.position;
            let offset = position.y * 14 + position.x;

            data.replace_range((offset as usize)..=(offset as usize), &amphipod.color.to_string());
        }

        writeln!(f, "{}", data)
    }
}

/// Return the heuristic for finishing this map.
///
/// Since this is used for prioritizing which states that should be examined in the A*-algorithm
/// this needs to be close to the actual cost of finishing this map and never higher than the
/// actual cost.
fn heuristic(map: &Map) -> i32 {
    map.amphipods.iter().map(|a| {
        let y = match a.position.x == a.home_column() {
            true => u8::max(2, a.position.y),
            false => 0,  // Ugly hack to get distance of first moving into corridor, then into room
        };
        let goal = Position{ x: a.home_column(), y };
        goal.distance(&a.position) * a.energy_cost()
    }).sum()
}

fn easiest_moves(map: Map) -> i32 {

    let mut queue: PriorityQueue<(Map, i32), Reverse<i32>> = PriorityQueue::<_, _>::new();
    queue.push((map, 0), Reverse(0 + heuristic(&map)));

    let mut seen: HashSet<Map> = HashSet::new();

    let mut minimal_cost = std::i32::MAX;

    while let Some(((map, cost), _priority)) = queue.pop() {
        //println!("{}{}, {}", map, cost, _priority.0);

        if let Some(_) = seen.replace(map) {
            continue;
        }

        if map.amphipods_organized() {
            println!("{}", cost);
            println!("q: {}", queue.len());
            minimal_cost = i32::min(minimal_cost, cost);
        }

        map.amphipods.iter().enumerate().for_each(|(n, a)| {
            let current_position = &a.position;
            let energy_cost = a.energy_cost();

            a.possible_moves(&map).iter().for_each(|goal| {
                let map_cost = cost + current_position.distance(&goal) * energy_cost;

                let mut map_copy = map;  // Copy
                map_copy.amphipods[n].position = *goal;
                map_copy.amphipods[n].has_moved = true;

                let heuristic_cost = map_cost + heuristic(&map_copy);

                if (map_cost + heuristic_cost) <= minimal_cost {
                    queue.push((map_copy, map_cost), Reverse(heuristic_cost));
                }

            });
        });
    }

    minimal_cost
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let filename = args.get(1).expect("missing filename");
    let input_file = File::open(filename).expect("failed to open file");
    let reader = BufReader::new(input_file);
    let map: Map = reader.lines().filter_map(|s| Some(s.unwrap())).collect();

    let result = easiest_moves(map);

    println!("{}", result);
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
                let map = Map::empty();

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
                let map = Map::empty();

                let start = Position { x: 1, y: 1 };
                let goal = Position { x: 2, y: 1 };

                // When
                let result: Vec<Position> = map.path(&start, &goal);

                // Then
                let expected: Vec<Position> = vec![Position { x: 2, y: 1 }];
                assert_eq!(expected, result);
            }

            #[test]
            fn test_start_single_step_right_of_goal_should_have_single_position() {
                // Given
                let map = Map::empty();

                let start = Position { x: 2, y: 1 };
                let goal = Position { x: 1, y: 1 };

                // When
                let result: Vec<Position> = map.path(&start, &goal);

                // Then
                let expected: Vec<Position> = vec![Position { x: 1, y: 1 }];
                assert_eq!(expected, result);
            }

            #[test]
            fn test_start_single_step_over_goal_should_have_single_position() {
                // Given
                let map = Map::empty();

                let start = Position { x: 3, y: 1 };
                let goal = Position { x: 3, y: 2 };

                // When
                let result: Vec<Position> = map.path(&start, &goal);

                // Then
                let expected: Vec<Position> = vec![Position { x: 3, y: 2 }];
                assert_eq!(expected, result);
            }

            #[test]
            fn test_start_single_step_under_goal_should_have_single_position() {
                // Given
                let map = Map::empty();

                let start = Position { x: 3, y: 2 };
                let goal = Position { x: 3, y: 1 };

                // When
                let result: Vec<Position> = map.path(&start, &goal);

                // Then
                let expected: Vec<Position> = vec![Position { x: 3, y: 1 }];
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
                let map = Map::empty();

                let start = Position { x: 3, y: 3 };
                let goal = Position { x: 2, y: 1 };

                // When
                let result: Vec<Position> = map.path(&start, &goal);

                // Then
                let expected: Vec<Position> = vec![
                    Position { x: 2, y: 1 },
                    Position { x: 3, y: 1 },
                    Position { x: 3, y: 2 },
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
                let map = Map::empty();

                let start = Position { x: 12, y: 1 };
                let goal = Position { x: 10, y: 3 };

                // When
                let result: Vec<Position> = map.path(&start, &goal);

                // Then
                let expected: Vec<Position> = vec![
                    Position { x: 10, y: 1 },
                    Position { x: 11, y: 1 },
                    Position { x: 10, y: 2 },
                    Position { x: 10, y: 3 },
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
                    amphipods: [
                        Amphipod {
                            position: Position { x: 9, y: 1 },
                            color: 'B',
                            has_moved: true,
                        },
                        Amphipod {
                            color: 'A',
                            position: Position { y: 0, x: 0 },
                            has_moved: false,
                        },
                        Amphipod {
                            color: 'A',
                            position: Position { y: 0, x: 0 },
                            has_moved: false,
                        },
                        Amphipod {
                            color: 'A',
                            position: Position { y: 0, x: 0 },
                            has_moved: false,
                        },
                        Amphipod {
                            color: 'A',
                            position: Position { y: 0, x: 0 },
                            has_moved: false,
                        },
                        Amphipod {
                            color: 'A',
                            position: Position { y: 0, x: 0 },
                            has_moved: false,
                        },
                        Amphipod {
                            color: 'A',
                            position: Position { y: 0, x: 0 },
                            has_moved: false,
                        },
                        Amphipod {
                            color: 'A',
                            position: Position { y: 0, x: 0 },
                            has_moved: false,
                        },
                    ],
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
                    amphipods: [
                        Amphipod {
                            position: Position { x: 5, y: 1 },
                            color: 'B',
                            has_moved: true,
                        },
                        Amphipod {
                            color: 'A',
                            position: Position { y: 0, x: 0 },
                            has_moved: false,
                        },
                        Amphipod {
                            color: 'A',
                            position: Position { y: 0, x: 0 },
                            has_moved: false,
                        },
                        Amphipod {
                            color: 'A',
                            position: Position { y: 0, x: 0 },
                            has_moved: false,
                        },
                        Amphipod {
                            color: 'A',
                            position: Position { y: 0, x: 0 },
                            has_moved: false,
                        },
                        Amphipod {
                            color: 'A',
                            position: Position { y: 0, x: 0 },
                            has_moved: false,
                        },
                        Amphipod {
                            color: 'A',
                            position: Position { y: 0, x: 0 },
                            has_moved: false,
                        },
                        Amphipod {
                            color: 'A',
                            position: Position { y: 0, x: 0 },
                            has_moved: false,
                        },
                    ],
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
                ]
                .into_iter()
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
                ]
                .into_iter()
                .map(String::from)
                .collect();

                // When
                let result = map.amphipods_organized();

                // Then
                assert_eq!(false, result);
            }
        }
    }

    mod heuristic {
        use super::*;

        #[test]
        fn goal_state_should_be_zero() {
            // Given
            let map: Map = vec![
                "#############",
                "#...........#",
                "###A#B#C#D###",
                "  #A#B#C#D#",
                "  #########",
            ]
            .into_iter()
            .map(String::from)
            .collect();

            // When
            let result = heuristic(&map);

            // Then
            assert_eq!(0, result);
        }

        #[test]
        fn amber_amphipod_out_of_place_should_be_2() {
            // Given
            let map: Map = vec![
                "#############",
                "#.A.........#",
                "###.#B#C#D###",
                "  #A#B#C#D#",
                "  #########",
            ]
            .into_iter()
            .map(String::from)
            .collect();

            // When
            let result = heuristic(&map);

            // Then
            assert_eq!(2, result);
        }

        #[test]
        fn bronze_amphipod_out_of_place_should_be_correct() {
            // Given
            let map: Map = vec![
                "#############",
                "#.........B.#",
                "###A#.#C#D###",
                "  #A#B#C#D#",
                "  #########",
            ]
            .into_iter()
            .map(String::from)
            .collect();

            // When
            let result = heuristic(&map);

            // Then
            assert_eq!(60, result);
        }

        #[test]
        fn switching_rooms_should_be_correct() {
            // Given
            let map: Map = vec![
                "#############",
                "#...........#",
                "###B#A#C#D###",
                "  #A#B#C#D#",
                "  #########",
            ]
            .into_iter()
            .map(String::from)
            .collect();

            // When
            let result = heuristic(&map);

            // Then
            assert_eq!(44, result);
        }
    }
}

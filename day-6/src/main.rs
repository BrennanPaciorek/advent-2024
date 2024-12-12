use std::collections::HashSet;
use std::env::{self, args};
use std::ops::{BitAnd, BitOr};
use std::{fs, panic};

#[derive(Clone, Copy, Debug)]
struct Direction(i32, i32);

impl Direction {
    fn change_direction(&mut self) {
        self.1 = self.0 ^ self.1;
        self.0 = self.0 ^ self.1;
        self.1 = self.0 ^ self.1;
        self.1 *= -1;
    }

    fn next_direction(&self) -> Direction {
        let row = self.1;
        let tile = self.0 * -1;
        return Direction(row, tile);
    }

    fn from_char(chr: &char) -> Direction {
        return match chr {
            '^' => Direction(-1, 0),
            '>' => Direction(0, 1),
            'v' => Direction(1, 0),
            '<' => Direction(0, -1),
            x => panic!("unexpected character {}", x),
        };
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct DirectionLog(bool, bool, bool, bool);

impl BitOr for DirectionLog {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        return Self(
            self.0 | rhs.0,
            self.1 | rhs.1,
            self.2 | rhs.2,
            self.3 | rhs.3,
        );
    }
}

impl BitAnd for DirectionLog {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        return Self(
            self.0 & rhs.0,
            self.1 & rhs.1,
            self.2 & rhs.2,
            self.3 & rhs.3,
        );
    }
}

impl DirectionLog {
    fn from_direction(direction: &Direction) -> DirectionLog {
        let up = direction.0 == -1;
        let down = direction.0 == 1;
        let right = direction.1 == 1;
        let left = direction.1 == -1;

        return DirectionLog(up, down, right, left);
    }

    fn or(&self, direction: &Direction) -> DirectionLog {
        let other_log = DirectionLog::from_direction(direction);
        return *self | other_log;
    }

    fn in_log(&self, direction: &Direction) -> bool {
        let empty_direction = Self(false, false, false, false);
        let other_log = DirectionLog::from_direction(direction);
        return *self & other_log != empty_direction;
    }

    fn to_char(&self) -> char {
        let up_down: bool = self.0 | self.1;
        let right_left: bool = self.2 | self.3;

        if up_down && right_left {
            return '+';
        } else if up_down {
            return '|';
        } else if right_left {
            return '-';
        } else {
            return '?';
        }
    }
}

#[derive(Clone, Debug)]
enum Tile {
    Guard(Direction),
    Empty,
    Obstacle,
    Visited(DirectionLog),
}

impl Tile {
    fn from_char(chr: &char) -> Self {
        match chr {
            '.' => Self::Empty,
            '#' => Self::Obstacle,
            _ => Self::Guard(Direction::from_char(chr)),
        }
    }

    fn to_char(&self) -> char {
        return match self {
            Self::Empty => '.',
            Self::Obstacle => '#',
            Self::Visited(direction) => direction.to_char(),
            _ => 'X',
        };
    }
}

type Board = Vec<Vec<Tile>>;

fn produce_board(input_data: &str) -> Board {
    return input_data
        .lines()
        .map(|s| {
            s.chars()
                .map(|c| Tile::from_char(&c))
                .collect::<Vec<Tile>>()
        })
        .collect();
}

fn print_board(board: &Board, collision: (usize, usize)) {
    for row in board.iter().enumerate() {
        for tile in row.1.iter().enumerate() {
            if (row.0, tile.0) == collision {
                print!("X");
            } else {
                print!("{}", tile.1.to_char());
            }
        }
        println!("");
    }
}

fn guard_start(board: &Board) -> (i32, i32) {
    for row in board.iter().enumerate() {
        for tile in row.1.iter().enumerate() {
            match tile.1 {
                Tile::Guard(_) => return (row.0 as i32, tile.0 as i32),
                _ => {}
            }
        }
    }
    panic!("Guard not found");
}

fn to_valid_coordinates(position: &(i32, i32)) -> Option<(usize, usize)> {
    if position.0.is_negative() || position.1.is_negative() {
        return None;
    }
    return Some((position.0 as usize, position.1 as usize));
}

fn get_tile(board: &Board, position: &(i32, i32)) -> Option<Tile> {
    if let Some(position) = to_valid_coordinates(position) {
        if let Some(row) = board.get(position.0) {
            if let Some(tile) = row.get(position.1) {
                return Some(tile.clone());
            }
        }
    }
    return None;
}

fn part_one(input_data: &str) -> Result<u64, &str> {
    let mut board: Board = produce_board(input_data);
    let guard_position: (i32, i32) = guard_start(&board);
    let guard: Tile = get_tile(&board, &guard_position).expect("Guard could not be fetched");

    board = run_simulation(board, false, guard, guard_position)
        .expect_err("found infinite loop in part one board");

    return Ok(board
        .iter()
        .map(|row| {
            row.iter()
                .filter(|x| match *x {
                    Tile::Visited(_) => true,
                    _ => false,
                })
                .count() as u64
        })
        .sum());
}

// Every time the guard moves, check the position clockwise to it
// if it was visited, and if the guard were to change direction and it would match the direction of
// the visited tile, append the answer counter, since placing an obstacle in the target position
// would result in an infinite loop.
//
//
// This does not work, since we need to be able to store multiple directions in a visited tile's
// state.We can account for this by making efficient use of (i32, i32) directions to store one up/down
// direction, one right/left direction.
//
// The check needs to accound for if an obstacle can be placed (ie if the obstacle location would
// be on the board if placed)
//
fn run_simulation(
    mut board: Board,
    debug: bool,
    mut simulated_guard: Tile,
    start: (i32, i32),
) -> Result<Board, Board> {
    let mut simulated_guard_position = start;
    while let Some(adjacent_tile) = get_tile(&board, &simulated_guard_position) {
        if let Tile::Guard(simulated_guard_direction) = simulated_guard {
            match adjacent_tile {
                Tile::Visited(adjacent_tile_direction) => {
                    if adjacent_tile_direction.in_log(&simulated_guard_direction) {
                        if debug {
                            println!("Loop found");
                            print_board(
                                &board,
                                to_valid_coordinates(&simulated_guard_position).unwrap(),
                            );
                        }
                        return Ok(board);
                    }

                    board[simulated_guard_position.0 as usize]
                        [simulated_guard_position.1 as usize] =
                        Tile::Visited(adjacent_tile_direction.or(&simulated_guard_direction));
                    simulated_guard_position = (
                        simulated_guard_position.0 + simulated_guard_direction.0,
                        simulated_guard_position.1 + simulated_guard_direction.1,
                    );
                }
                Tile::Obstacle => {
                    simulated_guard_position = (
                        simulated_guard_position.0 - simulated_guard_direction.0,
                        simulated_guard_position.1 - simulated_guard_direction.1,
                    );
                    simulated_guard = Tile::Guard(simulated_guard_direction.next_direction())
                }
                _ => {
                    board[simulated_guard_position.0 as usize]
                        [simulated_guard_position.1 as usize] =
                        Tile::Visited(DirectionLog::from_direction(&simulated_guard_direction));
                    simulated_guard_position = (
                        simulated_guard_position.0 + simulated_guard_direction.0,
                        simulated_guard_position.1 + simulated_guard_direction.1,
                    );
                }
            };
        }
    }
    return Err(board);
}

fn part_two(input_data: &str, debug: bool) -> Result<u64, &str> {
    let mut board: Board = produce_board(input_data);
    let original_guard_position: (i32, i32) = guard_start(&board);
    let original_guard: Tile =
        get_tile(&board, &original_guard_position).expect("Guard could not be fetched");
    let mut guard_position: (i32, i32) = original_guard_position.clone();
    let mut guard: Tile = original_guard.clone();

    let mut obstacles = HashSet::new();
    while let Some(_) = get_tile(&board, &guard_position) {
        if let Tile::Guard(mut direction) = guard {
            let target_position: (i32, i32) = (
                guard_position.0 + direction.0,
                guard_position.1 + direction.1,
            );
            match get_tile(&board, &target_position) {
                Some(tile) => {
                    let coordinates = to_valid_coordinates(&target_position).unwrap();
                    let mut simulation_board: Board = produce_board(input_data);
                    simulation_board[coordinates.0][coordinates.1] = Tile::Obstacle;
                    match run_simulation(
                        simulation_board,
                        debug,
                        original_guard.clone(),
                        original_guard_position.clone(),
                    ) {
                        Ok(_) => {
                            obstacles.insert(coordinates);
                        }
                        Err(_) => {}
                    };

                    match tile {
                        Tile::Obstacle => {
                            direction.change_direction();
                            guard = Tile::Guard(direction);
                        }
                        _ => {
                            guard_position = target_position;
                        }
                    };
                }
                None => guard_position = target_position,
            };
        }
    }

    return Ok(obstacles.len() as u64);
}

fn print_result(prefix: &str, result: Result<u64, &str>) {
    match result {
        Ok(num) => println!("{}: {}", prefix, num),
        Err(reason) => println!("{}: {}", prefix, reason),
    }
}

fn main() {
    let argv: Vec<String> = args().collect();
    let debug: bool = env::vars()
        .find(|(var, value)| var == "DEBUG" && value == "1")
        .is_some();

    let input_file_path = argv.get(1).expect("missing file name argument");
    let input_file =
        fs::canonicalize(input_file_path).expect("Could not find and cannonicalize input file");

    println!("Using input file {:?}", input_file);

    let input_data = fs::read_to_string(input_file)
        .expect("Couldn't read input file, need to do that to run code.");

    let part_one_output = part_one(&input_data);
    let part_two_output = part_two(&input_data, debug);

    print_result("Part 1", part_one_output);
    print_result("Part 2", part_two_output);
}

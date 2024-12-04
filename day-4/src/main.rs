use std::env::args;
use std::fs;

const KEY: [char; 4] = ['X', 'M', 'A', 'S'];

fn get_char(grid: &Vec<&str>, x: usize, y: usize) -> Option<char> {
    match grid.get(y) {
        Some(line) => {
            return line.chars().nth(x)
        },
        None => return None
    }
}

fn has_ma(c1: Option<char>, c2: Option<char>) -> bool{
    if c1.is_some() && c2.is_some() {
        let c1: char = c1.unwrap();
        let c2: char = c2.unwrap();
        if (c1 == 'M' || c1 == 'S') && (c2 == 'M' || c2 == 'S') {
            return c1 != c2;
        }
    }
    return false;
}

fn has_two_mas(grid: &Vec<&str>, x: usize, y: usize) -> bool {
    match get_char(grid, x, y) {
        Some(character) => {
            if character == 'A' {
                let x = x as i32;
                let y = y as i32;

                let slash1_1: (i32, i32) = (x+1, y+1);
                let slash1_2: (i32, i32) = (x-1, y-1);
                
                let slash2_1: (i32, i32) = (x-1, y+1);
                let slash2_2: (i32, i32) = (x+1, y-1);

                if slash1_2.1.is_negative() || slash1_2.0.is_negative() || slash2_1.0.is_negative() || slash2_2.1.is_negative() {
                    return false;
                }
                return has_ma(get_char(grid, slash1_1.0 as usize, slash1_1.1 as usize), get_char(grid, slash1_2.0 as usize, slash1_2.1 as usize)) && has_ma(get_char(grid, slash2_1.0 as usize, slash2_1.1 as usize), get_char(grid, slash2_2.0 as usize, slash2_2.1 as usize));
            }
        }
        None => {}
    };
    return false;
}

fn has_xmas(grid: &Vec<&str>, x: usize, y: usize, index: usize, direction: (i32, i32)) -> u64 {
    match get_char(grid, x, y) {
        Some(character) => {
            if character == KEY[index] {
                match index {
                    0 => {
                        let mut matches = 0;
                        // Check all directions for the next letter
                        for dx in -1..=1 {
                            for dy in -1..=1 {
                                if dx == 0 && dy == 0 {
                                    continue;
                                }
                                let x2 = x as i32 + dx;
                                let y2 = y as i32 + dy;
                                if x2 >= 0 && y2 >= 0 {
                                    matches += has_xmas(grid, x2 as usize, y2 as usize, 1, (dx, dy));
                                }
                            }
                        }
                        return matches;
                    },
                    3 => return 1,
                    i => {
                        let x2 = x as i32 + direction.0;
                        let y2 = y as i32 + direction.1;
                        return has_xmas(grid, x2 as usize, y2 as usize, i + 1, direction);
                    }
                }
            }
        }
        None => return 0,
    };
    return 0;
}

fn part_one(input_data: &str) -> Result<u64, &str> {
    let grid: Vec<&str> = input_data
        .lines()
        .collect();
    let mut answer: u64 = 0;
    for line in grid.iter().enumerate() {
        for char in (*line.1).chars().enumerate() {
            answer += has_xmas(&grid, char.0, line.0, 0, (0, 0));
        }
    }
    return Ok(answer);
}

fn part_two(input_data: &str) -> Result<u64, &str>{
    let grid: Vec<&str> = input_data
        .lines()
        .collect();
    let mut answer: u64 = 0;
    for line in grid.iter().enumerate() {
        for char in (*line.1).chars().enumerate() {
            if has_two_mas(&grid, char.0, line.0,) {
                answer += 1;
            }
        }
    }
    return Ok(answer);
}

fn print_result(prefix: &str, result: Result<u64, &str>) {
    match result {
        Ok(num) => println!("{}: {}", prefix, num),
        Err(reason) => println!("{}: {}", prefix, reason),
    }
}

fn main() {
    let argv: Vec<String> = args().collect();

    let input_file_path = argv.get(1)
        .expect("missing file name argument");
    let input_file = fs::canonicalize(input_file_path)
        .expect("Could not find and cannonicalize input file");

    println!("Using input file {:?}", input_file);

    let input_data = fs::read_to_string(input_file)
        .expect("Couldn't read input file, need to do that to run code.");

    let part_one_output = part_one(&input_data);
    let part_two_output = part_two(&input_data);

    print_result("Part 1", part_one_output);
    print_result("Part 2", part_two_output);
}

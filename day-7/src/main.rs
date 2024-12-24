use std::env::args;
use std::fs;
// use std::ops::Mul;
// use std::ops::Add;
use std::str::FromStr;

// trait Operator {
//     fn run(&self, op1: i64, op2: i64) -> i64;
// }
// 
// enum Operators {
//     Add,
//     Multiply,
// }
// 
// impl Operators {
// }
// 
// impl Operator for Operators {
//     fn run(&self, op1: i64, op2: i64) -> i64 {
//         return match self {
//             Self::Add => op1 + op2,
//             Self::Multiply => op1 * op2,
//         }
//     }
// }

struct Calibration {
    target: i64,
    operands: Vec<i64>,
}

#[derive(Debug)]
enum CalibrationParseError {
    IncorrectSeparatorNumber,
    ParseIntError,
}

impl FromStr for Calibration {
    type Err = CalibrationParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<&str> = s.split(": ").collect();
        if split.len() != 2 {
            return Err(CalibrationParseError::IncorrectSeparatorNumber);
        }

        let target: i64 = match split[0].parse::<i64>() {
            Ok(i) => i,
            Err(_) => return Err(CalibrationParseError::ParseIntError),
        };

        let operands_result = split[1].split_whitespace()
            .map(|x| x.parse::<i64>())
            .collect();

        let operands = match operands_result {
            Ok(ops) => ops,
            Err(_) => return Err(CalibrationParseError::ParseIntError),
        };

        return Ok(Calibration{target, operands});
    }
}

impl Calibration {

    // TODO make this call a recursive function _is_valid(&self, index, target), drop num adds
    fn is_valid(&self, num_adds: usize) -> Option<i64> {
        // println!("doing {}", self.target);
        if num_adds > self.operands.len() {
            let product: i64 = self.operands.iter().product();
            if  product >= self.target {
                println!("miss: {}: {:?}", self.target, self.operands);
            }
            return None
        }

        let mut target = self.target.clone();
        let iter = self.operands.iter().rev().enumerate();
        let mut add_buffer = num_adds.clone();

        for (index, i) in iter {
            if *i == target && index == self.operands.len() - 1 {
                // println!("hit {}", self.target);
                return Some(self.target);
            } else if target % i == 0 && add_buffer == 0 {
                // println!("{} divisible by {}", target, i);
                target /= i;
                // println!("now {}", target);
            } else {
                if add_buffer > 0 {
                    add_buffer -= 1;
                }
                // println!("{} not divisible by {}", target, i);
                target -= i;
                // println!("now {}", target);
            }
        }

        // println!("{} not valid: {}", self.target, target);
        return self.is_valid(num_adds + 1)
    }
}

fn get_data_structure(input_data: &str) -> Vec<Calibration> {
    return input_data
        .lines()
        .map(|calibration| calibration.parse::<Calibration>().expect("error parsing to Calibrations"))
        .collect();
}

fn part_one(input_data: &str) -> Result<u64, &str> {
    let calibrations = get_data_structure(input_data);

    let answer: u64 = calibrations.iter()
        .filter_map(|calibration| calibration.is_valid(0))
        .sum::<i64>() as u64;

    return Ok(answer);
}

fn part_two(input_data: &str) -> Result<u64, &str> {
    return Err("todo");
}

fn print_result(prefix: &str, result: Result<u64, &str>) {
    match result {
        Ok(num) => println!("{}: {}", prefix, num),
        Err(reason) => println!("{}: {}", prefix, reason),
    }
}

fn main() {
    let argv: Vec<String> = args().collect();

    let input_file_path = argv.get(1).expect("missing file name argument");
    let input_file =
        fs::canonicalize(input_file_path).expect("Could not find and cannonicalize input file");

    println!("Using input file {:?}", input_file);

    let input_data = fs::read_to_string(input_file)
        .expect("Couldn't read input file, need to do that to run code.");

    let part_one_output = part_one(&input_data);
    let part_two_output = part_two(&input_data);

    print_result("Part 1", part_one_output);
    print_result("Part 2", part_two_output);
}

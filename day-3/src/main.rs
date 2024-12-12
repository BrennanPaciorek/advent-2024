use regex::Captures;
use regex::Regex;
use std::env::args;
use std::fs;

fn part_one(input_data: &str) -> Result<u64, &str> {
    let regex =
        Regex::new(r"mul\((?<x>[0-9]{1,3}),(?<y>[0-9]{1,3})\)").expect("Error part 1 regex");
    let mut answer: u64 = 0;
    for line in input_data.lines() {
        answer += regex
            .captures_iter(line)
            .map(|cap| cap["x"].parse::<u64>().unwrap() * cap["y"].parse::<u64>().unwrap())
            .sum::<u64>();
    }
    return Ok(answer);
}

enum Instruction {
    Do,
    Dont,
    Mult(u64),
}

fn parse_instruction(cap: &Captures) -> Instruction {
    if let Some(_) = cap.name("do") {
        return Instruction::Do;
    } else if let Some(_) = cap.name("dont") {
        return Instruction::Dont;
    } else {
        return Instruction::Mult(
            cap["x"].parse::<u64>().unwrap() * cap["y"].parse::<u64>().unwrap(),
        );
    }
}

fn part_two(input_data: &str) -> Result<u64, &str> {
    let regex =
        Regex::new(r"(mul\((?<x>[0-9]{1,3}),(?<y>[0-9]{1,3})\))|(?<do>do\(\))|(?<dont>don't\(\))")
            .expect("Error part 2 regex");
    let mut enabled = true;
    let mut answer: u64 = 0;
    for line in input_data.lines() {
        for cap in regex.captures_iter(line) {
            match parse_instruction(&cap) {
                Instruction::Do => {
                    enabled = true;
                }
                Instruction::Dont => {
                    enabled = false;
                }
                Instruction::Mult(n) => {
                    if enabled {
                        answer += n;
                    }
                }
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

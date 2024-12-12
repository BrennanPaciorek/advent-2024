use std::env::args;
use std::fs;

// TODO: can make this use a base iter, so we can feed an iterator without nth element for part 2,
// allowing us to only clone the iterator in try_is_safe
// TODO: return only 0 or 1 like before, refactor solution code to consume the more reasonable return types
fn is_safe(vec: &mut Vec<u64>) -> Result<u64, usize> {
    let mut peek_iter = vec.iter().enumerate().peekable();

    let first = peek_iter.next().expect("need first number");
    let peek = *peek_iter.peek().expect("need second number");
    if *(first.1) == *(peek.1) {
        return Err(peek.0);
    }
    let first_diff = first.1.abs_diff(*(peek.1));
    if !(first_diff >= 1 && first_diff <= 3) {
        return Err(peek.0);
    }
    let ascending = *(first.1) < *(peek.1);

    while let Some(curr) = peek_iter.next() {
        match peek_iter.peek() {
            Some(p) => {
                if (*(curr.1) < *(p.1)) != ascending {
                    return Err(curr.0);
                }

                let diff = curr.1.abs_diff(*(p.1));

                if !(diff >= 1 && diff <= 3) {
                    return Err(p.0);
                }
            }
            None => {}
        };
    }
    return Ok(1);
}

fn try_is_safe(vec: &mut Vec<u64>) -> u64 {
    for i in 0..vec.len() {
        let mut vec_clone = vec.clone();
        vec_clone.remove(i);
        match is_safe(&mut vec_clone) {
            Ok(_) => return 1,
            Err(_) => {}
        };
    }
    return 0;
}

// Ascending or decending order, safe if difference is 1-3, unsafe otherwise, sum up num safe
fn part_one(input_data: &str) -> Result<u64, &str> {
    let mut reports: Vec<Vec<u64>> = Vec::new();
    for line in input_data.lines() {
        let nums = line
            .split_whitespace()
            .map(|s| s.parse::<u64>().expect("expected number"))
            .collect();
        reports.push(nums);
    }

    let answer: u64 = reports.iter_mut().map(|x| is_safe(x).unwrap_or(0)).sum();

    return Ok(answer);
}

fn part_two(input_data: &str) -> Result<u64, &str> {
    let mut reports: Vec<Vec<u64>> = Vec::new();
    for line in input_data.lines() {
        let nums = line
            .split_whitespace()
            .map(|s| s.parse::<u64>().expect("expected number"))
            .collect();
        reports.push(nums);
    }

    let answer: u64 = reports.iter_mut().map(|x| try_is_safe(x)).sum();

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

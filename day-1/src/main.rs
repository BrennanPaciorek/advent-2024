use std::env::args;
use std::fs;

// Wrapper for unwrap_unchecked on binary_search, since it gets the wanted index
fn get_insert_index(vec: &mut Vec<u64>, num: &u64) -> usize {
    match vec.binary_search(num) {
        Ok(i) => return i,
        Err(i) => return i,
    }
}

fn part_one(input_data: &str) -> Result<u64, &str> {
    let mut left_list = Vec::new();
    let mut right_list = Vec::new();
    for line in input_data.lines() {
        let mut split = line.split_whitespace();

        let left: u64 = split
            .next()
            .expect("Could not get left number string")
            .parse()
            .expect("Could not parse left number string");
        let right: u64 = split
            .next()
            .expect("Could not get right number string")
            .parse()
            .expect("Could not parse right number string");

        let left_index = get_insert_index(&mut left_list, &left);
        let right_index = get_insert_index(&mut right_list, &right);

        left_list.insert(left_index, left);
        right_list.insert(right_index, right);
    }
    let answer: u64 = left_list
        .iter()
        .zip(right_list.iter())
        .map(|(x, y)| x.abs_diff(*y))
        .sum();
    return Ok(answer);
}

fn part_two(input_data: &str) -> Result<u64, &str> {
    let mut left_list = Vec::new();
    let mut right_list = Vec::new();
    for line in input_data.lines() {
        let mut split = line.split_whitespace();

        let left: u64 = split
            .next()
            .expect("Could not get left number string")
            .parse()
            .expect("Could not parse left number string");
        let right: u64 = split
            .next()
            .expect("Could not get right number string")
            .parse()
            .expect("Could not parse right number string");

        let left_index = get_insert_index(&mut left_list, &left);
        let right_index = get_insert_index(&mut right_list, &right);

        left_list.insert(left_index, left);
        right_list.insert(right_index, right);
    }
    let mut answer: u64 = 0;
    for item in left_list.iter() {
        answer += right_list.iter().filter(|x| **x == *item).count() as u64 * item;
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

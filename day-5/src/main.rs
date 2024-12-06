use std::env::args;
use std::{fs, iter};
use std::str::FromStr;

// Input format
// X|Y
// ...
//
// X1,X2,X3
// ...
enum ParserState {
    ParsingRules,
    ParsingUpdates,
}
type Update = Vec<u64>;
struct Rule(u64, u64);

impl Rule {
    fn update_passes(&self, update: &Update) -> Option<(usize, usize)> {
        let lower = update.iter().enumerate().find(|x| *x.1 == self.0);
        let upper = update.iter().enumerate().find(|x| *x.1 == self.1);
        match (lower, upper) {
            (Some(l), Some(u)) => {
                if l.0 > u.0 {
                    return Some((l.0, u.0))
                }
                return None
            },
            _ => return None
        };
    }
}

impl FromStr for Rule {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split("|")
            .map(|x| x.parse::<u64>().expect("parse int fail"));

        let l = iter.next().expect("Rule::from_str: lower bound not found");
        let u = iter.next().expect("Rule::from_str: upper bound not found");

        return Ok(Self(l, u))
    }
}

// X|Y are page rules
// if page X and page Y produced as result of update, X < Y
// X1,X2,X3 is an update
// Sum the middle of correctly ordered updates
fn part_one(input_data: &str) -> Result<u64, &str> {
    let mut rules: Vec<Rule> = Vec::new();
    let mut answer: u64 = 0;
    let mut state = ParserState::ParsingRules;

    for line in input_data.lines() {
        match &state {
            ParserState::ParsingRules => {
                if line.len() == 0 {
                    state = ParserState::ParsingUpdates;
                } else {
                    rules.push(Rule::from_str(line).expect("encountered an error when parsing a rule"));
                }
            },
            ParserState::ParsingUpdates => {
                let update: Update = line.split(",")
                    .map(|n| n.parse::<u64>().expect("Parse int fail when serializing update"))
                    .collect();

                if rules.len() == rules.iter().filter(|rule| rule.update_passes(&update).is_none()).count() {
                    answer += *update.get(update.len() / 2)
                        .unwrap_or(&0);
                }
            }
        };
    }
    return Ok(answer);
}

fn part_two(input_data: &str) -> Result<u64, &str>{
    let mut rules: Vec<Rule> = Vec::new();
    let mut answer: u64 = 0;
    let mut state = ParserState::ParsingRules;

    for line in input_data.lines() {
        match &state {
            ParserState::ParsingRules => {
                if line.len() == 0 {
                    state = ParserState::ParsingUpdates;
                } else {
                    rules.push(Rule::from_str(line).expect("encountered an error when parsing a rule"));
                }
            },
            ParserState::ParsingUpdates => {
                let mut update: Update = line.split(",")
                    .map(|n| n.parse::<u64>().expect("Parse int fail when serializing update"))
                    .collect();

                // Switch for determining when to terminate loop
                let mut rules_pass: bool = false;
                // Switch for determining if update was once incorrecly ordred by end of loop
                let mut missed_once: bool = false;
                while !rules_pass {
                    rules_pass = true;
                    for rule in rules.iter() {
                        if let Some((l, u)) = rule.update_passes(&update) {
                            update.swap(l, u);
                            missed_once = true;
                            rules_pass = false;
                        }
                    }
                }
                if missed_once {
                    answer += *update.get(update.len() / 2)
                       .unwrap_or(&0);
                }
            }
        };
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

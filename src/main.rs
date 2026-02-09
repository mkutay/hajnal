use std::env;
use std::fs;

mod dpll;

fn parse_benchmark(lines: std::str::Lines) -> (i64, i64, Vec<Vec<i64>>) {
    let mut num_variables = 0;
    let mut num_clauses = 0;
    let mut clauses: Vec<Vec<i64>> = Vec::new();

    for line in lines {
        if line.starts_with("p") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            num_variables = parts[2].parse::<i64>().unwrap();
            num_clauses = parts[3].parse::<i64>().unwrap();
        } else if !line.starts_with("c") {
            let literals = line
                .split_whitespace()
                .filter_map(|s| s.parse::<i64>().ok())
                .collect();
            clauses.push(literals);
        }
    }

    assert_eq!(
        num_clauses,
        clauses.len() as i64,
        "The number of clauses in the file should be equal to the given parameters."
    );

    (num_variables, num_clauses, clauses)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).expect("Failed to parse arguments.");

    let contents = fs::read_to_string(config.file_path).expect("File path should exist.");
    let (num_variables, num_clauses, mut clauses) = parse_benchmark(contents.lines());

    println!("{} {} {:?}", num_variables, num_clauses, clauses);

    println!("result: {}", dpll::dpll(&mut clauses));
}

struct Config {
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("Not enough arguments. Usage: <program> <file_path>");
        }

        let file_path = args[1].clone();
        Ok(Config { file_path })
    }
}

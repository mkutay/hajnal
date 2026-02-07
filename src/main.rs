use std::fs;

#[derive(Debug)]
struct Clause {
    pub literals: Vec<i64>,
}

fn parse_benchmark(lines: std::str::Lines) -> (i64, i64, Vec<Clause>) {
    let mut num_variables = 0;
    let mut num_clauses = 0;
    let mut clauses: Vec<Clause> = Vec::new();

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
            clauses.push(Clause { literals });
        }
    }

    assert_eq!(
        num_clauses,
        clauses.len() as i64,
        "The number of clauses in the file should be equal to the given config."
    );

    (num_variables, num_clauses, clauses)
}

fn main() {
    let file_path = String::from("./benchmarks/sample.cnf");
    let contents = fs::read_to_string(file_path).expect("File path should exist.");
    let (num_variables, num_clauses, clauses) = parse_benchmark(contents.lines());

    println!("{} {} {:?}", num_clauses, num_variables, clauses);
}

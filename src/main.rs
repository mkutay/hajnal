use std::{collections::HashSet, fs};

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

fn get_unit_clause(clauses: &Vec<Vec<i64>>) -> Option<i64> {
    clauses.iter().find(|c| c.len() == 1).map(|c| c[0])
}

fn unit_propogate(clauses: &mut Vec<Vec<i64>>, lit: i64) {
    clauses.retain(|c| !c.contains(&lit));

    for clause in clauses.iter_mut() {
        clause.retain(|&l| l != -lit);
    }
}

fn get_pure_literals(clauses: &Vec<Vec<i64>>) -> Vec<i64> {
    let mut positive_literals = HashSet::new();
    let mut negative_literals = HashSet::new();

    for clause in clauses {
        for literal in clause {
            if *literal < 0 {
                negative_literals.insert(-literal);
            } else {
                positive_literals.insert(*literal);
            }
        }
    }

    let pure_positive = positive_literals
        .difference(&negative_literals)
        .copied()
        .collect::<Vec<i64>>();
    let pure_negative = negative_literals
        .difference(&positive_literals)
        .map(|l| -(*l))
        .collect::<Vec<i64>>();

    pure_positive
        .into_iter()
        .chain(pure_negative.into_iter())
        .collect()
}

fn assign_pure_literals(clauses: &mut Vec<Vec<i64>>, pure_literals: Vec<i64>) {
    clauses.retain(|c| {
        let mut is_contained = false;
        for lit in c {
            if pure_literals.contains(lit) {
                is_contained = true;
                break;
            }
        }
        !is_contained
    });
}

fn choose_literal(clauses: &Vec<Vec<i64>>) -> i64 {
    clauses[0][0]
}

fn dpll(clauses: &mut Vec<Vec<i64>>) -> bool {
    while let Some(lit) = get_unit_clause(clauses) {
        unit_propogate(clauses, lit);
    }

    let pure_literals = get_pure_literals(clauses);
    assign_pure_literals(clauses, pure_literals);

    if clauses.is_empty() {
        return true;
    }

    if clauses.iter().any(|c| c.is_empty()) {
        return false;
    }

    let lit = choose_literal(clauses);

    let mut add_positive = clauses.clone();
    add_positive.push(vec![lit]);

    let mut add_negative = clauses.clone();
    add_negative.push(vec![-lit]);

    dpll(&mut add_positive) || dpll(&mut add_negative)
}

fn main() {
    let file_path = String::from("./benchmarks/sample.cnf");
    let contents = fs::read_to_string(file_path).expect("File path should exist.");
    let (num_variables, num_clauses, mut clauses) = parse_benchmark(contents.lines());

    println!("{} {} {:?}", num_variables, num_clauses, clauses);

    println!("result: {}", dpll(&mut clauses));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_clause_some() {
        let clauses: Vec<Vec<i64>> = vec![vec![1, 2], vec![1], vec![2, 3]];
        let unit_clause = get_unit_clause(&clauses);
        assert!(unit_clause.is_some());
    }

    #[test]
    fn unit_clause_none() {
        let clauses: Vec<Vec<i64>> = vec![vec![1, 2], vec![1, 3], vec![2, 3]];
        let unit_clause = get_unit_clause(&clauses);
        assert!(unit_clause.is_none());
    }

    #[test]
    fn does_unit_propogation() {
        let mut clauses: Vec<Vec<i64>> = vec![vec![1, 2], vec![1], vec![2, 3]];
        unit_propogate(&mut clauses, 1);
        assert_eq!(clauses, vec![vec![2, 3]]);
    }

    #[test]
    fn does_unit_propogation_negation() {
        let mut clauses: Vec<Vec<i64>> = vec![vec![1, 2], vec![1], vec![2, 3], vec![2, -1]];
        unit_propogate(&mut clauses, 1);
        assert_eq!(clauses, vec![vec![2, 3], vec![2]]);
    }

    #[test]
    fn gets_pure_literals() {
        let clauses: Vec<Vec<i64>> = vec![vec![1, 2], vec![1], vec![2, 3], vec![2, -1]];
        let pure_literals: HashSet<i64> = HashSet::from_iter(get_pure_literals(&clauses));
        assert_eq!(pure_literals, HashSet::from_iter(vec![2, 3]));
    }

    #[test]
    fn assigns_pure_literals() {
        let mut clauses: Vec<Vec<i64>> = vec![vec![1, 2], vec![1], vec![2, 3], vec![2, -1]];
        assign_pure_literals(&mut clauses, vec![2, 3]);
        assert_eq!(clauses, vec![vec![1]]);
    }
}

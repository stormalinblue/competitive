use std::error::Error;
use std::io::{self, BufRead};

#[derive(Debug)]
struct Problem {
    numbers: Vec<u8>,
}

fn parse_input() -> Result<Vec<Problem>, Box<dyn Error>> {
    let mut lines = io::stdin().lock().lines();

    let num_problems = lines
        .next()
        .expect("Should have num problems")?
        .parse::<usize>()?;

    let mut results: Vec<Problem> = Vec::with_capacity(num_problems);
    for _ in 0..num_problems {
        let _ = lines
            .next()
            .expect("Should have length")?
            .parse::<usize>()?;

        results.push(Problem {
            numbers: lines
                .next()
                .expect("Should have numbers")?
                .split_ascii_whitespace()
                .map(|x| x.parse::<u8>().expect("Should be number"))
                .collect(),
        })
    }
    Ok(results)
}

fn solve_problem(problem: &Problem) -> &'static str {
    let lines = &problem.numbers;

    match (lines[0], lines[lines.len() - 1]) {
        (0, 0) => "Bob",
        (_, _) => "Alice",
    }
}

fn main() {
    // I looked at the editorial for this

    let input = parse_input().unwrap();

    for problem in input {
        println!("{}", solve_problem(&problem));
    }
}

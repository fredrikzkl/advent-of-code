use std::fs::File;
use std::io::{BufRead, BufReader};

mod math_problem;
use math_problem::MathProblem;

fn main() {
    let file = File::open("data.txt");
    let reader = BufReader::new(file.unwrap());

    let mut problems: Vec<MathProblem> = Vec::new();

    for linte in reader.lines() {
        let line = linte.unwrap();

        let symbols = parse_line(line.as_str());

        for (index, symbol) in symbols.iter().enumerate() {
            if problems.len() <= index {
                problems.push(MathProblem::new());
            }

            if let Ok(number) = symbol.parse::<u64>() {
                problems[index].add_number(number);
            }
            if let Ok(operator_char) = symbol.parse::<char>() {
                if operator_char == '+' || operator_char == '*' {
                    problems[index].set_operator(operator_char);
                }
            }
        }
    }

    let mut total_sum = 0;
    for problm in problems.iter() {
        problm.print_problem();
        total_sum += problm.calculate();
    }

    println!("Total Sum: {}", total_sum);
}

fn parse_line(line: &str) -> Vec<String> {
    println!("Parsing Line: {}", line);
    let mut result: Vec<String> = Vec::new();

    let mut entry = String::new();
    for c in line.chars() {
        if c == ' ' {
            if !entry.is_empty() {
                result.push(entry.clone());
            }
            entry.clear();
            continue;
        }

        entry.push(c);
    }

    if !entry.is_empty() {
        result.push(entry.clone());
    }

    println!("Parsed Line: {:?}\n", result);
    result
}

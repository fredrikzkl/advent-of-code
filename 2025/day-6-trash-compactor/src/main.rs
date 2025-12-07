use std::fs::File;
use std::io::{BufRead, BufReader, Seek};

mod math_problem;
use math_problem::MathProblem;

mod cephalopod;
use cephalopod::Cephalopod;

fn main() {
    let file = File::open("data.txt");
    let mut reader = BufReader::new(file.unwrap());

    solve_part_one(&mut reader);

    reader.rewind().unwrap();
    solve_part_two(&mut reader);
}

fn solve_part_two(reader: &mut BufReader<File>) {
    println!("--- Part Two ---");
    let mut cephalopods: Cephalopod = Cephalopod::new();

    for linte in reader.lines() {
        let line = linte.unwrap();
        cephalopods.read_line(line.as_str());
    }

    cephalopods.build_problems();
    cephalopods.print_problems();
    println!("Total Sum: {}\n", cephalopods.calculate_total());
}

fn solve_part_one(reader: &mut BufReader<File>) {
    println!("--- Part One ---");
    let mut problems: Vec<MathProblem> = Vec::new();

    for linte in reader.lines() {
        let line = linte.unwrap();

        let entries = parse_line(line.as_str());

        for (index, symbol) in entries.iter().enumerate() {
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

    println!("Total Sum: {}\n", total_sum);
}

fn parse_line(line: &str) -> Vec<String> {
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

    result
}

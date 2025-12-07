use crate::math_problem::MathProblem;

pub struct Cephalopod {
    char_matrix: Vec<Vec<char>>,
    problems: Vec<MathProblem>,
    height: usize,
    width: usize,
}

impl Cephalopod {
    pub fn new() -> Self {
        Cephalopod {
            char_matrix: Vec::new(),
            problems: Vec::new(),
            height: 0,
            width: 0,
        }
    }

    pub fn build_problems(&mut self) {
        let mut current_column = self.width - 1;

        let mut current_problem = MathProblem::new();
        loop {
            let mut current_number_str = String::new();
            for row in 0..self.char_matrix.len() {
                let current_char = self.char_matrix[row][current_column].to_string();

                if current_char.parse::<u64>().is_ok() {
                    println!(
                        "Pushing char: {}, current: {}",
                        current_char, current_number_str
                    );
                    current_number_str.push_str(&current_char);
                    continue;
                }

                // If we hit a + or *, we know that the math problem has ended
                if let Ok(operator_char) = current_char.parse::<char>() {
                    if operator_char == '+' || operator_char == '*' {
                        println!("Setting operator: {}", operator_char);
                        current_problem.set_operator(operator_char);
                    }
                }
            }

            if let Ok(number) = current_number_str.parse::<u64>() {
                println!("Adding number: {}", number);
                current_problem.add_number(number);
            }

            if current_problem.has_oerator() {
                println!("Finished problem:");
                self.problems.push(current_problem);
                current_problem = MathProblem::new();
            }

            if current_column == 0 {
                break;
            }
            current_column -= 1;
        }
    }

    pub fn calculate_total(&self) -> u64 {
        let mut total = 0;
        for problem in self.problems.iter() {
            total += problem.calculate();
        }
        total
    }

    pub fn read_line(&mut self, line: &str) {
        let chars: Vec<char> = line.chars().collect();
        self.char_matrix.push(chars);
        self.height = self.height + 1;
        self.width = line.len();
    }

    pub fn print_problems(&self) {
        println!("\n--- Problems ---");
        for problem in self.problems.iter() {
            problem.print_problem();
        }
    }
}

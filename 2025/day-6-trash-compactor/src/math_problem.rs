pub struct MathProblem {
    numbers: Vec<u64>,
    operator: char,
}

impl MathProblem {
    pub fn new() -> Self {
        MathProblem {
            numbers: Vec::new(),
            operator: '+',
        }
    }

    pub fn calculate(&self) -> u64 {
        if self.operator == '+' {
            self.numbers.iter().sum()
        } else if self.operator == '*' {
            self.numbers.iter().product()
        } else {
            panic!("Unsupported operator");
        }
    }

    pub fn add_number(&mut self, number: u64) {
        self.numbers.push(number);
    }

    pub fn set_operator(&mut self, operator: char) {
        self.operator = operator;
    }

    pub fn print_problem(&self) {
        print!("Problem: ");
        for (index, number) in self.numbers.iter().enumerate() {
            if index > 0 {
                print!(" {} ", self.operator);
            }
            print!("{}", number);
        }
        println!();
    }
}

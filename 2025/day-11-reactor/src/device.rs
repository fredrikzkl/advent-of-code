#[derive(Clone, PartialEq)]
pub struct Device {
    pub input: String,
    pub output: Vec<String>,
}

impl Device {
    pub fn new(line: &str) -> Device {
        let split = line.split(":").collect::<Vec<&str>>();

        Device {
            input: split[0].to_string(),
            output: Self::parse_output(split[1]),
        }
    }

    pub fn is_out(&self) -> bool {
        if self.output.len() == 1 && self.output[0] == "out" {
            return true;
        }
        false
    }

    pub fn parse_output(raw_output: &str) -> Vec<String> {
        raw_output
            .split(" ")
            .filter(|s| !s.is_empty())
            .map(|s| s.trim().to_string())
            .collect()
    }

    pub fn print(&self) {
        println!("Input: {}, Output: {:?}", self.input, self.output);
    }
}

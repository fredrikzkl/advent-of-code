pub struct BatteryBank {
    batteries: String,
    highest_tens: u8,
    highest_ones: u8,
    highest_joltage: u16,
}

impl BatteryBank {
    pub fn new(batteries: String) -> Self {
        BatteryBank {
            batteries,
            highest_tens: 0,
            highest_ones: 0,
            highest_joltage: 0,
        }
    }

    pub fn calculate_highest_joltage(&mut self) {
        let mut index_of_highest_tens = 0;
        self.batteries
            .chars()
            .enumerate()
            .for_each(|(i, c): (usize, char)| {
                if i == self.batteries.len() - 1 {
                    return;
                }
                let candidate = c.to_digit(10).unwrap() as u8;
                if candidate > self.highest_tens {
                    self.highest_tens = candidate;
                    index_of_highest_tens = i;
                }
            });

        self.batteries
            .chars()
            .enumerate()
            .for_each(|(i, c): (usize, char)| {
                if i <= index_of_highest_tens {
                    return;
                }

                let candidate = c.to_digit(10).unwrap() as u8;
                if candidate > self.highest_ones {
                    self.highest_ones = candidate;
                }
            });

        let highest_voltage_str = format!("{}{}", self.highest_tens, self.highest_ones);
        self.highest_joltage = highest_voltage_str.parse::<u16>().unwrap();
    }

    pub fn joltage(&self) -> u16 {
        self.highest_joltage
    }
}

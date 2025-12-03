pub struct BatteryBank {
    batteries: String,
    highest_joltage: u64,
}

impl BatteryBank {
    pub fn new(batteries: String) -> Self {
        BatteryBank {
            batteries,
            highest_joltage: 0,
        }
    }

    pub fn calculate_highest_joltage(&mut self, size: i8) {
        let mut battery_list: Vec<u8> = Vec::new();
        let mut startindex: usize = 0;

        for i in 0..size {
            let max = (self.batteries.len() - size as usize + 1) + i as usize;
            let (next_battery, next_battery_index) = self.get_next_battery(startindex, max);

            battery_list.push(next_battery);
            startindex = next_battery_index + 1;
        }

        let battery_string = battery_list
            .iter()
            .map(|b| b.to_string())
            .collect::<String>();

        self.highest_joltage = battery_string.parse::<u64>().unwrap();
    }

    fn get_next_battery(&self, start_index: usize, end_index: usize) -> (u8, usize) {
        let mut highest: u8 = 0;
        let mut highest_index: usize = start_index;

        for i in start_index..end_index {
            let candidate = self.batteries.chars().nth(i).unwrap().to_digit(10).unwrap() as u8;
            if candidate > highest {
                highest = candidate;
                highest_index = i;
            }
        }
        (highest, highest_index)
    }

    pub fn joltage(&self) -> u64 {
        self.highest_joltage
    }
}

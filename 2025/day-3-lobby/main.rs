use std::fs::File;
use std::io::{BufRead, BufReader};

mod battery_bank;
use battery_bank::BatteryBank;

fn main() {
    let file = File::open("data/input.txt");
    let reader = BufReader::new(file.unwrap());

    let mut total_joltage: i16 = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        println!("{}", line);
        let mut battery = BatteryBank::new(line);

        battery.calculate_highest_joltage();
        println!("Highest joltage: {}", battery.joltage());
        total_joltage = total_joltage + battery.joltage() as i16;
    }
    println!("Total joltage: {}", total_joltage);
}

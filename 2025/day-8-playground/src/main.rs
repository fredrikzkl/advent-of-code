use std::fs::File;
use std::io::{BufRead, BufReader};

mod circuits;
use circuits::Circuits;

fn main() {
    let file = File::open("data.txt");
    let reader = BufReader::new(file.unwrap());

    let mut circuits = Circuits::new();

    for (idx, linte) in reader.lines().enumerate() {
        let line = linte.unwrap();
        circuits.add_box(idx, line);
    }

    for _ in 0..=10 {
        circuits.find_closest_circuit();
    }

    circuits.sort_by_size();

    circuits.print_circuits();
    println!("Circuit Product: {}", circuits.product(3));
}

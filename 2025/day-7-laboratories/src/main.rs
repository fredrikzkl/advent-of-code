use std::fs::File;
use std::io::{BufRead, BufReader};

mod laboratory;
use laboratory::Laboriatory;

fn main() {
    let file = File::open("data.txt");
    let reader = BufReader::new(file.unwrap());

    let mut laboratory = Laboriatory::new();

    for line in reader.lines() {
        laboratory.add_line(&line.unwrap());
    }

    laboratory.solve_diagram();
    laboratory.print_map();
    println!("Beam count: {}", laboratory.count_beams());
    println!("Split count: {}", laboratory.get_splits());
}

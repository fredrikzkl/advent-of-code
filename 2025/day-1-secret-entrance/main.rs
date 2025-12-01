use std::fs::File;
use std::io::{BufRead, BufReader};

mod lock;
use lock::Lock;

fn main() {
    let mut lock = Lock::new();

    let file = File::open("data/code.txt");
    let reader = BufReader::new(file.unwrap());

    for line in reader.lines() {
        let line = line.unwrap();
        lock.dial(&line);
    }

    lock.print_password();
}

use std::fs::File;
use std::io::{BufRead, BufReader};

mod point;
mod rectangle;
mod theater;

use theater::Theater;

fn main() {
    let file = File::open("example.txt");
    let reader = BufReader::new(file.unwrap());

    let mut theater = Theater::new();

    for linte in reader.lines() {
        let line = linte.unwrap();
        theater.add_point(line);
    }

    theater.generate_rectangles();
    theater.sort_rectangles();

    theater.rectangles().last().unwrap().print()
}

use std::fs::File;
use std::io::{BufRead, BufReader};

mod map;
mod point;
mod rectangle;
mod theater;

use map::Map;
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

    let map = Map::new(theater.corners());
    map.print();

    // theater.rectangles().last().unwrap().print()
}

use std::fs::File;
use std::io::{BufRead, BufReader};

mod map;
mod point;
mod rectangle;
mod theater;

use map::Map;
use theater::Theater;

fn main() {
    let file = File::open("data.txt");
    let reader = BufReader::new(file.unwrap());

    let mut theater = Theater::new();

    for linte in reader.lines() {
        let line = linte.unwrap();
        theater.add_point(line);
    }

    println!("Starting creating map...");
    let map = Map::new(theater.corners());
    println!("Map created!");

    map.print();
    map.write_to_file("data_map.txt").unwrap();

    println!("Generating rectangles...");
    theater.generate_rectangles();
    theater.sort_rectangles();

    // Goes through all rectangles and print the valid ones
    println!("Valid rectangles:");
    for rect in theater.rectangles() {
        if map.is_valid(rect) {
            rect.print();
            break;
        }
    }

    // theater.rectangles().last().unwrap().print()
}

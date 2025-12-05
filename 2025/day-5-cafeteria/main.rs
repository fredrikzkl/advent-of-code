use std::fs::File;
use std::io::{BufRead, BufReader};

mod fresh_list;
use fresh_list::FreshList;

fn main() {
    let file = File::open("data/puzzle.txt");
    let reader = BufReader::new(file.unwrap());

    let mut fresh_list = FreshList::new();
    let mut read_fresh_range: bool = true;
    let mut fresh_count: u64 = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            read_fresh_range = false;
            continue;
        }

        if read_fresh_range {
            fresh_list.add_range(&line);
        } else {
            // Read food ID
            let food_id: u64 = line.parse().unwrap();
            if fresh_list.is_fresh(food_id) {
                fresh_count += 1;
            }
        }
    }

    println!("Number of fresh food items: {}", fresh_count);
}

use std::fs::File;
use std::io::{BufRead, BufReader};

mod scroll_grid;
use scroll_grid::ScrollGrid;

fn main() {
    let file = File::open("data/example.txt");
    let reader = BufReader::new(file.unwrap());

    let mut scroll_grid = ScrollGrid::new();

    for line in reader.lines() {
        scroll_grid.add_line(&line.unwrap());
    }

    let valid_scrolls = scroll_grid.calculate_valid_scrolls();
    println!("{}", valid_scrolls);

    scroll_grid.print_grid();
}

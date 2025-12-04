use std::fs::File;
use std::io::{BufRead, BufReader};

mod scroll_grid;
use scroll_grid::ScrollGrid;

fn main() {
    let file = File::open("data/input.txt");
    let reader = BufReader::new(file.unwrap());

    let mut scroll_grid = ScrollGrid::new();

    for line in reader.lines() {
        scroll_grid.add_line(&line.unwrap());
    }

    let mut total_valid_scrolls = 0;
    loop {
        let valid_scrolls: usize = scroll_grid.calculate_valid_scrolls();
        scroll_grid.print_grid();
        if valid_scrolls == 0 {
            break;
        }
        total_valid_scrolls += valid_scrolls;
        scroll_grid.clean_all_marked_scrolls();
    }

    scroll_grid.print_grid();
    println!("Total Valid Scrolls: {}", total_valid_scrolls);
}

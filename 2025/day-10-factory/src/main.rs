use std::fs::File;
use std::io::{BufRead, BufReader};

mod button;
mod machine;
mod state;
mod tree;

use machine::Machine;
use tree::Tree;

fn main() {
    let file = File::open("example.txt");
    let reader = BufReader::new(file.unwrap());

    let mut machines: Vec<Machine> = Vec::new();

    for linte in reader.lines() {
        let line = linte.unwrap();
        let new_machine = Machine::new(&line);
        new_machine.print();
        machines.push(new_machine);
    }

    return;

    let mut total_cycles = 0;
    for machine in machines.iter() {
        let mut tree = Tree::new(machine.clone());
        let result = tree.search();
        total_cycles += result;
    }

    println!("Total cycles: {}", total_cycles);
}

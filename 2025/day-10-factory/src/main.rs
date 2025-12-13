use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

mod button;
mod jolt_state;
mod jolt_tree;
mod machine;
mod state;
mod tree;

use jolt_tree::JoltTree;
use machine::Machine;
use tree::Tree;

fn main() {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("data.txt");
    let file = File::open(path);
    let reader = BufReader::new(file.unwrap());

    let mut machines: Vec<Machine> = Vec::new();

    for linte in reader.lines() {
        let line = linte.unwrap();
        let new_machine = Machine::new(&line);
        new_machine.print();
        machines.push(new_machine);
    }

    solve_part_2(&machines);
}

fn solve_part_2(machines: &Vec<Machine>) {
    println!("\n-- Solving Part 2 -- \n");
    let mut min_depth = 0;
    for machine in machines.iter() {
        let mut jolt_tree = JoltTree::new(machine.clone());
        let result = jolt_tree.search();
        min_depth += result;
    }
    println!("Pt2: Total cycles: {}", min_depth);
}

fn solve_part_1(machines: &Vec<Machine>) {
    let mut min_depth = 0;
    for machine in machines.iter() {
        let mut tree = Tree::new(machine.clone());
        let result = tree.search();
        min_depth += result;
    }
    println!("Total cycles: {}", min_depth);
}

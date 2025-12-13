use std::fs::File;
use std::io::{BufRead, BufReader};

mod device;
mod tree;

use device::Device;
use tree::Tree;

fn main() {
    let file = File::open("data.txt");
    let reader = BufReader::new(file.unwrap());

    let mut devices: Vec<Device> = Vec::new();

    for linte in reader.lines() {
        let line = linte.unwrap();

        let new_device = Device::new(&line);
        new_device.print();
        devices.push(new_device);
    }

    let mut tree = Tree::new(devices);
    let solution = tree.grow_tree();
    println!("Solution: {}", solution);
}

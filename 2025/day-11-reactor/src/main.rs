use std::fs::File;
use std::io::{BufRead, BufReader};

mod device;
use device::Device;

fn main() {
    let file = File::open("example.txt");
    let reader = BufReader::new(file.unwrap());

    let mut devices: Vec<Device> = Vec::new();

    for linte in reader.lines() {
        let line = linte.unwrap();

        let new_device = Device::new(&line);
        new_device.print();
        devices.push(new_device);
    }
}

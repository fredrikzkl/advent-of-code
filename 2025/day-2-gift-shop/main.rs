mod id_cracker;
use id_cracker::IdCracker;
use std::fs;

fn main() {
    let content = fs::read_to_string("data/ids.txt").expect("Failed to read file");
    let entries: Vec<&str> = content.trim().split(',').collect();

    let mut valid_ids = Vec::<u64>::new();

    for entry in entries {
        let cracked_result = IdCracker::new(entry).crack_ids();
        for n in cracked_result.iter() {
            println!("{}", n);
        }
        valid_ids.extend(cracked_result);
    }

    let mut added_sum: u64 = 0;
    for id in valid_ids.iter() {
        added_sum += id;
    }
    println!("Sum of all valid IDs: {}", added_sum);
}

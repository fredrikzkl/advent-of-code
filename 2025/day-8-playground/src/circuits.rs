use std::collections::HashSet;

use nalgebra::Vector3;

pub struct Circuits {
    boxes: Vec<JunctionBox>,
    circuits: Vec<Circuit>,
    prev_closest: f64,
    resolved_pairs: HashSet<String>,
}

impl Circuits {
    pub fn new() -> Circuits {
        Circuits {
            boxes: Vec::new(),
            circuits: Vec::new(),
            prev_closest: 0.0,
            resolved_pairs: HashSet::new(),
        }
    }

    pub fn add_box(&mut self, idx: usize, line: String) {
        let coordinates = line.split(",").collect::<Vec<_>>();
        if coordinates.len() != 3 {
            panic!("Invalid box coordinates: {}", line);
        }

        self.boxes.push(JunctionBox::new(
            idx,
            Vector3::new(
                coordinates[0].parse::<f64>().unwrap(),
                coordinates[1].parse::<f64>().unwrap(),
                coordinates[2].parse::<f64>().unwrap(),
            ),
        ));
    }

    pub fn find_closest_circuit(&mut self) {
        let mut closest_distance = f64::MAX;
        let mut box_a: Option<&JunctionBox> = None;
        let mut box_b: Option<&JunctionBox> = None;

        for box_item in self.boxes.iter() {
            for other_box in self.boxes.iter() {
                if box_item.index == other_box.index {
                    continue;
                }

                if self
                    .resolved_pairs
                    .contains(&Self::get_pair_hash(box_item, other_box))
                {
                    continue;
                }

                if self.is_in_same_circuit(box_item, other_box) {
                    continue;
                }

                let distance = Self::distance(box_item, other_box);
                if distance <= closest_distance && distance >= self.prev_closest {
                    closest_distance = distance;
                    box_a = Some(box_item);
                    box_b = Some(other_box);
                }
            }
        }

        self.prev_closest = closest_distance;

        if box_a.is_none() || box_b.is_none() {
            return;
        }

        let mut new_circuelt = Circuit::new();
        new_circuelt.boxes.push(box_a.unwrap().clone());
        new_circuelt.boxes.push(box_b.unwrap().clone());

        self.resolved_pairs
            .insert(Self::get_pair_hash(box_a.unwrap(), box_b.unwrap()));

        // Check if the circuit already exists
        for circuit in self.circuits.iter_mut() {
            if circuit.try_connect_circuit(&new_circuelt) {
                return;
            }
        }

        self.circuits.push(new_circuelt);
    }

    pub fn print_circuits(&self) {
        for (idx, circuit) in self.circuits.iter().enumerate() {
            println!("-- Circuit {} (size: {}) --", idx + 1, circuit.boxes.len());
            for box_item in circuit.boxes.iter() {
                println!("{}: {}", box_item.index, box_item.position);
            }
        }
    }

    pub fn product(&self, include_count: usize) -> u64 {
        let mut product = 1u64;
        for circuit in self.circuits.iter().take(include_count) {
            product *= circuit.boxes.len() as u64;
        }
        product
    }

    fn distance(a: &JunctionBox, b: &JunctionBox) -> f64 {
        (a.position - b.position).norm() // Euclidean distance
    }

    pub fn sort_by_size(&mut self) {
        self.circuits
            .sort_by(|a, b| b.boxes.len().cmp(&a.boxes.len()));
    }

    fn get_pair_hash(a: &JunctionBox, b: &JunctionBox) -> String {
        if a.index < b.index {
            format!("{}-{}", a.index, b.index)
        } else {
            format!("{}-{}", b.index, a.index)
        }
    }

    fn is_in_same_circuit(&self, a: &JunctionBox, b: &JunctionBox) -> bool {
        for circuit in self.circuits.iter() {
            if circuit.contains(a) && circuit.contains(b) {
                return true;
            }
        }
        false
    }
}

pub struct Circuit {
    boxes: Vec<JunctionBox>,
}

impl Circuit {
    pub fn new() -> Circuit {
        Circuit { boxes: Vec::new() }
    }

    // Attempt to connect this circuit with another
    // If there is a match, we add the other box to this circuit
    pub fn try_connect_circuit(&mut self, other_circuit: &Circuit) -> bool {
        let mut self_indices: HashSet<usize> = self.boxes.iter().map(|b| b.index).collect();

        // Check if there's any overlap between the two circuits
        let has_overlap = other_circuit
            .boxes
            .iter()
            .any(|b| self_indices.contains(&b.index));

        if !has_overlap {
            return false;
        }

        // There is a match, we connect the other boxes
        for other_box in other_circuit.boxes.iter() {
            // If hash set succeeds inserts, we know that it's a new box
            if self_indices.insert(other_box.index) {
                self.boxes.push(other_box.clone());
            }
        }

        true
    }

    pub fn contains(&self, box_item: &JunctionBox) -> bool {
        self.boxes.iter().any(|b| b.index == box_item.index)
    }
}

#[derive(Clone)]
pub struct JunctionBox {
    index: usize,
    position: Vector3<f64>,
}

impl JunctionBox {
    pub fn new(index: usize, position: Vector3<f64>) -> JunctionBox {
        JunctionBox { index, position }
    }
}

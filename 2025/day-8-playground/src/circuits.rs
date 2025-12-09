use nalgebra::Vector3;

pub struct Circuits {
    boxes: Vec<JunctionBox>,
    circuits: Vec<Circuit>,
    prev_closest: f64,
}

impl Circuits {
    pub fn new() -> Circuits {
        Circuits {
            boxes: Vec::new(),
            circuits: Vec::new(),
            prev_closest: 0.0,
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

                let distance = Self::distance(box_item, other_box);
                if distance < closest_distance && distance > self.prev_closest {
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

        // Check if the circuit already exists
        for circuit in self.circuits.iter_mut() {
            if circuit.try_connect_circuit(&mut new_circuelt) {
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
    pub fn try_connect_circuit(&mut self, other_circuit: &mut Circuit) -> bool {
        let mut is_connected = false;

        for own_box in self.boxes.iter() {
            for other_box in other_circuit.boxes.iter() {
                if own_box.index == other_box.index {
                    is_connected = true;
                    break;
                }
            }
        }

        if !is_connected {
            return false;
        }

        // There is match, we connect the other boxes
        for other_box in other_circuit.boxes.iter() {
            let mut exists = false;
            for box_item in self.boxes.iter() {
                if other_box.index == box_item.index {
                    exists = true;
                }
            }
            if !exists {
                self.boxes.push(other_box.clone());
            }
        }

        true
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

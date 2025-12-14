use crate::device::Device;

pub struct Tree {
    devices: Vec<Device>,
    nodes: Vec<TreeNode>,
    root_idx: usize,
}

pub struct TreeNode {
    pub device: Device,
    pub children: Vec<usize>,
    pub dac_visited: bool,
    pub fft_visited: bool,
}

impl Tree {
    pub fn new(devices: Vec<Device>) -> Tree {
        let root_device = Self::get_root_device(&devices);
        let root = TreeNode {
            device: root_device,
            children: Vec::new(),
            dac_visited: false,
            fft_visited: false,
        };
        Tree {
            devices,
            nodes: vec![root],
            root_idx: 0,
        }
    }

    pub fn grow_tree(&mut self) -> u32 {
        if self.nodes[self.root_idx].device.output.is_empty() {
            panic!("Root node required");
        }

        let mut solutions: u32 = 0;
        let mut current_indices: Vec<usize> = vec![self.root_idx];

        loop {
            if current_indices.is_empty() {
                break;
            }

            let mut next_level_indices: Vec<usize> = Vec::new();

            for &node_idx in current_indices.iter() {
                let current_node = &self.nodes[node_idx];
                let outputs = current_node.device.output.clone();

                let is_dac_visited = current_node.dac_visited || (current_node.device.is_dac());
                let is_fft_visited = current_node.fft_visited || (current_node.device.is_fft());

                for output in outputs.iter() {
                    let new_node = TreeNode {
                        device: self.get_device(output),
                        children: Vec::new(),
                        dac_visited: is_dac_visited,
                        fft_visited: is_fft_visited,
                    };
                    let new_idx = self.nodes.len();
                    self.nodes.push(new_node);

                    self.nodes[node_idx].children.push(new_idx);

                    if self.nodes[new_idx].device.is_out() {
                        if is_dac_visited && is_fft_visited {
                            solutions += 1;
                        }
                    } else {
                        next_level_indices.push(new_idx);
                    }
                }
            }

            current_indices = next_level_indices;
        }

        solutions
    }

    fn get_device(&self, input: &String) -> Device {
        for idx in 0..self.devices.len() {
            if &self.devices[idx].input == input {
                return self.devices[idx].clone();
            }
        }
        panic!("Device '{}' not found...", input);
    }

    fn get_root_device(devices: &Vec<Device>) -> Device {
        for device in devices {
            if device.input == "svr" {
                return device.clone();
            }
        }
        panic!("No initial state found");
    }
}

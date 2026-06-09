use crate::device::Device;
use std::{
    cell::RefCell,
    collections::VecDeque,
    rc::{Rc, Weak},
};

pub enum SpruceType {
    DAC,
    FFT,
}

pub struct Spruce {
    pub spruce_type: SpruceType,
    pub origin: Rc<RefCell<Node>>,
    pub devices: Vec<Device>,

    // Paths
    pub origin_to_out: Vec<Node>,
    pub origin_to_root: Vec<Node>,
}

pub struct Node {
    pub device: Device,
    pub children: Vec<Rc<RefCell<Node>>>,
    pub parent: Option<Weak<RefCell<Node>>>,
}

impl Spruce {
    pub fn new(spruce_type: SpruceType, devices: Vec<Device>) -> Spruce {
        let origin_node = Self::get_origin(&spruce_type, &devices);
        let origin = Rc::new(RefCell::new(origin_node));
        Spruce {
            spruce_type,
            origin,
            devices,
            origin_to_out: Vec::new(),
            origin_to_root: Vec::new(),
        }
    }

    pub fn grow_down(&self) {
        let mut queue: VecDeque<Rc<RefCell<Node>>> = VecDeque::new();
        queue.push_back(self.origin.clone());

        while !queue.is_empty() {
            let current_node = queue.pop_front().unwrap();

            let outputs = current_node.borrow().device.output.clone();

            for output in outputs.iter() {
                let new_device = self.get_device(output);
                let new_node = Node {
                    device: new_device,
                    children: Vec::new(),
                    parent: Some(Rc::downgrade(&current_node)),
                };

                if new_node.device.is_out() {
                    // Create new path from origin to out
                    continue;
                }

                let new_node_rc = Rc::new(RefCell::new(new_node));
                current_node.borrow_mut().children.push(new_node_rc.clone());
                queue.push_back(new_node_rc);
            }
        }
    }

    pub fn grow_up(&self) {}

    fn get_origin(spruce_type: &SpruceType, devices: &Vec<Device>) -> Node {
        for device in devices.iter() {
            match spruce_type {
                SpruceType::DAC => {
                    if device.is_dac() {
                        return Node {
                            device: device.clone(),
                            children: Vec::new(),
                            parent: None,
                        };
                    }
                }
                SpruceType::FFT => {
                    if device.is_fft() {
                        return Node {
                            device: device.clone(),
                            children: Vec::new(),
                            parent: None,
                        };
                    }
                }
            }
        }
        panic!("No origin device found");
    }

    fn get_device(&self, input: &String) -> Device {
        for idx in 0..self.devices.len() {
            if &self.devices[idx].input == input {
                return self.devices[idx].clone();
            }
        }
        panic!("Device '{}' not found...", input);
    }
}

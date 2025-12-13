use crate::button::Button;
use crate::jolt_state::JoltState;

#[derive(Clone)]
pub struct Machine {
    pub lights: Vec<bool>,
    pub buttons: Vec<Button>,
    pub jolts: Vec<u32>,
}

impl Machine {
    pub fn new(input: &str) -> Machine {
        let split = input.split(" ").collect::<Vec<&str>>();

        let lights = Self::parse_lights(split[0]);

        let mut buttons: Vec<Button> = Vec::new();
        for idx in 1..split.len() - 1 {
            let part = split[idx];

            let l = part.replace("(", "").replace(")", "");
            let impacted_lights: Vec<u32> = l.split(",").map(|s| s.parse().unwrap()).collect();
            buttons.push(Button {
                on_click: impacted_lights,
            });
        }

        let mut jolts: Vec<u32> = Vec::new();
        let jolt_str = split[split.len() - 1].replace("{", "").replace("}", "");
        let jolt_parts = jolt_str.split(",");
        for jp in jolt_parts {
            jolts.push(jp.parse().unwrap());
        }

        Machine {
            lights,
            buttons,
            jolts,
        }
    }

    pub fn get_lowest_jolt(&self) -> u32 {
        let mut lowest = u32::MAX;
        for j in &self.jolts {
            if *j < lowest {
                lowest = *j;
            }
        }
        lowest
    }

    fn parse_lights(input: &str) -> Vec<bool> {
        input
            .chars()
            .filter(|c| *c == '#' || *c == '.')
            .map(|c| if c == '#' { true } else { false })
            .collect()
    }

    pub fn print(&self) {
        println!("\nMachine [{:?}]", self.lights);
        for b in &self.buttons {
            println!("{:?}", b.on_click);
        }
        println!("Jolts: {:?}", self.jolts);
    }
}

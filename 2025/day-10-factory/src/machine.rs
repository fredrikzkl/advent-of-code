use crate::button::Button;

#[derive(Clone)]
pub struct Machine {
    pub lights: Vec<bool>,
    pub buttons: Vec<Button>,
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

        Machine { lights, buttons }
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
    }
}

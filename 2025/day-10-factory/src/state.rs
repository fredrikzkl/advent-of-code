use crate::button::Button;

pub struct State {
    pub depth: usize,
    pub index: usize,
    pub parent: Option<(usize, usize)>, // (depth_index, state_index)
    pub clicked_button: Option<Button>,
    pub lights: Vec<bool>,
}

impl State {
    pub fn new(
        depth: usize,
        index: usize,
        parent: Option<(usize, usize)>,
        clicked_button: Option<Button>,
        lights: Vec<bool>,
    ) -> State {
        State {
            depth,
            index,
            parent,
            clicked_button,
            lights,
        }
    }

    pub fn get_lights(&self) -> &Vec<bool> {
        &self.lights
    }
}

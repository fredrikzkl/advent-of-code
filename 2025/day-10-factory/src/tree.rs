use crate::button::Button;
use crate::machine::Machine;
use crate::state::State;

pub struct Tree {
    machine: Machine,
    tree: Vec<Vec<State>>,
}

impl Tree {
    pub fn new(machine: Machine) -> Tree {
        let tree: Vec<Vec<State>> = vec![Vec::new()];
        Tree { machine, tree }
    }

    pub fn search(&mut self) -> u32 {
        let initial_state = self.get_initial_state();
        self.tree[0].push(initial_state);

        for depth in 0.. {
            for idx in 0..self.tree[depth].len() {
                let state = &self.tree[depth][idx];

                let current_level_states =
                    self.apply_buttons_to_state(state, depth as u32, idx as u32);

                for new_state in current_level_states {
                    if self.check_if_goal_state(&new_state) {
                        println!(
                            "Goal state reached at depth {}, index {}",
                            new_state.depth, new_state.index
                        );
                        return new_state.depth as u32;
                    }

                    if self.tree.len() <= new_state.depth {
                        self.tree.push(Vec::new());
                    }

                    self.tree[new_state.depth].push(new_state);
                }
            }
        }
        panic!("Goal state not found");
    }

    fn check_if_goal_state(&self, state: &State) -> bool {
        for (i, &light) in state.lights.iter().enumerate() {
            if light != self.machine.lights[i] {
                return false;
            }
        }
        true
    }

    fn apply_buttons_to_state(
        &self,
        state: &State,
        parent_depth: u32,
        parent_index: u32,
    ) -> Vec<State> {
        let mut result: Vec<State> = Vec::new();
        for current_button in self.machine.buttons.iter() {
            if state.clicked_button.as_ref() == Some(current_button) {
                continue;
            }

            let new_lights = self.change_lights(state, current_button);

            let new_state = State::new(
                (parent_depth + 1) as usize,
                parent_index as usize,
                Some((parent_depth as usize, parent_index as usize)),
                Some(current_button.clone()),
                new_lights,
            );
            result.push(new_state);
        }
        result
    }

    fn change_lights(&self, state: &State, button: &Button) -> Vec<bool> {
        let mut new_lights = state.lights.clone();
        for &light_index in &button.on_click {
            let idx = light_index as usize;
            if idx < new_lights.len() {
                new_lights[idx] = !new_lights[idx];
            }
        }
        new_lights
    }

    fn get_initial_state(&self) -> State {
        let mut init_lights = self.machine.lights.clone();
        init_lights.fill(false);
        State::new(0, 0, None, None, init_lights)
    }
}

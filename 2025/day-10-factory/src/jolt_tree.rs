use crate::button::Button;
use crate::jolt_state::JoltState;
use crate::machine::Machine;

pub struct JoltTree {
    machine: Machine,
    tree: Vec<Vec<JoltState>>,
}

impl JoltTree {
    pub fn new(machine: Machine) -> JoltTree {
        let tree: Vec<Vec<JoltState>> = vec![Vec::new()];
        JoltTree { machine, tree }
    }

    pub fn search(&mut self) -> u32 {
        let initial_state = self.get_initial_state();
        let max_highway_depth = (self.machine.get_lowest_jolt() as f32 * 0.8_f32).floor();

        let button_to_spam = self.get_button_to_spam();
        let mut depth_counter = 0;

        self.tree[0].push(initial_state);

        while depth_counter < max_highway_depth as usize {
            let new_jolts = self.add_jolts(&self.tree[depth_counter][0], &button_to_spam);
            let new_state = JoltState::new(depth_counter + 1, 0, new_jolts);
            self.tree.push(vec![new_state]);

            depth_counter += 1;
        }

        println!(
            "Completed jolt highway of depth {} using button impacting jolts {:?}",
            depth_counter, button_to_spam.on_click
        );

        let depth_from_search = self.generate_tree();
        depth_from_search + depth_counter as u32
    }

    fn generate_tree(&mut self) -> u32 {
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
            println!("Completed depth {}", depth + 1);
            println!("States at this depth: {}", self.tree[depth + 1].len());
        }
        0
    }

    fn check_if_goal_state(&self, state: &JoltState) -> bool {
        for (i, &jolt) in state.jolts.iter().enumerate() {
            if jolt != self.machine.jolts[i] {
                return false;
            }
        }
        true
    }

    fn apply_buttons_to_state(
        &self,
        state: &JoltState,
        parent_depth: u32,
        parent_index: u32,
    ) -> Vec<JoltState> {
        let mut result: Vec<JoltState> = Vec::new();
        for current_button in self.machine.buttons.iter() {
            let new_jolts = self.add_jolts(state, current_button);

            if self.is_jolt_overflow(&new_jolts) {
                continue;
            }

            let new_state = JoltState::new(
                (parent_depth + 1) as usize,
                parent_index as usize,
                new_jolts,
            );
            result.push(new_state);
        }
        result
    }

    fn is_jolt_overflow(&self, jolts: &Vec<u32>) -> bool {
        if jolts.len() != self.machine.jolts.len() {
            panic!("Invalid state! Jolt length mismatch");
        }
        for idx in 0..self.machine.jolts.len() {
            if jolts[idx] > self.machine.jolts[idx] {
                return true;
            }
        }
        false
    }

    fn add_jolts(&self, state: &JoltState, button: &Button) -> Vec<u32> {
        let mut new_jolts = state.jolts.clone();
        for &jolt_index in &button.on_click {
            let idx = jolt_index as usize;
            if idx < new_jolts.len() {
                new_jolts[idx] += 1;
            }
        }
        new_jolts
    }

    fn get_button_to_spam(&self) -> Button {
        let lowest_jolt_index = self.get_lowest_jolt_index();

        // Sort buttons based on most imacted jolts
        let mut sorted_buttons: Vec<Button> = self.machine.buttons.clone();
        sorted_buttons.sort_by_key(|b| b.on_click.len());

        for button in sorted_buttons.iter().rev() {
            if button.on_click.contains(&(lowest_jolt_index as u32)) {
                return button.clone();
            }
        }
        sorted_buttons[0].clone()
    }

    fn get_lowest_jolt_index(&self) -> usize {
        let mut lowest_jolt_index = 0;
        for idx in 1..self.machine.jolts.len() {
            if self.machine.jolts[idx] < self.machine.jolts[lowest_jolt_index] {
                lowest_jolt_index = idx;
            }
        }
        lowest_jolt_index
    }

    fn get_initial_state(&self) -> JoltState {
        let mut init_jolts = self.machine.jolts.clone();
        init_jolts.fill(0);
        JoltState::new(0, 0, init_jolts)
    }
}

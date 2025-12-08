pub struct Laboriatory {
    map: Vec<Vec<char>>,
    map_width: usize,
    map_height: usize,
    splits: u64,
    timelines_splitted: u64,
}

impl Laboriatory {
    pub fn new() -> Self {
        Self {
            map: Vec::new(),
            map_width: 0,
            map_height: 0,
            splits: 0,
            timelines_splitted: 0,
        }
    }

    pub fn add_line(&mut self, line: &str) {
        self.map.push(line.chars().collect());
        self.map_width = line.len();
        self.map_height += 1;
    }

    pub fn solve_part_2(&mut self) {
        // Find the first first spot
        let col = self.map[0]
            .iter()
            .position(|&c| c == 'S')
            .expect("No starting point found");

        // Start the timeline
        self.quantum_split(0, col);
    }

    pub fn quantum_split(&mut self, row: usize, col: usize) {
        if row + 1 >= self.map_height {
            return;
        }

        let cell_below = self.map[row + 1][col];

        if cell_below == '.' {
            self.quantum_split(row + 1, col);
            return;
        }

        if cell_below == '^' {
            self.timelines_splitted += 1;

            // New timeline at left col
            if let Some(left_col) = col.checked_sub(1) {
                if self.map[row + 1][left_col] == '.' {
                    self.quantum_split(row + 1, left_col);
                }
            }

            // New timeline at right col
            if col + 1 < self.map_width {
                if self.map[row + 1][col + 1] == '.' {
                    self.quantum_split(row + 1, col + 1);
                }
            }
        }
    }

    pub fn solve_diagram(&mut self) {
        for row in 0..self.map_height - 1 {
            // We do -1, since we don't care about the last row
            for col in 0..self.map_width {
                let cell = self.map[row][col];

                if cell == 'S' || cell == '|' {
                    let cell_below = self.map[row + 1][col];
                    if cell_below == '.' {
                        self.map[row + 1][col] = '|';
                        continue;
                    }

                    if cell_below == '^' {
                        self.splits += 1;
                        // Left cell
                        if let Some(left_col) = col.checked_sub(1) {
                            if self.map[row + 1][left_col] == '.' {
                                self.map[row + 1][left_col] = '|';
                            }
                        }
                        // Right cell
                        if col + 1 < self.map_width {
                            if self.map[row + 1][col + 1] == '.' {
                                self.map[row + 1][col + 1] = '|';
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn count_beams(&self) -> u64 {
        let mut count = 0;
        for row in 0..self.map_height {
            for col in 0..self.map_width {
                if self.map[row][col] == '|' {
                    count += 1;
                }
            }
        }
        count
    }

    pub fn print_map(&self) {
        for row in 0..self.map_height {
            for col in 0..self.map_width {
                print!("{}", self.map[row][col]);
            }
            println!();
        }
    }

    pub fn get_splits(&self) -> u64 {
        self.splits
    }

    pub fn get_timelines_splitted(&self) -> u64 {
        self.timelines_splitted
    }
}

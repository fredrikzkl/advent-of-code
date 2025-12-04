pub struct ScrollGrid {
    pub orignal_grid: Vec<Vec<char>>,
    height: usize,
    width: usize,
}

impl ScrollGrid {
    pub fn new() -> ScrollGrid {
        let new_grid: Vec<Vec<char>> = Vec::new();
        ScrollGrid {
            orignal_grid: new_grid,
            height: 0,
            width: 0,
        }
    }

    pub fn calculate_valid_scrolls(&mut self) -> usize {
        let mut valid_scrolls = 0;
        for x in 0..self.width {
            for y in 0..self.height {
                let current_char = self.orignal_grid[x][y];
                if current_char == '@' && self.is_valid_scroll(x, y) {
                    valid_scrolls += 1;
                    self.orignal_grid[x][y] = 'x';
                }
            }
        }
        valid_scrolls
    }

    fn is_valid_scroll(&self, scroll_x: usize, scroll_y: usize) -> bool {
        let mut adjacent_scrolls = 0;
        for x in -1..=1 {
            for y in -1..=1 {
                if x == 0 && y == 0 {
                    continue;
                }

                let new_x = scroll_x as isize + x as isize;
                let new_y = scroll_y as isize + y as isize;

                if !self.is_valid_coordinate(new_x, new_y) {
                    continue;
                }

                let temp = self.orignal_grid[new_x as usize][new_y as usize];
                if temp == '@' || temp == 'x' {
                    adjacent_scrolls += 1;
                }
            }
        }

        if adjacent_scrolls < 4 {
            return true;
        }
        false
    }

    fn is_valid_coordinate(&self, x: isize, y: isize) -> bool {
        if x < 0 || y < 0 {
            return false;
        }

        if x as usize >= self.width || y as usize >= self.height {
            return false;
        }
        true
    }

    pub fn clean_all_marked_scrolls(&mut self) {
        for x in 0..self.width {
            for y in 0..self.height {
                if self.orignal_grid[x][y] == 'x' {
                    self.orignal_grid[x][y] = '.';
                }
            }
        }
    }

    pub fn add_line(&mut self, line: &str) {
        self.orignal_grid.push(line.chars().collect());
        self.height += 1;
        self.width = line.len();
    }

    pub fn print_grid(&self) {
        for x in 0..self.width {
            for y in 0..self.height {
                print!("{}", self.orignal_grid[x][y]);
            }
            println!("");
        }
    }
}

use crate::point::Point;

pub struct Map {
    map: Vec<Vec<char>>,
}

impl Map {
    pub fn new(red_tiles: &Vec<Point>) -> Self {
        let map = Self::generate_map(red_tiles);
        Map { map }
    }

    fn generate_map(red_tiles: &Vec<Point>) -> Vec<Vec<char>> {
        let mut max_x = 0.0;
        let mut max_y = 0.0;

        for tile in red_tiles {
            if tile.x > max_x {
                max_x = tile.x;
            }
            if tile.y > max_y {
                max_y = tile.y;
            }
        }

        let width = max_x as usize + 1;
        let height = max_y as usize + 1;

        let mut map = vec![vec!['.'; width]; height];

        for tile in red_tiles {
            map[tile.y as usize][tile.x as usize] = '#';
        }

        // Green tiles
        for point in red_tiles {
            for other in red_tiles {
                if point.x == other.x {
                    Self::populate_green_tiles_vertically(
                        &mut map,
                        point.x as usize,
                        point.y as usize,
                        other.y as usize,
                    );
                }

                if point.y == other.y {
                    Self::populate_green_tiles_horizontally(
                        &mut map,
                        point.y as usize,
                        point.x as usize,
                        other.x as usize,
                    );
                }
            }
        }

        // Fill the rest...
        for row in 0..height {
            let start = Self::find_first_occurence(&map[row]);
            let end = Self::find_last_occurence(&map[row]);

            if let (Some(start_idx), Some(end_idx)) = (start, end) {
                if end_idx < start_idx {
                    continue;
                }
                for i in start_idx..=end_idx.min(map[row].len().saturating_sub(1)) {
                    if map[row][i] == '.' {
                        map[row][i] = 'X';
                    }
                }
            }
        }

        map
    }

    pub fn find_first_occurence(row: &[char]) -> Option<usize> {
        for (idx, c) in row.iter().enumerate() {
            if *c != '.' {
                return Some(idx);
            }
        }
        None
    }

    pub fn find_last_occurence(row: &[char]) -> Option<usize> {
        for idx in (0..row.len()).rev() {
            if row[idx] != '.' {
                return Some(idx);
            }
        }
        None
    }

    fn populate_green_tiles_vertically(
        map: &mut Vec<Vec<char>>,
        col: usize,
        start_y: usize,
        end_y: usize,
    ) {
        let start = start_y.min(end_y);
        let end = start_y.max(end_y).min(map.len() - 1);
        for row in start..=end {
            if map[row][col] != '#' {
                map[row][col] = 'X';
            }
        }
    }

    fn populate_green_tiles_horizontally(
        map: &mut Vec<Vec<char>>,
        row: usize,
        start_x: usize,
        end_x: usize,
    ) {
        if row >= map.len() {
            return;
        }
        let start = start_x.min(end_x);
        let end = start_x.max(end_x).min(map[row].len() - 1);
        for col in start..=end {
            if map[row][col] != '#' {
                map[row][col] = 'X';
            }
        }
    }

    pub fn print(&self) {
        for row in &self.map {
            let line: String = row.iter().collect();
            println!("{}", line);
        }
    }
}

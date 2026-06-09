use crate::point::Point;
use crate::rectangle::Rectangle;

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

        let mut map = vec![vec!['.'; height]; width];

        for tile in red_tiles {
            map[tile.x as usize][tile.y as usize] = '#';
        }

        println!("Outline created. Map size: {} x {}", width, height);

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

        println!("red_tiles created");

        // Fill the rest...
        for col in 0..width {
            let start = Self::find_first_occurence(&map[col]);
            let end = Self::find_last_occurence(&map[col]);

            if let (Some(start_idx), Some(end_idx)) = (start, end) {
                if end_idx < start_idx {
                    continue;
                }
                for i in start_idx..=end_idx.min(map[col].len().saturating_sub(1)) {
                    if map[col][i] == '.' {
                        map[col][i] = 'X';
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
        let end = start_y.max(end_y).min(map[col].len().saturating_sub(1));
        for row in start..=end {
            if map[col][row] != '#' {
                map[col][row] = 'X';
            }
        }
    }

    fn populate_green_tiles_horizontally(
        map: &mut Vec<Vec<char>>,
        row: usize,
        start_x: usize,
        end_x: usize,
    ) {
        let start = start_x.min(end_x);
        let end = start_x.max(end_x).min(map.len().saturating_sub(1));
        for col in start..=end {
            if map[col][row] != '#' {
                map[col][row] = 'X';
            }
        }
    }

    pub fn print(&self) {
        if self.map.is_empty() {
            return;
        }
        let height = self.map[0].len();
        for y in 0..height {
            let line: String = (0..self.map.len())
                .map(|x| self.map[x][y])
                .collect();
            println!("{}", line);
        }
    }

    pub fn write_to_file(&self, filename: &str) -> std::io::Result<()> {
        use std::fs::File;
        use std::io::Write;

        let mut file = File::create(filename)?;
        if self.map.is_empty() {
            return Ok(());
        }
        let height = self.map[0].len();
        for y in 0..height {
            let line: String = (0..self.map.len())
                .map(|x| self.map[x][y])
                .collect();
            writeln!(file, "{}", line)?;
        }
        Ok(())
    }

    pub fn is_valid(&self, rect: &Rectangle) -> bool {
        let start_x = rect.point_a.x.min(rect.point_b.x) as usize;
        let end_x = rect.point_a.x.max(rect.point_b.x) as usize;

        let start_y = rect.point_a.y.min(rect.point_b.y) as usize;
        let end_y = rect.point_a.y.max(rect.point_b.y) as usize;

        // Check bounds - map uses map[x][y]
        if end_x >= self.map.len() {
            return false;
        }

        // Map uses map[x][y]
        for x in start_x..=end_x {
            if end_y >= self.map[x].len() {
                return false;
            }
            for y in start_y..=end_y {
                if self.map[x][y] == '.' {
                    return false;
                }
            }
        }

        true
    }
}

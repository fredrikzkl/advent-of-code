pub struct FreshList {
    ranges: Vec<Range>,
}

struct Range {
    start: u64,
    end: u64,
}

impl FreshList {
    pub fn new() -> Self {
        FreshList { ranges: Vec::new() }
    }

    pub fn add_range(&mut self, range_str: &str) {
        let parts: Vec<&str> = range_str.split('-').collect();
        let start: u64 = parts[0].parse().unwrap();
        let end: u64 = parts[1].parse().unwrap();
        self.ranges.push(Range { start, end });
    }

    pub fn is_fresh(&self, food_id: u64) -> bool {
        for range in &self.ranges {
            if food_id >= range.start && food_id <= range.end {
                return true;
            }
        }
        false
    }

    pub fn get_valid_ingredients_count(&mut self) -> u64 {
        self.adjust_overlapping_ranges();
        self.ranges
            .iter()
            .filter(|range| range.end >= range.start)
            .map(|range| range.end - range.start + 1)
            .sum()
    }

    pub fn adjust_overlapping_ranges(&mut self) {
        let mut perfect_list: Vec<Range> = Vec::new();

        self.ranges.sort_by(|a, b| a.start.cmp(&b.start));
        for i in 0..self.ranges.len() {
            let mut current = self.clone_range(i);

            if perfect_list.is_empty() {
                perfect_list.push(current);
                continue;
            }

            let previous = perfect_list.last().unwrap();

            if current.start <= previous.end {
                current.start = previous.end + 1;

                if current.end <= previous.start {
                    continue;
                }
            }

            perfect_list.push(current);
        }

        self.ranges = perfect_list;
    }

    pub fn get_ranges(&self) -> Vec<(u64, u64)> {
        self.ranges
            .iter()
            .map(|range| (range.start, range.end))
            .collect()
    }

    pub fn print_ranges(&self) {
        for range in &self.ranges {
            println!("{}-{})", range.start, range.end);
        }
    }

    fn clone_range(&self, index: usize) -> Range {
        Range {
            start: self.ranges[index].start,
            end: self.ranges[index].end,
        }
    }
}

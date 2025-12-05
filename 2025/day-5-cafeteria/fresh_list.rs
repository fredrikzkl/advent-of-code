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
        let total_ids: u64 = self
            .ranges
            .iter()
            .filter(|range| range.end >= range.start)
            .map(|range| range.end - range.start + 1)
            .sum();
        total_ids
    }

    pub fn adjust_overlapping_ranges(&mut self) {
        self.ranges.sort_by(|a, b| a.start.cmp(&b.start));
        for i in 0..self.ranges.len() {
            if i > 0 && self.ranges[i].start <= self.ranges[i - 1].end {
                self.ranges[i].start = self.ranges[i - 1].end + 1;
            }

            if i == self.ranges.len() - 1 {
                break;
            }

            if self.ranges[i].end > self.ranges[i + 1].start {
                self.ranges[i].end = self.ranges[i + 1].start - 1;
            }
        }
    }
}

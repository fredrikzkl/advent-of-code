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
}

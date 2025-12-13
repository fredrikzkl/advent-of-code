pub struct JoltState {
    pub depth: usize,
    pub index: usize,
    pub jolts: Vec<u32>,
}

impl JoltState {
    pub fn new(depth: usize, index: usize, jolts: Vec<u32>) -> JoltState {
        JoltState {
            depth,
            index,
            jolts,
        }
    }
}

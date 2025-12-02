pub struct IdCracker {
    first_id: u64,
    last_id: u64,
}

impl IdCracker {
    pub fn new(entry: &str) -> Self {
        let entries: Vec<&str> = entry.split('-').collect();
        IdCracker {
            first_id: entries[0].parse().unwrap(),
            last_id: entries[1].parse().unwrap(),
        }
    }

    pub fn crack_ids(&self) -> Vec<u64> {
        let mut valid_ids = Vec::new();

        for current_id in self.first_id..=self.last_id {
            let id_as_string = current_id.to_string();
            if id_as_string.len() % 2 != 0 {
                continue;
            }

            let (first_chunk, second_chunk) = self.split_id(id_as_string.as_str());

            if first_chunk == second_chunk {
                valid_ids.push(current_id);
            }
        }

        valid_ids
    }

    fn split_id<'a>(&self, id: &'a str) -> (&'a str, &'a str) {
        let split_location = id.len() / 2;
        id.split_at(split_location)
    }
}

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

            // Part 1:
            // if id_as_string.len() % 2 != 0 {
            //     continue;
            // }
            // let (first_chunk, second_chunk) = self.split_id(id_as_string.as_str());
            //
            // if first_chunk == second_chunk {
            //     valid_ids.push(current_id);
            // }

            // Part 2
            if self.id_has_valid_split(id_as_string.as_str()) {
                valid_ids.push(current_id);
            };
        }

        valid_ids
    }

    fn split_id<'a>(&self, id: &'a str) -> (&'a str, &'a str) {
        let split_location = id.len() / 2;
        id.split_at(split_location)
    }

    fn id_has_valid_split(&self, id: &str) -> bool {
        for chunk_count in 1..=id.len() {
            if id.len() % chunk_count != 0 || chunk_count == 1 {
                continue;
            }

            let chunk_size = id.len() / chunk_count;
            let chunks: Vec<&str> = id
                .char_indices()
                .step_by(chunk_size)
                .map(|(i, _)| &id[i..i + chunk_size])
                .collect();

            if self.check_all_chuks_equal(chunks) {
                return true;
            }
        }
        return false;
    }

    fn check_all_chuks_equal(&self, chunks: Vec<&str>) -> bool {
        if chunks.is_empty() || chunks.len() == 1 {
            return false;
        }
        for chunk in chunks.iter() {
            if chunk != &chunks[0] {
                return false;
            }
        }
        true
    }
}

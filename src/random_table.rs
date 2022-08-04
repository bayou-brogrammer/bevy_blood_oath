pub struct RandomEntry {
    name: String,
    weight: i32,
}

impl RandomEntry {
    pub fn new<S: ToString>(name: S, weight: i32) -> RandomEntry {
        RandomEntry { name: name.to_string(), weight }
    }
}

#[derive(Default)]
pub struct RandomTable {
    total_weight: i32,
    entries: Vec<RandomEntry>,
}

impl RandomTable {
    pub fn new() -> RandomTable {
        RandomTable { entries: Vec::new(), total_weight: 0 }
    }

    pub fn add<S: ToString>(mut self, name: S, weight: i32) -> RandomTable {
        if weight > 0 {
            self.total_weight += weight;
            self.entries.push(RandomEntry::new(name.to_string(), weight));
        }
        self
    }

    pub fn roll(&self) -> String {
        if self.total_weight == 0 {
            return "None".to_string();
        }

        let mut roll = crate::rng::roll_dice(1, self.total_weight) - 1;
        let mut index: usize = 0;

        while roll > 0 {
            if roll < self.entries[index].weight {
                return self.entries[index].name.clone();
            }

            roll -= self.entries[index].weight;
            index += 1;
        }

        "None".to_string()
    }
}

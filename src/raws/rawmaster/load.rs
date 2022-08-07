use super::*;
use std::collections::HashSet;

impl RawMaster {
    pub fn load(&mut self, raws: Raws) {
        self.raws = raws;

        self.item_index = HashMap::new();
        let mut used_names: HashSet<String> = HashSet::new();

        // Items
        load_entity_data(&self.raws.items, &mut self.item_index, &mut used_names);
        // Mobs
        load_entity_data(&self.raws.mobs, &mut self.mob_index, &mut used_names);
        // Props
        load_entity_data(&self.raws.props, &mut self.prop_index, &mut used_names);
    }
}

pub fn load_entity_data<T: 'static + BaseRawComponent>(
    raws: &[T],
    entiy_index: &mut HashMap<String, usize>,
    used_names: &mut HashSet<String>,
) {
    for (i, entity) in raws.iter().enumerate() {
        let entity_name = entity.name();

        if used_names.contains(&entity_name) {
            println!("WARNING - duplicate entity name in raws [{}]", entity_name);
        }

        entiy_index.insert(entity_name.clone(), i);
        used_names.insert(entity_name.clone());
    }
}

use crate::prelude::*;

pub fn add_debug_systems(app: &mut App) {
    app.add_system_set_to_stage(
        CoreStage::Update,
        ConditionSet::new()
            .with_system(|m_q: Query<&Point, Added<Monster>>, i_q: Query<&Point, Added<Item>>| {
                for pos in m_q.iter() {
                    eprintln!("Monster Spawned at {:?}", pos)
                }
                for pos in i_q.iter() {
                    eprintln!("Item Spawned at {:?}", pos)
                }
            })
            .into(),
    );
}

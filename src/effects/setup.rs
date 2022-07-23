use super::*;

pub struct EffectsPlugin;
impl Plugin for EffectsPlugin {
    fn build(&self, app: &mut App) {
        // Main Effect Queue
        app.add_system_set(ConditionSet::new().with_system(effects_queue).into());

        // Triggers
        app.add_system_set(
            // GameStage::Effects,
            ConditionSet::new().run_on_event::<AffectEntity>().with_system(affect_entity).into(),
        )
        .add_system_set(
            // GameStage::Effects,
            ConditionSet::new().run_on_event::<AffectTile>().with_system(affect_tile).into(),
        )
        .add_system_set(
            // GameStage::Effects,
            ConditionSet::new().run_on_event::<ItemTrigger>().with_system(item_trigger).into(),
        );

        // Effect Events
        app.add_system_set_to_stage(
            GameStage::Effects,
            ConditionSet::new().run_on_event::<DamageEvent>().with_system(inflict_damage).into(),
        )
        .add_system_set_to_stage(
            GameStage::Effects,
            ConditionSet::new().run_on_event::<DeathEvent>().with_system(death).into(),
        )
        .add_system_set_to_stage(
            GameStage::Effects,
            ConditionSet::new().run_on_event::<HealEvent>().with_system(heal_damage).into(),
        );
    }
}

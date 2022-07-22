use crate::prelude::*;
use bevy_app::App;
use bevy_ecs::schedule::StageLabel;
use iyes_loopless::prelude::ConditionSet;

pub fn setup_effect_system(app: &mut App, effect_stage: impl StageLabel + Copy) {
    app.add_system_set_to_stage(
        effect_stage,
        ConditionSet::new().with_system(effects_queue).into(),
    );

    app.add_system_set_to_stage(
        effect_stage,
        ConditionSet::new()
            .run_on_event::<AffectEntity>()
            .with_system(affect_entity)
            .into(),
    );

    app.add_system_set_to_stage(
        effect_stage,
        ConditionSet::new()
            .run_on_event::<DamageEvent>()
            .with_system(inflict_damage)
            .into(),
    );

    app.add_system_set_to_stage(
        effect_stage,
        ConditionSet::new()
            .run_on_event::<DeathEvent>()
            .with_system(death)
            .into(),
    );

    app.add_system_set_to_stage(
        effect_stage,
        ConditionSet::new()
            .run_on_event::<ItemTrigger>()
            .with_system(item_trigger)
            .into(),
    );

    app.add_system_set_to_stage(
        effect_stage,
        ConditionSet::new()
            .run_on_event::<HealEvent>()
            .with_system(heal_damage)
            .into(),
    );
}

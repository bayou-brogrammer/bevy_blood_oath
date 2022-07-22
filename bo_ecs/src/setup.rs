use crate::prelude::*;
use bevy_app::App;
use bevy_ecs::schedule::StageLabel;
use iyes_loopless::prelude::ConditionSet;

pub fn setup_events(app: &mut App) {
    // Intent Events
    app.add_event::<WantsToMove>();
    app.add_event::<WantsToAttack>();

    // Item Events
    app.add_event::<WantsToPickupItem>();
    app.add_event::<WantsToUseItem>();
    app.add_event::<WantsToDropItem>();

    // Effects
    app.add_event::<AffectEntity>();
    app.add_event::<AffectTile>();

    // Effect Events
    app.add_event::<DamageEvent>()
        .add_event::<DeathEvent>()
        .add_event::<HealEvent>()
        .add_event::<ParticleEvent>();

    // Trigger Events
    app.add_event::<ItemTrigger>();
}

pub fn setup_effect_system(app: &mut App, effect_stage: impl StageLabel + Copy) {
    // Queue System
    app.add_system_set_to_stage(effect_stage, ConditionSet::new().with_system(effects_queue).into());

    // Affect Events
    app.add_system_set_to_stage(
        effect_stage,
        ConditionSet::new().run_on_event::<AffectEntity>().with_system(affect_entity).into(),
    );

    // app.add_system_set_to_stage(
    //     effect_stage,
    //     ConditionSet::new()
    //         .run_on_event::<AffectTile>()
    //         .with_system(affect_entity)
    //         .into(),
    // );

    // Trigger Events

    app.add_system_set_to_stage(
        effect_stage,
        ConditionSet::new().run_on_event::<ItemTrigger>().with_system(item_trigger).into(),
    );

    // Events
    app.add_system_set_to_stage(
        effect_stage,
        ConditionSet::new().run_on_event::<DamageEvent>().with_system(inflict_damage).into(),
    )
    .add_system_set_to_stage(
        effect_stage,
        ConditionSet::new().run_on_event::<HealEvent>().with_system(heal_damage).into(),
    )
    .add_system_set_to_stage(
        effect_stage,
        ConditionSet::new().run_on_event::<ParticleEvent>().with_system(particle_to_tile).into(),
    )
    .add_system_set_to_stage(
        effect_stage,
        ConditionSet::new().run_on_event::<DeathEvent>().with_system(death).into(),
    );
}

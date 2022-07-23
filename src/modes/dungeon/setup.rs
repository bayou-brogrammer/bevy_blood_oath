use crate::prelude::*;

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

/**
 * We need multiple stages to handle the following:
 * 1. Handle input from player and generate actions
 * 2. Generate Player Actions
 * 3. Handle Player Actions
 * 4. Generate AI Actions
 * 5. Handle AI Actions
 * 6. Effects
 */

pub fn setup_stages(app: &mut App) {
    // Player Stages
    app.add_stage_after(CoreStage::Update, GameStage::GeneratePlayerActions, SystemStage::parallel())
        .add_stage_after(
            GameStage::GeneratePlayerActions,
            GameStage::HandlePlayerActions,
            SystemStage::parallel(),
        )
        .add_stage_after(
            GameStage::HandlePlayerActions,
            GameStage::GenerateAIActions,
            SystemStage::parallel(),
        )
        // AI Stages
        .add_stage_after(GameStage::GenerateAIActions, GameStage::HandleAIActions, SystemStage::parallel())
        .add_stage_after(GameStage::HandleAIActions, GameStage::AICleanup, SystemStage::parallel())
        .add_stage_after(GameStage::HandleAIActions, GameStage::Effects, SystemStage::parallel())
        .add_stage_after(GameStage::Effects, GameStage::Cleanup, SystemStage::parallel());
}

pub fn setup_debug_systems(app: &mut App) {
    app.add_system_set_to_stage(
        CoreStage::Update,
        ConditionSet::new()
            .with_system(|m_q: Query<&Position, Added<Monster>>, i_q: Query<&Position, Added<Item>>| {
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

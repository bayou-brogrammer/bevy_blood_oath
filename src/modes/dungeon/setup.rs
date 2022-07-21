use super::{render::gui, *};

/**
 * We need multiple stages to handle the following:
 * 1. Update all events live in the world
 * 2. Handle input from player and generate actions
 * 3. Handle Player Turn
 * 4. Genrate Player Actions
 * 5. Handle AI Turn
 */
pub fn setup_dungeon_scheduler(app: &mut App) {
    setup_events(app);
    setup_stages(app);
    setup_bevy_internals(app);
    setup_debug_systems(app);

    app.add_plugin(systems::SystemsPlugin);
    app.add_plugin(gui::GUIPlugin);
}

fn setup_bevy_internals(app: &mut App) {
    app.init_resource::<Time>();
    app.add_system(|mut time: ResMut<Time>| time.update());
}

fn setup_events(app: &mut App) {
    app.add_event::<WantsToMove>()
        .add_event::<WantsToAttack>()
        .add_event::<SufferDamage>()
        .add_event::<WantsToPickupItem>()
        .add_event::<WantsToDrinkPotion>()
        .add_event::<WantsToDropItem>();
}

fn setup_stages(app: &mut App) {
    // Player Stages
    app.add_stage_after(
        CoreStage::Update,
        GameStage::GeneratePlayerActions,
        SystemStage::parallel(),
    )
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
    .add_stage_after(
        GameStage::GenerateAIActions,
        GameStage::HandleAIActions,
        SystemStage::parallel(),
    )
    .add_stage_after(
        GameStage::HandleAIActions,
        GameStage::AICleanup,
        SystemStage::parallel(),
    )
    .add_stage_after(
        GameStage::HandleAIActions,
        GameStage::Cleanup,
        SystemStage::parallel(),
    )
    // Render Stages
    .add_stage_after(
        GameStage::AICleanup,
        GameStage::Render,
        SystemStage::parallel(),
    );
}

fn setup_debug_systems(app: &mut App) {
    app.add_system_set_to_stage(
        CoreStage::Update,
        ConditionSet::new()
            .with_system(
                |m_q: Query<&Position, Added<Monster>>, i_q: Query<&Position, Added<Item>>| {
                    for pos in m_q.iter() {
                        eprintln!("Monster Spawned at {:?}", pos)
                    }
                    for pos in i_q.iter() {
                        eprintln!("Item Spawned at {:?}", pos)
                    }
                },
            )
            .into(),
    );
}

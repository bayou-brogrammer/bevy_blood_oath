use super::*;

fn game_over_input(mut commands: Commands, key: Option<Res<VirtualKeyCode>>) {
    if let Some(key) = key.as_deref() {
        match key {
            VirtualKeyCode::Escape => commands.insert_resource(AppExit),
            _ => commands.insert_resource(NextState(GameCondition::MainMenu)),
        }
    }
}

pub fn game_over(assets: Res<RexAssets>) {
    let mut draw_batch = DrawBatch::new();

    let sprite = MultiTileSprite::from_xp(&assets.skull);
    sprite.add_to_batch(&mut draw_batch, Point::new(SCREEN_WIDTH / 2 - 15, SCREEN_HEIGHT / 2 - 15));

    draw_batch.print_color_centered(15, "Your journey has ended!", ColorPair::new(YELLOW, BLACK));
    draw_batch.print_color_centered(
        17,
        "One day, we'll tell you all about how you did.",
        ColorPair::new(WHITE, BLACK),
    );
    draw_batch.print_color_centered(
        18,
        "That day, sadly, is not in this chapter..",
        ColorPair::new(WHITE, BLACK),
    );

    draw_batch.print_color_centered(
        19,
        &format!("You lived for {} turns.", bo_logging::get_event_count(TURN_DONE_EVENT)),
        ColorPair::new(WHITE, BLACK),
    );
    draw_batch.print_color_centered(
        20,
        &format!("You suffered {} points of damage.", bo_logging::get_event_count(DAMAGE_TAKE_EVENT)),
        ColorPair::new(RED, BLACK),
    );
    draw_batch.print_color_centered(
        21,
        &format!(
            "You inflicted {} points of damage.",
            bo_logging::get_event_count(DAMAGE_INFLICT_EVENT)
        ),
        ColorPair::new(RED, BLACK),
    );

    draw_batch.print_color_centered(
        23,
        "Press any key to return to the menu.",
        ColorPair::new(MAGENTA, BLACK),
    );

    draw_batch.submit(BATCH_ZERO).expect("Error batching title");
}

pub struct GameOverPlugin;
impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            ConditionSet::new()
                .run_in_state(GameCondition::GameOver)
                .with_system(game_over_input.chain(game_over))
                .into(),
        );
    }
}

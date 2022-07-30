use super::*;

#[derive(Debug)]
pub enum GameOver {
    NewGame,
    Quit,
}

impl ActionMenu<GameOver> for GameOver {
    fn actions() -> Vec<GameOver> {
        let mut actions = vec![GameOver::NewGame];

        #[cfg(not(target_arch = "wasm32"))]
        actions.push(GameOver::Quit);

        actions
    }

    fn label(&self) -> &'static str {
        match self {
            GameOver::NewGame => "New Game",
            GameOver::Quit => "Quit",
        }
    }
}

fn game_over_input(
    mut commands: Commands,
    mut selection: Local<usize>,
    mut exit: EventWriter<AppExit>,
    key: Option<Res<VirtualKeyCode>>,
) -> usize {
    if let Some(key) = key.as_deref() {
        let actions = GameOver::actions();

        match key {
            VirtualKeyCode::Escape => exit.send(AppExit),
            VirtualKeyCode::Down => {
                if *selection < actions.len().saturating_sub(1) {
                    *selection += 1;
                } else {
                    *selection = 0;
                }
            }
            VirtualKeyCode::Up => {
                if *selection > 0 {
                    *selection -= 1;
                } else {
                    *selection = actions.len().saturating_sub(1);
                }
            }
            VirtualKeyCode::Return => {
                assert!(*selection < actions.len());

                *selection = 0;
                match actions[*selection] {
                    GameOver::Quit => exit.send(AppExit),
                    GameOver::NewGame => commands.insert_resource(NextState(GameCondition::InGame)),
                }
            }
            _ => {}
        }
    }

    *selection
}

pub fn game_over(In(selection): In<usize>) {
    let mut batch = DrawBatch::new();

    let box_rect = bo_utils::prelude::center_box_with_title(
        &mut batch,
        (SCREEN_WIDTH, SCREEN_HEIGHT),
        BoxConfigWithTitle {
            box_config: BoxConfig::new((40, 20), ColorPair::new(WHITE, BLACK), true, false),
            text_config: TextConfig::new("GameOver", ColorPair::new(RED, BLACK), Alignment::Center),
        },
    );

    let mut y = SCREEN_HEIGHT / 2 - 10;
    batch.print_color_centered(
        y + 1,
        "Use Up/Down Arrows and Enter",
        ColorPair::new(RGB::named(GRAY), RGB::named(BLACK)),
    );

    y = box_rect.center().y - 2;
    for (i, action) in GameOver::actions().iter().enumerate() {
        let color = if i == selection { RGB::named(MAGENTA) } else { RGB::named(GRAY) };

        batch.print_color_centered(
            y + i as i32,
            action.label(),
            ColorPair::new(color, RGB::named(BLACK)),
        );
    }

    batch.submit(BATCH_ZERO).expect("Error batching title");
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

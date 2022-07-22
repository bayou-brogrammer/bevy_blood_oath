use super::*;

pub const MAIN_MENU_SCREEN_HEIGHT: usize = 31;

enum MainMenu {
    NewGame,
    LoadGame,
    Options,
    Quit,
}

impl ActionMenu<MainMenu> for MainMenu {
    fn label(&self) -> &'static str {
        match self {
            MainMenu::NewGame => "New Game",
            MainMenu::LoadGame => "Load Game",
            MainMenu::Options => "Options",
            MainMenu::Quit => "Quit",
        }
    }

    fn actions() -> Vec<MainMenu> {
        let mut actions = vec![MainMenu::NewGame];

        // There's no obvious way to get Emscripten to load the IndexedDB filesystem in time to
        // realize that a save file exists, so always include the Load Game option for it and just
        // check if there really is a save file when the option is chosen instead.
        if cfg!(target_os = "emscripten") {
            actions.push(MainMenu::LoadGame);
        }

        actions.push(MainMenu::Options);

        #[cfg(not(target_arch = "wasm32"))]
        actions.push(MainMenu::Quit);

        actions
    }
}

fn main_menu_input(
    mut selection: Local<usize>,
    key: Res<Option<VirtualKeyCode>>,
    mut commands: Commands,
) -> usize {
    if let Some(key) = key.as_ref() {
        let actions = MainMenu::actions();

        match key {
            VirtualKeyCode::Escape => commands.insert_resource(AppExit),
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

                match actions[*selection] {
                    MainMenu::Quit => commands.insert_resource(AppExit),
                    MainMenu::NewGame => {
                        commands.insert_resource(StateStack::new(TurnState::SetupDungeon));
                    }
                    _ => {} // Don't Handle loading or options yet.
                }
            }
            _ => commands.insert_resource(StateStack::new(TurnState::SetupDungeon)),
        }
    }

    *selection
}

fn main_menu_render(In(selection): In<usize>) {
    let mut batch = DrawBatch::new();
    batch.target(LAYER_TEXT);
    batch.cls();

    let box_rect = bo_utils::prelude::center_box_with_title(
        &mut batch,
        (SCREEN_WIDTH, SCREEN_HEIGHT),
        BoxConfigWithTitle {
            box_config: BoxConfig::new((40, 20), ColorPair::new(WHITE, BLACK), true, false),
            text_config: TextConfig::new(
                "BloodOath",
                ColorPair::new(RED, BLACK),
                Alignment::Center,
            ),
        },
    );

    let mut y = MAIN_MENU_SCREEN_HEIGHT / 2 - 10;
    batch.print_color_centered(
        y + 1,
        "by Jacob LeCoq",
        ColorPair::new(RGB::named(CYAN), RGB::named(BLACK)),
    );
    batch.print_color_centered(
        y + 2,
        "Use Up/Down Arrows and Enter",
        ColorPair::new(RGB::named(GRAY), RGB::named(BLACK)),
    );

    y = box_rect.center().y as usize - 2;
    for (i, action) in MainMenu::actions().iter().enumerate() {
        let color = if i == selection { RGB::named(MAGENTA) } else { RGB::named(GRAY) };

        batch.print_color_centered(y + i, action.label(), ColorPair::new(color, RGB::named(BLACK)));
    }

    batch.submit(BATCH_ZERO).expect("Error batching title");
}

////////////////////////////////////////////////////////////////////////////////

pub struct MainMenuPlugin;
impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            ConditionSet::new()
                .run_if(run_in_stack(TurnState::MainMenu))
                .with_system(main_menu_input.chain(main_menu_render))
                .into(),
        );
    }
}

////////////////////////////////////////////////////////////////////////////////

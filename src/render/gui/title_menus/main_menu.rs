use super::*;

#[derive(Debug)]
#[allow(dead_code)]
pub enum MainMenu {
    NewGame,
    LoadGame,
    Quit,
}

impl ActionMenu<MainMenu> for MainMenu {
    fn actions() -> Vec<MainMenu> {
        let mut actions = vec![MainMenu::NewGame];

        #[cfg(not(target_arch = "wasm32"))]
        actions.push(MainMenu::Quit);

        actions
    }

    fn label(&self) -> &'static str {
        match self {
            MainMenu::NewGame => "New Game",
            MainMenu::LoadGame => "Load Game",
            MainMenu::Quit => "Quit",
        }
    }
}

fn main_menu_input(
    mut commands: Commands,
    mut selection: Local<usize>,
    key: Option<Res<VirtualKeyCode>>,
) -> usize {
    if let Some(game_key) = key.as_deref().get_key() {
        let actions = MainMenu::actions();

        match game_key {
            GameKey::Escape => commands.insert_resource(AppExit),
            GameKey::Down => {
                if *selection < actions.len().saturating_sub(1) {
                    *selection += 1;
                } else {
                    *selection = 0;
                }
            }
            GameKey::Up => {
                if *selection > 0 {
                    *selection -= 1;
                } else {
                    *selection = actions.len().saturating_sub(1);
                }
            }
            GameKey::Select => {
                assert!(*selection < actions.len());

                match actions[*selection] {
                    MainMenu::Quit => commands.insert_resource(AppExit),
                    MainMenu::LoadGame => {}
                    MainMenu::NewGame => {
                        commands.insert_resource(NextState(GameCondition::MapGen(MapGenState::NewGame)));
                        *selection = 0;
                    }
                }
            }
            _ => {}
        }
    }

    *selection
}

pub fn main_menu(In(selection): In<usize>) {
    let mut batch = DrawBatch::new();
    batch.cls();

    let box_rect = crate::utils::center_box_with_title(
        &mut batch,
        (SCREEN_WIDTH, SCREEN_HEIGHT),
        BoxConfigWithTitle {
            box_config: BoxConfig::new((40, 20), ColorPair::new(WHITE, BLACK), true, false),
            text_config: TextConfig::new("BloodOath", ColorPair::new(RED, BLACK), Alignment::Center),
        },
    );

    let mut y = SCREEN_HEIGHT / 2 - 10;
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

    y = box_rect.center().y - 2;
    for (i, action) in MainMenu::actions().iter().enumerate() {
        let color = if i == selection { RGB::named(MAGENTA) } else { RGB::named(GRAY) };

        batch.print_color_centered(
            y + i as i32,
            action.label(),
            ColorPair::new(color, RGB::named(BLACK)),
        );
    }

    batch.submit(BATCH_ZERO).expect("Error batching title");
}

pub struct MainMenuPlugin;
impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            ConditionSet::new()
                .run_in_state(GameCondition::MainMenu)
                .with_system(main_menu_input.chain(main_menu))
                .into(),
        );
    }
}

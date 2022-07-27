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
    mut exit: EventWriter<AppExit>,
    key: Res<Option<VirtualKeyCode>>,
) -> usize {
    if let Some(key) = key.as_ref() {
        let actions = MainMenu::actions();

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

                match actions[*selection] {
                    MainMenu::Quit => exit.send(AppExit),
                    MainMenu::NewGame => commands.insert_resource(NextState(GameCondition::InGame)),
                    MainMenu::LoadGame => {}
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

    let box_rect = bo_utils::prelude::center_box_with_title(
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

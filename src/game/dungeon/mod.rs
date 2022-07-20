use super::*;
use crate::game_over::GameOverMode;
use bevy::{
    app::AppExit,
    ecs::{event::ManualEventReader, system::SystemState},
};
use setup::setup_dungeon_scheduler;

mod setup;
mod systems;

#[derive(Debug)]
pub enum DungeonModeResult {
    Done,
}

pub struct DungeonMode {
    app: App,
    consoles: Vec<usize>,
}

impl std::fmt::Debug for DungeonMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DungeonMode")
            .field("consoles", &self.consoles)
            .finish()
    }
}

struct CachedExitEvents<'w> {
    state: SystemState<(Res<'w, TurnState>, Option<Res<'w, AppExit>>)>,
}

/// The main gameplay mode.  The player can move around and explore the map, fight monsters and
/// perform other actions while alive, directly or indirectly.
impl DungeonMode {
    pub fn new() -> Self {
        BTerm::cls_all();
        let mut app = App::new();

        DungeonMode::setup_game(&mut app);

        let system_state: SystemState<(Res<TurnState>, Option<Res<AppExit>>)> =
            SystemState::new(&mut app.world);

        app.insert_resource(CachedExitEvents {
            state: system_state,
        });

        Self {
            app,
            consoles: vec![LAYER_MAP, LAYER_DECOR, LAYER_ITEMS, LAYER_CHARS, LAYER_TEXT],
        }
    }

    pub fn setup_game(app: &mut App) {
        BTerm::cls_all();

        // Setup Scheduler
        setup_dungeon_scheduler(app);

        let map = Map::new_map_rooms_and_corridors();
        let start_pos = map.starting_point;

        // Spawn Player
        let player = spawner::spawn_player(&mut app.world, start_pos);

        app.world
            .spawn()
            .insert_bundle(ItemBundle::new(
                EntityBundle::new(Item, "Health Potion", "A potion that restores health"),
                RenderBundle::new(
                    to_cp437('!'),
                    ColorPair::new(MAGENTA, BLACK),
                    RenderOrder::Item,
                    start_pos,
                ),
                Potion { heal_amount: 8 },
            ))
            .insert(InBackpack(player))
            .remove::<Position>();

        app.world
            .spawn()
            .insert_bundle(ItemBundle::new(
                EntityBundle::new(Item, "Health Potion", "A potion that restores health"),
                RenderBundle::new(
                    to_cp437('!'),
                    ColorPair::new(MAGENTA, BLACK),
                    RenderOrder::Item,
                    start_pos,
                ),
                Potion { heal_amount: 8 },
            ))
            .insert(InBackpack(player))
            .remove::<Position>();

        // Spawn Enemies
        map.rooms.iter().skip(1).for_each(|room| {
            spawner::spawn_room(&mut app.world, room);
        });

        let monsters = app
            .world
            .query::<&Monster>()
            .iter(&app.world)
            .collect::<Vec<_>>();
        println!("Monsters Gen: {:?}", monsters.len());

        // Resource
        app.insert_resource(map);
        app.insert_resource(start_pos);
        app.insert_resource(TurnState::AwaitingInput);

        crate::gamelog::Logger::new()
            .append("Welcome to")
            .append_with_color("Rusty Roguelike", CYAN)
            .log();
    }

    fn inject_context(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(LAYER_MAP);
        self.app.insert_resource(ctx.key);
        self.app
            .insert_resource(Mouse::new(ctx.mouse_pos(), ctx.left_click));
    }

    pub fn tick(&mut self, ctx: &mut BTerm, _pop_result: &Option<ModeResult>) -> ModeControl {
        self.inject_context(ctx);
        self.app.update();

        self.app
            .world
            .resource_scope(|world, mut cached_state: Mut<CachedExitEvents>| {
                let (turn_state, exit_event) = cached_state.state.get(world);

                match (exit_event, *turn_state) {
                    (None, TurnState::GameOver) => ModeControl::Switch(GameOverMode::new().into()),
                    (Some(_), _) => ModeControl::Pop(DungeonModeResult::Done.into()),
                    _ => ModeControl::Stay,
                }
            })
    }

    pub fn draw(&mut self, ctx: &mut BTerm, _active: bool) {
        render::clear_all_consoles(ctx, &self.consoles);
    }
}

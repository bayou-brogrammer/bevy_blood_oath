use super::{ModeControl, ModeResult, *};

////////////////////////////////////////////////////////////////////////////////
/// Mode
////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct MapGenMode {
    mapgen_timer: f32,
    mapgen_index: usize,
    mapgen_history: Vec<Map>,
    mapgen_next_state: Option<TurnState>,
}

/// Show the title screen of the game with a menu that leads into the game proper.
impl MapGenMode {
    pub fn new_game(world: &mut World) -> Self {
        let mut map_gen_mode = MapGenMode {
            mapgen_index: 0,
            mapgen_timer: 0.0,
            mapgen_history: Vec::new(),
            mapgen_next_state: Some(TurnState::AwaitingInput),
        };

        map_gen_mode.setup_new_game(world).expect("Failed to setup new game");
        map_gen_mode
    }

    pub fn next_level(world: &mut World) -> Self {
        let mut map_gen_mode = MapGenMode {
            mapgen_index: 0,
            mapgen_timer: 0.0,
            mapgen_history: Vec::new(),
            mapgen_next_state: Some(TurnState::AwaitingInput),
        };

        map_gen_mode.goto_level(world, 1);
        map_gen_mode
    }

    pub fn tick(
        &mut self,
        ctx: &mut BTerm,
        app: &mut App,
        _pop_result: &Option<ModeResult>,
    ) -> (ModeControl, ModeUpdate) {
        app.update();

        if !SHOW_MAPGEN_VISUALIZER {
            app.insert_resource(self.mapgen_next_state.unwrap());
            app.insert_resource(NextState(GameCondition::Playing));
            return (ModeControl::Switch(DungeonMode::new().into()), ModeUpdate::Update);
        }

        self.mapgen_timer += ctx.frame_time_ms;
        if self.mapgen_timer > 100.0 {
            self.mapgen_timer = 0.0;
            self.mapgen_index += 1;
            if self.mapgen_index >= self.mapgen_history.len() {
                app.insert_resource(self.mapgen_next_state.unwrap());
                app.insert_resource(NextState(GameCondition::Playing));
                return (ModeControl::Switch(DungeonMode::new().into()), ModeUpdate::Update);
            }
        }

        (ModeControl::Stay, ModeUpdate::Update)
    }

    pub fn draw(&self, ctx: &mut BTerm, _app: &mut App, _active: bool) {
        if let Some(map) = self.mapgen_history.get(self.mapgen_index) {
            let player_pos = Point::new(map.width / 2, map.height / 2);
            let (x_chars, y_chars) = ctx.get_char_size();

            let center_x = (x_chars / 2) as i32;
            let center_y = (y_chars / 2) as i32;

            let min_x = player_pos.x - center_x;
            let max_x = min_x + x_chars as i32;
            let min_y = player_pos.y - center_y;
            let max_y = min_y + y_chars as i32;

            let map_width = map.width;
            let map_height = map.height;

            let mut draw_batch = DrawBatch::new();
            draw_batch.target(LAYER_ZERO);

            // Render Map
            for (y, ty) in (min_y..max_y).enumerate() {
                for (x, tx) in (min_x..max_x).enumerate() {
                    let pt = Point::new(tx, ty);
                    if tx > 0 && tx < map_width && ty > 0 && ty < map_height {
                        let idx = map.point2d_to_index(pt);

                        if map.revealed.get_bit(pt) {
                            let (glyph, color) = tile_glyph(idx, &*map);
                            draw_batch.set(Point::new(x, y), color, glyph);
                        }
                    } else if SHOW_BOUNDARIES {
                        draw_batch.set(Point::new(x, y), ColorPair::new(GRAY, BLACK), to_cp437('·'));
                    }
                }
            }

            draw_batch.submit(BATCH_ZERO).expect("Failed to submit draw batch");
        }
    }
}

impl MapGenMode {
    fn setup_new_game(&mut self, world: &mut World) -> Result<(), BoxedError> {
        world.clear_entities();

        world.insert_resource(ParticleBuilder::new());
        world.insert_resource(MasterDungeonMap::new());
        world.insert_resource(Map::new(0, 80, 50, "Dummy Map"));

        self.generate_world_map(world, 1, 0);

        Ok(())
    }

    fn goto_level(&mut self, world: &mut World, offset: i32) {
        MasterDungeonMap::freeze_level_entities(world);

        // Build a new map and place the player
        let current_depth = world.resource::<Map>().depth;
        self.generate_world_map(world, current_depth + offset, offset);

        // Notify the player
        bo_logging::Logger::new().append("You change level.").log();
    }

    fn generate_world_map(&mut self, world: &mut World, new_depth: i32, offset: i32) {
        self.mapgen_index = 0;
        self.mapgen_timer = 0.0;
        self.mapgen_history.clear();

        let map_building_info = MasterDungeonMap::level_transition(world, new_depth, offset);
        match map_building_info {
            Some(history) => self.mapgen_history = history,
            None => MasterDungeonMap::thaw_level_entities(world),
        }

        bo_logging::clear_log();
        bo_logging::clear_events();
        bo_logging::Logger::new().append("Welcome to").color(CYAN).append("Rusty Roguelike").log();
    }
}

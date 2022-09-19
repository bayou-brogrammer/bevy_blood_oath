use super::*;
use std::collections::HashSet;

#[derive(Debug)]
pub enum TargetingModeResult {
    Cancelled,
    Target(Entity, Point),
}

pub struct TargetingMode {
    radius: i32,
    item: Entity,
    warn_self: bool,
    item_name: String,
    camera: CameraView,
    player_positon: Point,
    active_mouse_pt: Point,
    valid_cells: HashSet<Point>,
}

/// Pick a target position within a certain range of the player.
impl TargetingMode {
    pub fn new(term: &mut BTerm, world: &World, item: Entity, range: i32, warn_self: bool) -> Self {
        let item_name = world.get::<Naming>(item).unwrap().0.clone();
        let radius = world.get::<AreaOfEffect>(item).map_or(0, |aoe| aoe.radius);

        assert!(range >= 0);
        assert!(radius >= 0);

        let player = world.resource::<Entity>();
        let player_positon = *world.resource::<Point>();

        let mut valid_cells = HashSet::new();
        if let Some(fov) = world.get::<FieldOfView>(*player) {
            let map = world.resource::<Map>();
            valid_cells = fov
                .visible_tiles
                .iter()
                .filter(|pt| DistanceAlg::Pythagoras.distance2d(player_positon, **pt) < range as f32)
                .filter(|pt| map.tiles[map.point2d_to_index(**pt)].walkable)
                .copied()
                .collect::<HashSet<Point>>();
        }

        Self {
            item,
            radius,
            warn_self,
            item_name,
            valid_cells,
            player_positon,
            active_mouse_pt: term.mouse_point(),
            camera: *world.resource::<CameraView>(),
        }
    }

    fn should_warn(&self) -> bool {
        if self.warn_self {
            let map_mouse_pos = self.camera.screen_to_world(self.active_mouse_pt);
            let distance = DistanceAlg::Pythagoras.distance2d(self.player_positon, map_mouse_pos);
            if self.player_positon == map_mouse_pos || (self.radius > 0 && distance <= self.radius as f32) {
                return true;
            }
        }

        false
    }
}

impl State for TargetingMode {
    type State = GameWorld;
    type StateResult = ModeResult;

    fn update(
        &mut self,
        term: &mut BTerm,
        _state: &mut Self::State,
        pop_result: &Option<Self::StateResult>,
    ) -> StateReturn<Self::State, Self::StateResult> {
        if let Some(result) = pop_result {
            return match result {
                ModeResult::YesNoDialogModeResult(result) => match result {
                    YesNoDialogModeResult::No => (Transition::Stay, TransitionControl::Update),
                    YesNoDialogModeResult::Yes => (
                        Transition::Pop(
                            TargetingModeResult::Target(
                                self.item,
                                self.camera.screen_to_world(self.active_mouse_pt),
                            )
                            .into(),
                        ),
                        TransitionControl::Immediate,
                    ),
                },
                _ => (Transition::Stay, TransitionControl::Update),
            };
        }

        let game_key = term.get_key();

        // Handle Escaping
        if game_key == Some(GameKey::Escape) {
            return (Transition::Pop(TargetingModeResult::Cancelled.into()), TransitionControl::Update);
        }

        // Handle Left Mouse || Resturn Key Press
        if game_key == Some(GameKey::Select) || term.left_click {
            let map_mouse_pos = self.camera.screen_to_world(self.active_mouse_pt);

            let result = if self.should_warn() {
                Transition::Push(
                    YesNoDialogMode::new(
                        format!(
                            "Really {} yourself?",
                            if map_mouse_pos == self.player_positon { "target" } else { "include" },
                        ),
                        false,
                    )
                    .boxed(),
                )
            } else {
                Transition::Pop(TargetingModeResult::Target(self.item, map_mouse_pos).into())
            };

            return (result, TransitionControl::Immediate);
        }

        (Transition::Stay, TransitionControl::Update)
    }

    fn render(&mut self, term: &mut BTerm, state: &mut Self::State, active: bool) {
        match (active, term.screen_burn_color == REGULAR_SCREEN_BURN.into()) {
            (true, false) => term.screen_burn_color(REGULAR_SCREEN_BURN.into()),
            (false, true) => term.screen_burn_color(RGB::named(LIGHTGRAY)),
            _ => {}
        }

        state.render_schedule.run(&mut state.app.world);

        let mut draw_batch = DrawBatch::new();
        draw_batch.target(LAYER_ZERO);

        draw_batch.print_color(
            Point::new(2, 2),
            format!("Select Target for {}", self.item_name),
            ColorPair::new(YELLOW, BLACK),
        );

        // Draw potential valid cells
        self.valid_cells.iter().for_each(|pt| {
            let screen_pt = self.camera.world_to_screen(*pt);
            draw_batch.set_bg(screen_pt, BLUE);
        });

        // Draw Blast Radius
        self.active_mouse_pt = if active { term.mouse_point() } else { self.active_mouse_pt };
        let mouse_map_pos = self.camera.screen_to_world(self.active_mouse_pt);

        if self.radius > 0 {
            let map = state.app.world.resource::<Map>();
            field_of_view_set(mouse_map_pos, self.radius, map)
                .iter()
                .filter(|pt| map.visible.get_bit(**pt))
                .for_each(|pt| {
                    let screen_pt = self.camera.world_to_screen(*pt);
                    draw_batch.set_bg(screen_pt, LIGHT_RED);
                });
        }

        // Draw Target Status
        let is_valid_target = self.valid_cells.iter().filter(|pt| **pt == mouse_map_pos).count() > 0;
        if is_valid_target {
            draw_batch.set_bg(self.active_mouse_pt, GREEN);
        } else {
            draw_batch.set_bg(self.active_mouse_pt, RED);
        }

        draw_batch.submit(BATCH_UI).expect("Batch error");
    }

    fn draw_behind(&self) -> bool {
        false
    }
}

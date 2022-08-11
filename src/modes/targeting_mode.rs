use super::*;
use std::collections::HashSet;

#[derive(Debug)]
pub enum TargetingModeResult {
    Cancelled,
    Target(Entity, Point),
}

#[derive(Debug)]
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
    pub fn new(ctx: &mut BTerm, world: &World, item: Entity, range: i32, warn_self: bool) -> Self {
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
            active_mouse_pt: ctx.mouse_point(),
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

    pub fn tick(
        &mut self,
        ctx: &mut BTerm,
        _app: &mut App,
        pop_result: &Option<ModeResult>,
    ) -> (ModeControl, ModeUpdate) {
        if let Some(result) = pop_result {
            return match result {
                ModeResult::YesNoDialogModeResult(result) => match result {
                    YesNoDialogModeResult::No => (ModeControl::Stay, ModeUpdate::Update),
                    YesNoDialogModeResult::Yes => (
                        ModeControl::Pop(
                            TargetingModeResult::Target(
                                self.item,
                                self.camera.screen_to_world(self.active_mouse_pt),
                            )
                            .into(),
                        ),
                        ModeUpdate::Immediate,
                    ),
                },
                _ => (ModeControl::Stay, ModeUpdate::Update),
            };
        }

        let game_key = ctx.get_key();

        // Handle Escaping
        if game_key == Some(GameKey::Escape) {
            return (ModeControl::Pop(TargetingModeResult::Cancelled.into()), ModeUpdate::Update);
        }

        // Handle Left Mouse || Resturn Key Press
        if game_key == Some(GameKey::Select) || ctx.left_click {
            let map_mouse_pos = self.camera.screen_to_world(self.active_mouse_pt);

            let result = if self.should_warn() {
                ModeControl::Push(
                    YesNoDialogMode::new(
                        format!(
                            "Really {} yourself?",
                            if map_mouse_pos == self.player_positon { "target" } else { "include" },
                        ),
                        false,
                    )
                    .into(),
                )
            } else {
                ModeControl::Pop(TargetingModeResult::Target(self.item, map_mouse_pos).into())
            };

            return (result, ModeUpdate::Immediate);
        }

        (ModeControl::Stay, ModeUpdate::Update)
    }

    pub fn draw(&mut self, ctx: &mut BTerm, world: &mut World, active: bool) {
        match (active, ctx.screen_burn_color == REGULAR_SCREEN_BURN.into()) {
            (true, false) => ctx.screen_burn_color(REGULAR_SCREEN_BURN.into()),
            (false, true) => ctx.screen_burn_color(RGB::named(LIGHTGRAY)),
            _ => {}
        }

        let mut draw_batch = DrawBatch::new();
        draw_batch.target(LAYER_ZERO);

        draw_batch.set_bg(Point::new(0, 0), GREEN);

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
        self.active_mouse_pt = if active { ctx.mouse_point() } else { self.active_mouse_pt };
        let mouse_map_pos = self.camera.screen_to_world(self.active_mouse_pt);

        if self.radius > 0 {
            let map = world.resource::<Map>();
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
}

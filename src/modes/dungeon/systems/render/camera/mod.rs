use super::*;
use bracket_lib::prelude::Rect;

mod entity_renderer;
mod map_renderer;
mod tooltips;

pub struct GameCamera {
    viewport: Rect,
    player_pos: Point,
}

impl GameCamera {
    pub fn new(player_pos: Point) -> Self {
        let viewport = Rect::with_size(player_pos.x - 20, player_pos.y - 15, 40, 31);

        Self { viewport, player_pos }
    }

    fn world_to_screen(&self, pt: Point) -> Point {
        let bot = pt - self.player_pos;
        bot + Point::new(20, 15)
    }

    fn world_to_screen_text(&self, pt: Point) -> Point {
        let ws = self.world_to_screen(pt);
        ws * Point::new(2, 1)
    }

    fn screen_to_world(&self, mouse_x: i32, mouse_y: i32) -> Point {
        Point::new(mouse_x + self.viewport.x1, mouse_y + self.viewport.y1)
    }
}

////////////////////////////////////////////////////////////////////////////////

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            ConditionSet::new()
                .run_if(run_not_in_state(TurnState::GameOver))
                .after(StateLabel::Fov)
                .with_system(map_renderer::map_render)
                .with_system(entity_renderer::entity_render)
                .with_system(entity_renderer::item_render)
                .with_system(entity_renderer::particle_render)
                .with_system(tooltips::render_tooltips)
                .into(),
        );
    }
}

////////////////////////////////////////////////////////////////////////////////

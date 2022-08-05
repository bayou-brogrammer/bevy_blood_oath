use super::*;

pub mod entity_renderer;
pub mod map_renderer;
pub mod tooltips;

#[derive(Debug, Copy, Clone)]
pub struct GameCamera {
    player_pos: Point,
}

impl GameCamera {
    pub fn new(player_pos: Point) -> Self {
        GameCamera { player_pos }
    }

    pub fn get_screen_bounds(&self) -> (i32, i32, i32, i32) {
        let (x_chars, y_chars) = (48, 44);

        let center_x = (x_chars / 2) as i32;
        let center_y = (y_chars / 2) as i32;

        let min_x = self.player_pos.x - center_x;
        let max_x = min_x + x_chars as i32;
        let min_y = self.player_pos.y - center_y;
        let max_y = min_y + y_chars as i32;

        (min_x, max_x, min_y, max_y)
    }

    pub fn screen_to_world(&self, pt: Point) -> Point {
        let (min_x, _, min_y, _) = self.get_screen_bounds();
        Point::new(pt.x - min_x, pt.y - min_y) + Point::new(1, 1)
    }

    pub fn world_to_screen(&self, pt: Point) -> Point {
        let (min_x, _, min_y, _) = self.get_screen_bounds();
        Point::new(pt.x + min_x, pt.y + min_y) + Point::new(-1, -1)
    }
}

use super::*;

pub struct SimpleMapBuilder {}

impl InitialMapBuilder for SimpleMapBuilder {
    fn build_map(&mut self, build_data: &mut BuilderMap) {
        self.build_rooms(build_data);
    }
}

impl SimpleMapBuilder {
    pub fn new() -> Box<SimpleMapBuilder> {
        Box::new(SimpleMapBuilder {})
    }

    fn build_rooms(&mut self, build_data: &mut BuilderMap) {
        const MAX_ROOMS: i32 = 25;
        const MIN_SIZE: i32 = 6;
        const MAX_SIZE: i32 = 10;

        let mut rooms: Vec<Rect> = Vec::new();

        for _i in 0..MAX_ROOMS {
            let w = bo_utils::rng::range(MIN_SIZE, MAX_SIZE);
            let h = bo_utils::rng::range(MIN_SIZE, MAX_SIZE);
            let x = bo_utils::rng::roll_dice(1, build_data.map.width - w - 1) - 1;
            let y = bo_utils::rng::roll_dice(1, build_data.map.height - h - 1) - 1;
            let new_room = Rect::with_size(x, y, w, h);

            let ok = rooms.iter().all(|room| !new_room.intersect(room));
            if ok {
                rooms.push(new_room);
            }
        }

        build_data.rooms = Some(rooms);
    }
}

use super::*;
pub struct RoomBasedStairs {}

impl MetaMapBuilder for RoomBasedStairs {
    fn build_map(&mut self, build_data: &mut BuilderMap) {
        self.build(build_data);
    }
}

impl RoomBasedStairs {
    pub fn new() -> Box<RoomBasedStairs> {
        Box::new(RoomBasedStairs {})
    }

    fn build(&mut self, build_data: &mut BuilderMap) {
        if let Some(rooms) = &build_data.rooms {
            let stairs_position = rooms[rooms.len() - 1].center();
            let stairs_idx = build_data.map.point2d_to_index(stairs_position);
            build_data.map.tiles[stairs_idx] = GameTile::stairs_down();
            build_data.take_snapshot();
        } else {
            panic!("Room Based Stairs only works after rooms have been created");
        }
    }
}

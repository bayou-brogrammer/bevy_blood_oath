use super::*;
use std::collections::HashSet;

pub struct StraightLineCorridors {}

impl MetaMapBuilder for StraightLineCorridors {
    fn build_map(&mut self, build_data: &mut BuilderMap) {
        self.corridors(build_data);
    }
}

impl StraightLineCorridors {
    pub fn new() -> Box<StraightLineCorridors> {
        Box::new(StraightLineCorridors {})
    }

    fn corridors(&mut self, build_data: &mut BuilderMap) {
        let rooms: Vec<Rect>;
        if let Some(rooms_builder) = &build_data.rooms {
            rooms = rooms_builder.clone();
        } else {
            panic!("Straight Line Corridors require a builder with room structures");
        }

        let mut connected: HashSet<usize> = HashSet::new();
        let mut corridors: Vec<Vec<usize>> = Vec::new();
        for (i, room) in rooms.iter().enumerate() {
            let mut room_distance: Vec<(usize, f32)> = Vec::new();
            let room_center = room.center();

            for (j, other_room) in rooms.iter().enumerate() {
                if i != j && !connected.contains(&j) {
                    let other_center = other_room.center();
                    let distance = DistanceAlg::Pythagoras.distance2d(room_center, other_center);
                    room_distance.push((j, distance));
                }
            }

            if !room_distance.is_empty() {
                room_distance.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
                let dest_center = rooms[room_distance[0].0].center();
                let line = line2d(LineAlg::Bresenham, room_center, dest_center);

                let mut corridor = Vec::new();
                for cell in line.iter() {
                    let idx = build_data.map.xy_idx(cell.x, cell.y);
                    if build_data.map.tiles[idx].tile_type != TileType::Floor {
                        build_data.map.tiles[idx] = GameTile::floor();
                        corridor.push(idx);
                    }
                }

                corridors.push(corridor);
                connected.insert(i);
                build_data.take_snapshot();
            }
        }
        build_data.corridors = Some(corridors);
    }
}

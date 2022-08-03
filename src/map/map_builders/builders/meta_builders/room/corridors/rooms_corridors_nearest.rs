use super::*;
use std::collections::HashSet;

pub struct NearestCorridors {}

impl MetaMapBuilder for NearestCorridors {
    fn build_map(&mut self, build_data: &mut BuilderMap) {
        self.corridors(build_data);
    }
}

impl NearestCorridors {
    pub fn new() -> Box<NearestCorridors> {
        Box::new(NearestCorridors {})
    }

    fn corridors(&mut self, build_data: &mut BuilderMap) {
        let rooms: Vec<Rect>;
        if let Some(rooms_builder) = &build_data.rooms {
            rooms = rooms_builder.clone();
        } else {
            panic!("Nearest Corridors require a builder with room structures");
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
                let corridor = draw_corridor(
                    &mut build_data.map,
                    room_center.x,
                    room_center.y,
                    dest_center.x,
                    dest_center.y,
                );
                connected.insert(i);
                build_data.take_snapshot();
                corridors.push(corridor);
            }
        }
        build_data.corridors = Some(corridors);
    }
}

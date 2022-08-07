use super::*;

pub struct CullUnreachable {}

impl MetaMapBuilder for CullUnreachable {
    fn build_map(&mut self, build_data: &mut BuilderMap) {
        self.build(build_data);
    }
}

impl CullUnreachable {
    pub fn new() -> Box<CullUnreachable> {
        Box::new(CullUnreachable {})
    }

    fn build(&mut self, build_data: &mut BuilderMap) {
        let starting_pos = *build_data.starting_position.as_ref().unwrap();
        let start_idx = build_data.map.xy_idx(starting_pos.x, starting_pos.y);
        crate::spatial::populate_blocked_from_map(&build_data.map);

        let map_starts: Vec<usize> = vec![start_idx];
        let dijkstra_map = DijkstraMap::new(
            build_data.map.width as usize,
            build_data.map.height as usize,
            &map_starts,
            &build_data.map,
            1000.0,
        );

        for (i, tile) in build_data.map.tiles.iter_mut().enumerate() {
            if tile.tile_type == TileType::Floor {
                let distance_to_start = dijkstra_map.map[i];
                // We can't get to this tile - so we'll make it a wall
                if distance_to_start == std::f32::MAX {
                    *tile = GameTile::wall()
                }
            }
        }
    }
}

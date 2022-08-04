use super::*;

pub struct WallBoundaries {}

impl MetaMapBuilder for WallBoundaries {
    fn build_map(&mut self, build_data: &mut BuilderMap) {
        self.build(build_data);
    }
}

impl WallBoundaries {
    pub fn new() -> Box<WallBoundaries> {
        Box::new(WallBoundaries {})
    }

    fn build(&mut self, build_data: &mut BuilderMap) {
        // Make the boundaries walls
        for x in 0..build_data.map.width {
            let idx_1 = build_data.map.xy_idx(x, 1);
            let idx_2 = build_data.map.xy_idx(x, build_data.map.height - 1);

            build_data.map.tiles[idx_1] = GameTile::wall();
            build_data.map.tiles[idx_2] = GameTile::wall();
        }
        build_data.take_snapshot();

        for y in 0..build_data.map.height {
            let idx_1 = build_data.map.xy_idx(1, y);
            let idx_2 = build_data.map.xy_idx(build_data.map.width - 1, y);

            build_data.map.tiles[idx_1] = GameTile::wall();
            build_data.map.tiles[idx_2] = GameTile::wall();
        }
        build_data.take_snapshot();
    }
}

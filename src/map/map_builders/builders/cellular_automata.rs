use super::*;

pub struct CellularAutomataBuilder {}

impl InitialMapBuilder for CellularAutomataBuilder {
    fn build_map(&mut self, build_data: &mut BuilderMap) { self.build(build_data); }
}

impl MetaMapBuilder for CellularAutomataBuilder {
    fn build_map(&mut self, build_data: &mut BuilderMap) { self.apply_iteration(build_data); }
}

impl CellularAutomataBuilder {
    pub fn new() -> Box<CellularAutomataBuilder> { Box::new(CellularAutomataBuilder {}) }

    fn apply_iteration(&mut self, build_data: &mut BuilderMap) {
        let mut newtiles = build_data.map.tiles.clone();

        for y in 1..build_data.map.height - 1 {
            for x in 1..build_data.map.width - 1 {
                let idx = build_data.map.xy_idx(x, y);
                let mut neighbors = 0;
                if build_data.map.tiles[idx - 1].tile_type == TileType::Wall {
                    neighbors += 1;
                }
                if build_data.map.tiles[idx + 1].tile_type == TileType::Wall {
                    neighbors += 1;
                }
                if build_data.map.tiles[idx - build_data.map.width as usize].tile_type == TileType::Wall {
                    neighbors += 1;
                }
                if build_data.map.tiles[idx + build_data.map.width as usize].tile_type == TileType::Wall {
                    neighbors += 1;
                }
                if build_data.map.tiles[idx - (build_data.map.width as usize - 1)].tile_type == TileType::Wall
                {
                    neighbors += 1;
                }
                if build_data.map.tiles[idx - (build_data.map.width as usize + 1)].tile_type == TileType::Wall
                {
                    neighbors += 1;
                }
                if build_data.map.tiles[idx + (build_data.map.width as usize - 1)].tile_type == TileType::Wall
                {
                    neighbors += 1;
                }
                if build_data.map.tiles[idx + (build_data.map.width as usize + 1)].tile_type == TileType::Wall
                {
                    neighbors += 1;
                }

                if neighbors > 4 || neighbors == 0 {
                    newtiles[idx] = GameTile::wall()
                } else {
                    newtiles[idx] = GameTile::floor()
                }
            }
        }

        build_data.map.tiles = newtiles.clone();
        build_data.take_snapshot();
    }

    fn build(&mut self, build_data: &mut BuilderMap) {
        // First we completely randomize the map, setting 55% of it to be floor.
        for y in 1..build_data.map.height - 1 {
            for x in 1..build_data.map.width - 1 {
                let roll = crate::rng::roll_dice(1, 100);
                let idx = build_data.map.xy_idx(x, y);
                if roll > 55 {
                    build_data.map.tiles[idx] = GameTile::floor()
                } else {
                    build_data.map.tiles[idx] = GameTile::wall()
                }
            }
        }
        build_data.take_snapshot();

        // Now we iteratively apply cellular automata rules
        for _i in 0..15 {
            self.apply_iteration(build_data);
        }
    }
}

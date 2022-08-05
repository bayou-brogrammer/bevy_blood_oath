use crate::prelude::*;

impl GameTile {
    pub fn get_tile_glyph_default(&self, map: &Map, idx: usize) -> (FontCharType, ColorPair) {
        let (glyph, fg) = match self.tile_type {
            TileType::Wall => {
                let pt = map.index_to_point2d(idx);
                (wall_glyph(&*map, pt.x, pt.y), RGB::named(GREEN))
            }
            _ => (self.glyph, self.color.fg.into()),
        };

        (glyph, ColorPair::new(fg, BLACK))
    }

    pub fn get_forest_glyph(&self) -> (FontCharType, ColorPair) {
        let (glyph, fg) = match self.tile_type {
            TileType::Wall => (to_cp437('♣'), RGB::from_f32(0.0, 0.6, 0.0)),
            TileType::Road => (self.glyph, RGB::named(YELLOW)),
            TileType::Grass => (self.glyph, self.color.fg.into()),
            TileType::Bridge => (self.glyph, self.color.fg.into()),
            TileType::Gravel => (self.glyph, self.color.fg.into()),
            TileType::UpStairs => (self.glyph, self.color.fg.into()),
            TileType::DeepWater => (self.glyph, self.color.fg.into()),
            TileType::DownStairs => (self.glyph, self.color.fg.into()),
            TileType::ShallowWater => (self.glyph, self.color.fg.into()),
            _ => (to_cp437('"'), RGB::from_f32(0.4, 0.4, 0.4)),
        };

        (glyph, ColorPair::new(fg, BLACK))
    }

    pub fn get_limestone_glyph(&self) -> (FontCharType, ColorPair) {
        let (glyph, fg) = match self.tile_type {
            TileType::Wall => (to_cp437('▒'), RGB::named(GRAY61)),
            TileType::Road => (self.glyph, RGB::named(YELLOW)),
            TileType::Grass => (self.glyph, self.color.fg.into()),
            TileType::Bridge => (self.glyph, self.color.fg.into()),
            TileType::Gravel => (self.glyph, self.color.fg.into()),
            TileType::UpStairs => (self.glyph, self.color.fg.into()),
            TileType::DeepWater => (to_cp437('░'), RGB::from_f32(0.2, 0.2, 1.0)),
            TileType::Stalactite => (self.glyph, RGB::named(GRAY61)),
            TileType::Stalagmite => (self.glyph, RGB::named(GRAY61)),
            TileType::DownStairs => (self.glyph, self.color.fg.into()),
            TileType::ShallowWater => (to_cp437('░'), self.color.fg.into()),
            _ => (to_cp437('\''), RGB::from_f32(0.0, 0.6, 0.0)),
        };

        (glyph, ColorPair::new(fg, BLACK))
    }
}

#[rustfmt::skip]
fn wall_glyph(map: &Map, x: i32, y: i32) -> FontCharType {
    if x < 1 || x > map.width - 1 || y < 1 || y > map.height - 1 {
        return 35;
    }
    let mut mask : u8 = 0;

    if is_revealed_and_wall(map, x, y - 1) { mask +=1; }
    if is_revealed_and_wall(map, x, y + 1) { mask +=2; }
    if is_revealed_and_wall(map, x - 1, y) { mask +=4; }
    if is_revealed_and_wall(map, x + 1, y) { mask +=8; }

    match mask {
        0 => { 9 } // Pillar because we can't see neighbors
        1 => { 186 } // Wall only to the north
        2 => { 186 } // Wall only to the south
        3 => { 186 } // Wall to the north and south
        4 => { 205 } // Wall only to the west
        5 => { 188 } // Wall to the north and west
        6 => { 187 } // Wall to the south and west
        7 => { 185 } // Wall to the north, south and west
        8 => { 205 } // Wall only to the east
        9 => { 200 } // Wall to the north and east
        10 => { 201 } // Wall to the south and east
        11 => { 204 } // Wall to the north, south and east
        12 => { 205 } // Wall to the east and west
        13 => { 202 } // Wall to the east, west, and south
        14 => { 203 } // Wall to the east, west, and north
        15 => { 206 }  // ╬ Wall on all sides
        _ => { 35 } // We missed one?
    }
}

fn is_revealed_and_wall(map: &Map, x: i32, y: i32) -> bool {
    let pt = Point::new(x, y);
    if map.in_bounds(pt) {
        let idx = map.point2d_to_index(pt);
        map.tiles[idx].tile_type == TileType::Wall && map.revealed.get_bit(pt)
    } else {
        false
    }
}

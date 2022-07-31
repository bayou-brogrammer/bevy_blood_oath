use crate::prelude::*;
use bracket_geometry::prelude::*;
use bracket_pathfinding::prelude::*;
use bracket_terminal::prelude::*;

pub fn tile_glyph(idx: usize, map: &Map) -> (FontCharType, ColorPair) {
    let (glyph, mut color) = match map.depth {
        3 => get_limestone_cavern_glyph(idx, map),
        2 => get_forest_glyph(idx, map),
        _ => get_tile_glyph_default(idx, map),
    };

    if map.bloodstains.contains_key(&idx) {
        color.bg = (*map.bloodstains.get(&idx).unwrap()).into();
    }

    if !map.visible.get_bit(map.index_to_point2d(idx)) {
        color.fg = color.fg.to_greyscale();
        color.bg = RGBA::from_f32(0., 0., 0., 0.); // Don't show stains out of visual range
    }

    (glyph, color)
}

fn get_forest_glyph(idx: usize, map: &Map) -> (FontCharType, ColorPair) {
    let glyph;
    let fg;
    let bg = RGB::from_f32(0., 0., 0.);

    match map.tiles[idx].tile_type {
        TileType::Wall => {
            glyph = to_cp437('♣');
            fg = RGB::from_f32(0.0, 0.6, 0.0);
        }
        TileType::Bridge => {
            glyph = to_cp437('.');
            fg = RGB::named(CHOCOLATE);
        }
        TileType::Road => {
            glyph = to_cp437('≡');
            fg = RGB::named(YELLOW);
        }
        TileType::Grass => {
            glyph = to_cp437('"');
            fg = RGB::named(GREEN);
        }
        TileType::ShallowWater => {
            glyph = to_cp437('~');
            fg = RGB::named(CYAN);
        }
        TileType::DeepWater => {
            glyph = to_cp437('~');
            fg = RGB::named(BLUE);
        }
        TileType::Gravel => {
            glyph = to_cp437(';');
            fg = RGB::from_f32(0.5, 0.5, 0.5);
        }
        TileType::DownStairs => {
            glyph = to_cp437('>');
            fg = RGB::from_f32(0., 1.0, 1.0);
        }
        TileType::UpStairs => {
            glyph = to_cp437('<');
            fg = RGB::from_f32(0., 1.0, 1.0);
        }
        _ => {
            glyph = to_cp437('"');
            fg = RGB::from_f32(0.0, 0.6, 0.0);
        }
    }

    (glyph, ColorPair::new(fg, bg))
}

fn get_limestone_cavern_glyph(idx: usize, map: &Map) -> (FontCharType, ColorPair) {
    let glyph;
    let fg;
    let bg = RGB::from_f32(0., 0., 0.);

    match map.tiles[idx].tile_type {
        TileType::Wall => {
            glyph = to_cp437('▒');
            fg = RGB::from_f32(0.7, 0.7, 0.7);
        }
        TileType::Bridge => {
            glyph = to_cp437('.');
            fg = RGB::named(CHOCOLATE);
        }
        TileType::Road => {
            glyph = to_cp437('≡');
            fg = RGB::named(YELLOW);
        }
        TileType::Grass => {
            glyph = to_cp437('"');
            fg = RGB::named(GREEN);
        }
        TileType::ShallowWater => {
            glyph = to_cp437('░');
            fg = RGB::named(CYAN);
        }
        TileType::DeepWater => {
            glyph = to_cp437('▓');
            fg = RGB::named(BLUE);
        }
        TileType::Gravel => {
            glyph = to_cp437(';');
            fg = RGB::from_f32(0.5, 0.5, 0.5);
        }
        TileType::DownStairs => {
            glyph = to_cp437('>');
            fg = RGB::from_f32(0., 1.0, 1.0);
        }
        TileType::UpStairs => {
            glyph = to_cp437('<');
            fg = RGB::from_f32(0., 1.0, 1.0);
        }
        TileType::Stalactite => {
            glyph = to_cp437('╨');
            fg = RGB::from_f32(0.7, 0.7, 0.7);
        }
        TileType::Stalagmite => {
            glyph = to_cp437('╥');
            fg = RGB::from_f32(0.7, 0.7, 0.7);
        }
        _ => {
            glyph = to_cp437('\'');
            fg = RGB::from_f32(0.4, 0.4, 0.4);
        }
    }

    (glyph, ColorPair::new(fg, bg))
}

fn get_tile_glyph_default(idx: usize, map: &Map) -> (FontCharType, ColorPair) {
    let tile = &map.tiles[idx];

    let (glyph, fg) = match map.tiles[idx].tile_type {
        TileType::Floor => (tile.glyph, RGB::from_f32(0.0, 0.5, 0.5)),
        TileType::WoodFloor => {
            // glyph = to_cp437('░');
            (tile.glyph, RGB::named(CHOCOLATE))
        }
        TileType::Wall => {
            let pt = map.index_to_point2d(idx);
            (wall_glyph(&*map, pt.x, pt.y), RGB::from_f32(0., 1.0, 0.))
        }
        TileType::DownStairs => {
            // glyph = to_cp437('>');
            (tile.glyph, RGB::from_f32(0., 1.0, 1.0))
        }
        TileType::UpStairs => {
            // glyph = to_cp437('<');
            (tile.glyph, RGB::from_f32(0., 1.0, 1.0))
        }
        TileType::Bridge => {
            // glyph = to_cp437('.');
            (tile.glyph, RGB::named(CHOCOLATE))
        }
        TileType::Road => {
            // glyph = to_cp437('≡');
            (tile.glyph, RGB::named(GRAY))
        }
        TileType::Grass => {
            // glyph = to_cp437('"');
            (tile.glyph, RGB::named(GREEN))
        }
        TileType::ShallowWater => {
            // glyph = to_cp437('~');
            (tile.glyph, RGB::named(CYAN))
        }
        TileType::DeepWater => {
            // glyph = to_cp437('~');
            (tile.glyph, RGB::named(BLUE))
        }
        TileType::Gravel => {
            // glyph = to_cp437(';');
            (tile.glyph, RGB::from_f32(0.5, 0.5, 0.5))
        }
        TileType::Stalactite => {
            // glyph = to_cp437('╨');
            (tile.glyph, RGB::from_f32(0.5, 0.5, 0.5))
        }
        TileType::Stalagmite => {
            // glyph = to_cp437('╥');
            (tile.glyph, RGB::from_f32(0.5, 0.5, 0.5))
        }
    };

    (glyph, ColorPair::new(fg, RGB::from_f32(0., 0., 0.)))
}

fn wall_glyph(map: &Map, x: i32, y: i32) -> FontCharType {
    if x < 1 || x > map.width - 2 || y < 1 || y > map.height - 2_i32 {
        return 35;
    }
    let mut mask: u8 = 0;

    if is_revealed_and_wall(map, x, y - 1) {
        mask += 1;
    }
    if is_revealed_and_wall(map, x, y + 1) {
        mask += 2;
    }
    if is_revealed_and_wall(map, x - 1, y) {
        mask += 4;
    }
    if is_revealed_and_wall(map, x + 1, y) {
        mask += 8;
    }

    match mask {
        0 => 9,    // Pillar because we can't see neighbors
        1 => 186,  // Wall only to the north
        2 => 186,  // Wall only to the south
        3 => 186,  // Wall to the north and south
        4 => 205,  // Wall only to the west
        5 => 188,  // Wall to the north and west
        6 => 187,  // Wall to the south and west
        7 => 185,  // Wall to the north, south and west
        8 => 205,  // Wall only to the east
        9 => 200,  // Wall to the north and east
        10 => 201, // Wall to the south and east
        11 => 204, // Wall to the north, south and east
        12 => 205, // Wall to the east and west
        13 => 202, // Wall to the east, west, and south
        14 => 203, // Wall to the east, west, and north
        15 => 206, // ╬ Wall on all sides
        _ => 35,   // We missed one?
    }
}

fn is_revealed_and_wall(map: &Map, x: i32, y: i32) -> bool {
    let pt = Point::new(x, y);
    let idx = map.point2d_to_index(pt);
    map.tiles[idx].tile_type == TileType::Wall && map.revealed.get_bit(pt)
}

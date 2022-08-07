use super::*;

pub enum XEnd {
    LEFT,
    CENTER,
    RIGHT,
}

pub enum YEnd {
    TOP,
    CENTER,
    BOTTOM,
}

pub struct AreaEndingPosition {
    x: XEnd,
    y: YEnd,
}

impl MetaMapBuilder for AreaEndingPosition {
    fn build_map(&mut self, build_data: &mut BuilderMap) { self.build(build_data); }
}

impl AreaEndingPosition {
    pub fn new(x: XEnd, y: YEnd) -> Box<AreaEndingPosition> { Box::new(AreaEndingPosition { x, y }) }

    fn build(&mut self, build_data: &mut BuilderMap) {
        let seed_x = match self.x {
            XEnd::LEFT => 1,
            XEnd::CENTER => build_data.map.width / 2,
            XEnd::RIGHT => build_data.map.width - 2,
        };

        let seed_y = match self.y {
            YEnd::TOP => 1,
            YEnd::CENTER => build_data.map.height / 2,
            YEnd::BOTTOM => build_data.map.height - 2,
        };

        let mut available_floors: Vec<(usize, f32)> = Vec::new();
        for (idx, tile) in build_data.map.tiles.iter().enumerate() {
            if tile.walkable {
                let pt = build_data.map.index_to_point2d(idx);

                available_floors
                    .push((idx, DistanceAlg::PythagorasSquared.distance2d(pt, Point::new(seed_x, seed_y))));
            }
        }
        if available_floors.is_empty() {
            panic!("No valid floors to start on");
        }

        available_floors.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

        build_data.map.tiles[available_floors[0].0] = GameTile::stairs_down();
        build_data.take_snapshot();
    }
}

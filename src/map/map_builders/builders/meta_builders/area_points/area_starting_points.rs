use super::*;

pub enum XStart {
    LEFT,
    CENTER,
    RIGHT,
}

pub enum YStart {
    TOP,
    CENTER,
    BOTTOM,
}

pub struct AreaStartingPosition {
    x: XStart,
    y: YStart,
}

impl MetaMapBuilder for AreaStartingPosition {
    fn build_map(&mut self, build_data: &mut BuilderMap) {
        self.build(build_data);
    }
}

impl AreaStartingPosition {
    pub fn new(x: XStart, y: YStart) -> Box<AreaStartingPosition> {
        Box::new(AreaStartingPosition { x, y })
    }

    fn build(&mut self, build_data: &mut BuilderMap) {
        let seed_x;
        let seed_y;

        match self.x {
            XStart::LEFT => seed_x = 1,
            XStart::CENTER => seed_x = build_data.map.width / 2,
            XStart::RIGHT => seed_x = build_data.map.width - 2,
        }

        match self.y {
            YStart::TOP => seed_y = 1,
            YStart::CENTER => seed_y = build_data.map.height / 2,
            YStart::BOTTOM => seed_y = build_data.map.height - 2,
        }

        let mut available_floors: Vec<(usize, f32)> = Vec::new();
        for (idx, tile) in build_data.map.tiles.iter().enumerate() {
            if tile.walkable {
                let pt = build_data.map.index_to_point2d(idx);
                available_floors.push((
                    idx,
                    DistanceAlg::PythagorasSquared.distance2d(pt, Point::new(seed_x, seed_y)),
                ));
            }
        }
        if available_floors.is_empty() {
            panic!("No valid floors to start on");
        }

        available_floors.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

        let pt = build_data.map.index_to_point2d(available_floors[0].0);
        build_data.starting_position = Some(pt);
    }
}

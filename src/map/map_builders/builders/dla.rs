use super::*;

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum DLAAlgorithm {
    WalkInwards,
    WalkOutwards,
    CentralAttractor,
}

pub struct DLABuilder {
    brush_size: i32,
    symmetry: Symmetry,
    floor_percent: f32,
    algorithm: DLAAlgorithm,
}

impl InitialMapBuilder for DLABuilder {
    fn build_map(&mut self, build_data: &mut BuilderMap) { self.build(build_data); }
}

impl MetaMapBuilder for DLABuilder {
    fn build_map(&mut self, build_data: &mut BuilderMap) { self.build(build_data); }
}

impl DLABuilder {
    pub fn new() -> Box<DLABuilder> {
        Box::new(DLABuilder {
            brush_size: 2,
            floor_percent: 0.25,
            symmetry: Symmetry::None,
            algorithm: DLAAlgorithm::WalkInwards,
        })
    }

    pub fn walk_inwards() -> Box<DLABuilder> {
        Box::new(DLABuilder {
            brush_size: 1,
            floor_percent: 0.25,
            symmetry: Symmetry::None,
            algorithm: DLAAlgorithm::WalkInwards,
        })
    }

    pub fn walk_outwards() -> Box<DLABuilder> {
        Box::new(DLABuilder {
            brush_size: 2,
            floor_percent: 0.25,
            symmetry: Symmetry::None,
            algorithm: DLAAlgorithm::WalkOutwards,
        })
    }

    pub fn heavy_erosion() -> Box<DLABuilder> {
        Box::new(DLABuilder {
            brush_size: 2,
            floor_percent: 0.35,
            symmetry: Symmetry::None,
            algorithm: DLAAlgorithm::WalkInwards,
        })
    }

    pub fn central_attractor() -> Box<DLABuilder> {
        Box::new(DLABuilder {
            brush_size: 2,
            floor_percent: 0.25,
            symmetry: Symmetry::None,
            algorithm: DLAAlgorithm::CentralAttractor,
        })
    }

    pub fn insectoid() -> Box<DLABuilder> {
        Box::new(DLABuilder {
            brush_size: 2,
            floor_percent: 0.25,
            symmetry: Symmetry::Horizontal,
            algorithm: DLAAlgorithm::CentralAttractor,
        })
    }

    #[allow(clippy::map_entry)]
    fn build(&mut self, build_data: &mut BuilderMap) {
        // Carve a starting seed
        let starting_position = Point::new(build_data.map.width / 2, build_data.map.height / 2);
        let start_idx = build_data.map.xy_idx(starting_position.x, starting_position.y);
        build_data.take_snapshot();

        build_data.map.tiles[start_idx] = GameTile::floor();
        build_data.map.tiles[start_idx - 1] = GameTile::floor();
        build_data.map.tiles[start_idx + 1] = GameTile::floor();
        build_data.map.tiles[start_idx - build_data.map.width as usize] = GameTile::floor();
        build_data.map.tiles[start_idx + build_data.map.width as usize] = GameTile::floor();

        // Random walker
        let total_tiles = build_data.map.width * build_data.map.height;
        let desired_floor_tiles = (self.floor_percent * total_tiles as f32) as usize;
        let mut floor_tile_count =
            build_data.map.tiles.iter().filter(|a| a.tile_type == TileType::Floor).count();

        while floor_tile_count < desired_floor_tiles {
            match self.algorithm {
                // Inwards walker
                DLAAlgorithm::WalkInwards => {
                    let mut digger_x = crate::rng::roll_dice(1, build_data.map.width - 3) + 1;
                    let mut digger_y = crate::rng::roll_dice(1, build_data.map.height - 3) + 1;
                    let mut prev_x = digger_x;
                    let mut prev_y = digger_y;

                    let mut digger_idx = build_data.map.xy_idx(digger_x, digger_y);
                    while build_data.map.tiles[digger_idx].tile_type == TileType::Wall {
                        prev_x = digger_x;
                        prev_y = digger_y;
                        let stagger_direction = crate::rng::roll_dice(1, 4);
                        match stagger_direction {
                            1 => {
                                if digger_x > 2 {
                                    digger_x -= 1;
                                }
                            }
                            2 => {
                                if digger_x < build_data.map.width - 2 {
                                    digger_x += 1;
                                }
                            }
                            3 => {
                                if digger_y > 2 {
                                    digger_y -= 1;
                                }
                            }
                            _ => {
                                if digger_y < build_data.map.height - 2 {
                                    digger_y += 1;
                                }
                            }
                        }
                        digger_idx = build_data.map.xy_idx(digger_x, digger_y);
                    }

                    paint(&mut build_data.map, self.symmetry, self.brush_size, prev_x, prev_y);
                }

                // Outwards walker
                DLAAlgorithm::WalkOutwards => {
                    let mut digger_x = starting_position.x;
                    let mut digger_y = starting_position.y;
                    let mut digger_idx = build_data.map.xy_idx(digger_x, digger_y);

                    while build_data.map.tiles[digger_idx].tile_type == TileType::Floor {
                        let stagger_direction = crate::rng::roll_dice(1, 4);
                        match stagger_direction {
                            1 => {
                                if digger_x > 2 {
                                    digger_x -= 1;
                                }
                            }
                            2 => {
                                if digger_x < build_data.map.width - 2 {
                                    digger_x += 1;
                                }
                            }
                            3 => {
                                if digger_y > 2 {
                                    digger_y -= 1;
                                }
                            }
                            _ => {
                                if digger_y < build_data.map.height - 2 {
                                    digger_y += 1;
                                }
                            }
                        }
                        digger_idx = build_data.map.xy_idx(digger_x, digger_y);
                    }
                    paint(&mut build_data.map, self.symmetry, self.brush_size, digger_x, digger_y);
                }

                // Central walker
                DLAAlgorithm::CentralAttractor => {
                    let mut digger_x = crate::rng::roll_dice(1, build_data.map.width - 3) + 1;
                    let mut digger_y = crate::rng::roll_dice(1, build_data.map.height - 3) + 1;
                    let mut prev_x = digger_x;
                    let mut prev_y = digger_y;
                    let mut digger_idx = build_data.map.xy_idx(digger_x, digger_y);

                    let mut path = line2d(
                        LineAlg::Bresenham,
                        Point::new(digger_x, digger_y),
                        Point::new(starting_position.x, starting_position.y),
                    );

                    while build_data.map.tiles[digger_idx].tile_type == TileType::Wall && !path.is_empty() {
                        prev_x = digger_x;
                        prev_y = digger_y;
                        digger_x = path[0].x;
                        digger_y = path[0].y;
                        path.remove(0);
                        digger_idx = build_data.map.xy_idx(digger_x, digger_y);
                    }
                    paint(&mut build_data.map, self.symmetry, self.brush_size, prev_x, prev_y);
                }
            }

            build_data.take_snapshot();

            floor_tile_count = build_data.map.tiles.iter().filter(|a| a.tile_type == TileType::Floor).count();
        }
    }
}

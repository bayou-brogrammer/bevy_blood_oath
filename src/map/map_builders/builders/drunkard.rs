use super::*;

#[derive(Eq, PartialEq, Copy, Clone)]

pub enum DrunkSpawnMode {
    StartingPoint,
    Random,
}

pub struct DrunkardSettings {
    pub brush_size: i32,
    pub symmetry: Symmetry,
    pub floor_percent: f32,
    pub drunken_lifetime: i32,
    pub spawn_mode: DrunkSpawnMode,
}

pub struct DrunkardsWalkBuilder {
    settings: DrunkardSettings,
}

impl InitialMapBuilder for DrunkardsWalkBuilder {
    fn build_map(&mut self, build_data: &mut BuilderMap) { self.build(build_data); }
}

impl MetaMapBuilder for DrunkardsWalkBuilder {
    fn build_map(&mut self, build_data: &mut BuilderMap) { self.build(build_data); }
}

impl DrunkardsWalkBuilder {
    pub fn new(settings: DrunkardSettings) -> DrunkardsWalkBuilder { DrunkardsWalkBuilder { settings } }

    pub fn open_area() -> Box<DrunkardsWalkBuilder> {
        Box::new(DrunkardsWalkBuilder {
            settings: DrunkardSettings {
                brush_size: 1,
                floor_percent: 0.5,
                drunken_lifetime: 400,
                symmetry: Symmetry::None,
                spawn_mode: DrunkSpawnMode::StartingPoint,
            },
        })
    }

    pub fn open_halls() -> Box<DrunkardsWalkBuilder> {
        Box::new(DrunkardsWalkBuilder {
            settings: DrunkardSettings {
                brush_size: 1,
                floor_percent: 0.5,
                drunken_lifetime: 400,
                symmetry: Symmetry::None,
                spawn_mode: DrunkSpawnMode::Random,
            },
        })
    }

    pub fn winding_passages() -> Box<DrunkardsWalkBuilder> {
        Box::new(DrunkardsWalkBuilder {
            settings: DrunkardSettings {
                brush_size: 1,
                floor_percent: 0.4,
                drunken_lifetime: 100,
                symmetry: Symmetry::None,
                spawn_mode: DrunkSpawnMode::Random,
            },
        })
    }

    pub fn fat_passages() -> Box<DrunkardsWalkBuilder> {
        Box::new(DrunkardsWalkBuilder {
            settings: DrunkardSettings {
                brush_size: 2,
                floor_percent: 0.4,
                drunken_lifetime: 100,
                symmetry: Symmetry::None,
                spawn_mode: DrunkSpawnMode::Random,
            },
        })
    }

    pub fn fearful_symmetry() -> Box<DrunkardsWalkBuilder> {
        Box::new(DrunkardsWalkBuilder {
            settings: DrunkardSettings {
                brush_size: 1,
                floor_percent: 0.4,
                drunken_lifetime: 100,
                symmetry: Symmetry::Both,
                spawn_mode: DrunkSpawnMode::Random,
            },
        })
    }

    fn build(&mut self, build_data: &mut BuilderMap) {
        // Set a central starting point
        let starting_position = Point { x: build_data.map.width / 2, y: build_data.map.height / 2 };
        let start_idx = build_data.map.xy_idx(starting_position.x, starting_position.y);
        build_data.map.tiles[start_idx] = GameTile::floor();

        let total_tiles = build_data.map.width * build_data.map.height;
        let desired_floor_tiles = (self.settings.floor_percent * total_tiles as f32) as usize;
        let mut floor_tile_count =
            build_data.map.tiles.iter().filter(|a| a.tile_type == TileType::Floor).count();

        let mut digger_count = 0;
        while floor_tile_count < desired_floor_tiles {
            let mut drunk_x;
            let mut drunk_y;
            let mut did_something = false;

            match self.settings.spawn_mode {
                DrunkSpawnMode::StartingPoint => {
                    drunk_x = starting_position.x;
                    drunk_y = starting_position.y;
                }
                DrunkSpawnMode::Random => {
                    if digger_count == 0 {
                        drunk_x = starting_position.x;
                        drunk_y = starting_position.y;
                    } else {
                        drunk_x = crate::rng::roll_dice(1, build_data.map.width - 3) + 1;
                        drunk_y = crate::rng::roll_dice(1, build_data.map.height - 3) + 1;
                    }
                }
            }
            let mut drunk_life = self.settings.drunken_lifetime;

            while drunk_life > 0 {
                let drunk_idx = build_data.map.xy_idx(drunk_x, drunk_y);
                if build_data.map.tiles[drunk_idx].tile_type == TileType::Wall {
                    did_something = true;
                }

                paint(
                    &mut build_data.map,
                    self.settings.symmetry,
                    self.settings.brush_size,
                    drunk_x,
                    drunk_y,
                );
                build_data.map.tiles[drunk_idx] = GameTile::stairs_down();

                let stagger_direction = crate::rng::roll_dice(1, 4);
                match stagger_direction {
                    1 => {
                        if drunk_x > 2 {
                            drunk_x -= 1;
                        }
                    }
                    2 => {
                        if drunk_x < build_data.map.width - 2 {
                            drunk_x += 1;
                        }
                    }
                    3 => {
                        if drunk_y > 2 {
                            drunk_y -= 1;
                        }
                    }
                    _ => {
                        if drunk_y < build_data.map.height - 2 {
                            drunk_y += 1;
                        }
                    }
                }

                drunk_life -= 1;
            }
            if did_something {
                build_data.take_snapshot();
            }

            digger_count += 1;
            for t in build_data.map.tiles.iter_mut() {
                if t.tile_type == TileType::DownStairs {
                    *t = GameTile::floor();
                }
            }
            floor_tile_count = build_data.map.tiles.iter().filter(|a| a.tile_type == TileType::Floor).count();
        }
    }
}

use super::*;
use std::collections::HashSet;

mod town_buildings;
mod town_people;

pub use town_buildings::*;
pub use town_people::*;

pub fn town_builder(new_depth: i32, width: i32, height: i32) -> BuilderChain {
    let mut chain = BuilderChain::new(new_depth, width, height, "The Town of Bracketon");
    chain.start_with(TownBuilder::new());
    chain
}

#[derive(Debug)]
enum BuildingTag {
    Pub,
    Temple,
    Blacksmith,
    Clothier,
    Alchemist,
    PlayerHouse,
    Hovel,
    Abandoned,
    Unassigned,
}

pub struct TownBuilder {}

impl InitialMapBuilder for TownBuilder {
    fn build_map(&mut self, build_data: &mut BuilderMap) { self.build_rooms(build_data); }
}

impl TownBuilder {
    pub fn new() -> Box<TownBuilder> { Box::new(TownBuilder {}) }

    pub fn build_rooms(&mut self, build_data: &mut BuilderMap) {
        self.grass_layer(build_data);
        self.water_and_piers(build_data);

        // Create the town!
        let (mut available_building_tiles, wall_gap_y) = self.town_walls(build_data);
        let mut buildings = self.buildings(build_data, &mut available_building_tiles);
        let doors = self.add_doors(build_data, &mut buildings, wall_gap_y);

        // Connect gravel to doors!
        self.add_paths(build_data, &doors);

        // Exit
        for y in wall_gap_y - 3..wall_gap_y + 4 {
            let exit_idx = build_data.map.xy_idx(build_data.width - 2, y);
            build_data.map.tiles[exit_idx] = GameTile::stairs_down();
        }

        let mut building_size: Vec<(usize, i32)> = Vec::new();
        for (i, building) in buildings.iter().enumerate() {
            building_size.push((i, building.2 * building.3));
        }
        building_size.sort_by(|a, b| b.1.cmp(&a.1));

        // Get Building sizes and grab largest
        let building_size = self.sort_buildings(&buildings);
        self.building_factory(build_data, &buildings, &building_size);

        // self.spawn_dockers(build_data);
        // self.spawn_townsfolk(build_data, &mut available_building_tiles);

        // Make visible for screenshot
        build_data.map.visible.apply_all_bits(true);
        build_data.take_snapshot();
    }

    fn grass_layer(&mut self, build_data: &mut BuilderMap) {
        // We'll start with a nice layer of grass
        build_data.map.tiles.iter_mut().for_each(|t| *t = GameTile::grass());
        build_data.take_snapshot();
    }

    fn water_and_piers(&mut self, build_data: &mut BuilderMap) {
        let mut n = (crate::rng::roll_dice(1, 65535) as f32) / 65535f32;
        let mut water_width: Vec<i32> = Vec::new();

        for y in 0..build_data.height {
            let n_water = (f32::sin(n) * 10.0) as i32 + 14 + crate::rng::roll_dice(1, 6);
            water_width.push(n_water);
            n += 0.1;
            for x in 0..n_water {
                let idx = build_data.map.xy_idx(x, y);
                build_data.map.tiles[idx] = GameTile::deep_water();
            }
            for x in n_water..n_water + 3 {
                let idx = build_data.map.xy_idx(x, y);
                build_data.map.tiles[idx] = GameTile::shallow_water();
            }
        }
        build_data.take_snapshot();

        // Add piers
        for _i in 0..crate::rng::roll_dice(1, 4) + 6 {
            let y = crate::rng::roll_dice(1, build_data.height) - 1;
            for x in 2 + crate::rng::roll_dice(1, 6)..water_width[y as usize] + 4 {
                let idx = build_data.map.xy_idx(x, y);
                build_data.map.tiles[idx] = GameTile::wood_floor();
            }
        }
        build_data.take_snapshot();
    }

    fn town_walls(&mut self, build_data: &mut BuilderMap) -> (HashSet<usize>, i32) {
        let mut available_building_tiles: HashSet<usize> = HashSet::new();
        let wall_gap_y = crate::rng::roll_dice(1, build_data.height - 9) + 5;

        for y in 1..build_data.height - 2 {
            if !(y > wall_gap_y - 4 && y < wall_gap_y + 4) {
                let idx = build_data.map.xy_idx(30, y);
                build_data.map.tiles[idx] = GameTile::wall();
                build_data.map.tiles[idx - 1] = GameTile::floor();

                let idx_right = build_data.map.xy_idx(build_data.width - 2, y);
                build_data.map.tiles[idx_right] = GameTile::wall();

                for x in 31..build_data.width - 2 {
                    let gravel_idx = build_data.map.xy_idx(x, y);
                    build_data.map.tiles[gravel_idx] = GameTile::gravel();
                    if y > 2 && y < build_data.height - 1 {
                        available_building_tiles.insert(gravel_idx);
                    }
                }
            } else {
                for x in 30..build_data.width {
                    let road_idx = build_data.map.xy_idx(x, y);
                    build_data.map.tiles[road_idx] = GameTile::road();
                }
            }
        }
        build_data.take_snapshot();

        for x in 30..build_data.width - 1 {
            let idx_top = build_data.map.xy_idx(x, 1);
            build_data.map.tiles[idx_top] = GameTile::wall();
            let idx_bot = build_data.map.xy_idx(x, build_data.height - 2);
            build_data.map.tiles[idx_bot] = GameTile::wall();
        }
        build_data.take_snapshot();

        (available_building_tiles, wall_gap_y)
    }

    fn buildings(
        &mut self,
        build_data: &mut BuilderMap,
        available_building_tiles: &mut HashSet<usize>,
    ) -> Vec<(i32, i32, i32, i32)> {
        let mut buildings: Vec<(i32, i32, i32, i32)> = Vec::new();
        let mut n_buildings = 0;
        while n_buildings < 12 {
            let bx = crate::rng::roll_dice(1, build_data.map.width - 32) + 30;
            let by = crate::rng::roll_dice(1, build_data.map.height) - 2;
            let bw = crate::rng::roll_dice(1, 8) + 4;
            let bh = crate::rng::roll_dice(1, 8) + 4;
            let mut possible = true;
            for y in by..by + bh {
                for x in bx..bx + bw {
                    if x < 0 || x > build_data.width - 1 || y < 0 || y > build_data.height - 1 {
                        possible = false;
                    } else {
                        let idx = build_data.map.xy_idx(x, y);
                        if !available_building_tiles.contains(&idx) {
                            possible = false;
                        }
                    }
                }
            }
            if possible {
                n_buildings += 1;
                buildings.push((bx, by, bw, bh));
                for y in by..by + bh {
                    for x in bx..bx + bw {
                        let idx = build_data.map.xy_idx(x, y);
                        build_data.map.tiles[idx] = GameTile::wood_floor();
                        available_building_tiles.remove(&idx);
                        available_building_tiles.remove(&(idx + 1));
                        available_building_tiles.remove(&(idx + build_data.width as usize));
                        available_building_tiles.remove(&(idx - 1));
                        available_building_tiles.remove(&(idx - build_data.width as usize));
                    }
                }
                build_data.take_snapshot();
            }
        }

        // Outline buildings
        let mut mapclone = build_data.map.clone();
        for y in 2..build_data.height - 2 {
            for x in 32..build_data.width - 2 {
                let idx = build_data.map.xy_idx(x, y);
                if build_data.map.tiles[idx].tile_type == TileType::WoodFloor {
                    let mut neighbors = 0;
                    if build_data.map.tiles[idx - 1].tile_type != TileType::WoodFloor {
                        neighbors += 1;
                    }
                    if build_data.map.tiles[idx + 1].tile_type != TileType::WoodFloor {
                        neighbors += 1;
                    }
                    if build_data.map.tiles[idx - build_data.width as usize].tile_type != TileType::WoodFloor
                    {
                        neighbors += 1;
                    }
                    if build_data.map.tiles[idx + build_data.width as usize].tile_type != TileType::WoodFloor
                    {
                        neighbors += 1;
                    }
                    if neighbors > 0 {
                        mapclone.tiles[idx] = GameTile::wall();
                    }
                }
            }
        }
        build_data.map = mapclone;
        build_data.take_snapshot();
        buildings
    }

    fn add_doors(
        &mut self,
        build_data: &mut BuilderMap,
        buildings: &mut [(i32, i32, i32, i32)],
        wall_gap_y: i32,
    ) -> Vec<usize> {
        let mut doors = Vec::new();
        for building in buildings.iter() {
            let door_x = building.0 + 1 + crate::rng::roll_dice(1, building.2 - 3);
            let cy = building.1 + (building.3 / 2);
            let idx = if cy > wall_gap_y {
                // Door on the north wall
                build_data.map.xy_idx(door_x, building.1)
            } else {
                build_data.map.xy_idx(door_x, building.1 + building.3 - 1)
            };
            build_data.map.tiles[idx] = GameTile::floor();
            build_data.spawn_list.push((idx, DOOR.to_string()));
            doors.push(idx);
        }
        build_data.take_snapshot();
        doors
    }

    fn add_paths(&mut self, build_data: &mut BuilderMap, doors: &[usize]) {
        let mut roads = Vec::new();
        for y in 0..build_data.height {
            for x in 0..build_data.width {
                let idx = build_data.map.xy_idx(x, y);
                if build_data.map.tiles[idx].tile_type == TileType::Road {
                    roads.push(idx);
                }
            }
        }

        crate::spatial::populate_blocked_from_map(&build_data.map);
        crate::spatial::populate_opaque_from_map(&build_data.map);
        for door_idx in doors.iter() {
            let mut nearest_roads: Vec<(usize, f32)> = Vec::new();
            let door_pt = Point::new(
                *door_idx as i32 % build_data.map.width as i32,
                *door_idx as i32 / build_data.map.width as i32,
            );
            for r in roads.iter() {
                nearest_roads.push((
                    *r,
                    DistanceAlg::PythagorasSquared.distance2d(
                        door_pt,
                        Point::new(*r as i32 % build_data.map.width, *r as i32 / build_data.map.width),
                    ),
                ));
            }
            nearest_roads.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

            let destination = nearest_roads[0].0;
            let path = a_star_search(*door_idx, destination, &build_data.map);
            if path.success {
                for step in path.steps.iter() {
                    let idx = *step as usize;
                    build_data.map.tiles[idx] = GameTile::road();
                    roads.push(idx);
                }
            }
            build_data.take_snapshot();
        }
    }

    fn sort_buildings(&mut self, buildings: &[(i32, i32, i32, i32)]) -> Vec<(usize, i32, BuildingTag)> {
        let mut building_size: Vec<(usize, i32, BuildingTag)> = Vec::new();

        for (i, building) in buildings.iter().enumerate() {
            building_size.push((i, building.2 * building.3, BuildingTag::Unassigned));
        }

        // building_size.sort_by(|a, b| a.1.cmp(&b.1));
        building_size.sort_by(|a, b| b.1.cmp(&a.1));

        building_size[0].2 = BuildingTag::Pub;
        building_size[1].2 = BuildingTag::Temple;
        building_size[2].2 = BuildingTag::Blacksmith;
        building_size[3].2 = BuildingTag::Clothier;
        building_size[4].2 = BuildingTag::Alchemist;
        building_size[5].2 = BuildingTag::PlayerHouse;

        for b in building_size.iter_mut().skip(6) {
            b.2 = BuildingTag::Hovel;
        }

        let last_index = building_size.len() - 1;
        building_size[last_index].2 = BuildingTag::Abandoned;

        building_size
    }

    fn building_factory(
        &mut self,

        build_data: &mut BuilderMap,
        buildings: &[(i32, i32, i32, i32)],
        building_index: &[(usize, i32, BuildingTag)],
    ) {
        for (i, building) in buildings.iter().enumerate() {
            let build_type = &building_index[i].2;

            match build_type {
                BuildingTag::Pub => self.build_pub(building, build_data),
                BuildingTag::Hovel => self.build_hovel(building, build_data),
                BuildingTag::Temple => self.build_temple(building, build_data),
                BuildingTag::Blacksmith => self.build_smith(building, build_data),
                BuildingTag::Clothier => self.build_clothier(building, build_data),
                BuildingTag::Alchemist => self.build_alchemist(building, build_data),
                BuildingTag::PlayerHouse => self.build_my_house(building, build_data),
                BuildingTag::Abandoned => self.build_abandoned_house(building, build_data),
                _ => {}
            }
        }
    }
}

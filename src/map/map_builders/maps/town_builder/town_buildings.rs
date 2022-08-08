use super::*;

impl TownBuilder {
    pub fn random_building_spawn(
        &mut self,
        building: &(i32, i32, i32, i32),
        build_data: &mut BuilderMap,

        to_place: &mut Vec<&str>,
        player_idx: usize,
    ) {
        for y in building.1..building.1 + building.3 {
            for x in building.0..building.0 + building.2 {
                let idx = build_data.map.xy_idx(x, y);

                if build_data.map.tiles[idx].tile_type == TileType::WoodFloor
                    && idx != player_idx
                    && crate::rng::roll_dice(1, 3) == 1
                    && !to_place.is_empty()
                {
                    let entity_tag = to_place[0];
                    to_place.remove(0);
                    build_data.spawn_list.push((idx, entity_tag.to_string()));
                }
            }
        }
    }

    pub fn build_pub(&mut self, building: &(i32, i32, i32, i32), build_data: &mut BuilderMap) {
        // Place the player
        build_data.starting_position =
            Some(Point { x: building.0 + (building.2 / 2), y: building.1 + (building.3 / 2) });
        let player_idx = build_data.map.xy_idx(building.0 + (building.2 / 2), building.1 + (building.3 / 2));

        // Place other items
        let mut to_place: Vec<&str> =
            vec![BARKEEP, SHADY_SALESMAN, PATRON, PATRON, KEG, TABLE, CHAIR, TABLE, CHAIR];
        self.random_building_spawn(building, build_data, &mut to_place, player_idx);
    }

    pub fn build_temple(&mut self, building: &(i32, i32, i32, i32), build_data: &mut BuilderMap) {
        // Place items
        let mut to_place: Vec<&str> =
            vec![PRIEST, PARISHIONER, PARISHIONER, CHAIR, CHAIR, CANDLE, CANDLE, ALTAR];

        self.random_building_spawn(building, build_data, &mut to_place, 0);
    }

    pub fn build_smith(&mut self, building: &(i32, i32, i32, i32), build_data: &mut BuilderMap) {
        // Place items
        let mut to_place: Vec<&str> = vec![BLACKSMITH, ANVIL, WATER_TROUGH, WEAPON_RACK, ARMOR_STAND];
        self.random_building_spawn(building, build_data, &mut to_place, 0);
    }

    pub fn build_clothier(&mut self, building: &(i32, i32, i32, i32), build_data: &mut BuilderMap) {
        // Place items
        let mut to_place: Vec<&str> = vec![CLOTHIER, CABINET, TABLE, LOOM, HIDE_RACK];
        self.random_building_spawn(building, build_data, &mut to_place, 0);
    }

    pub fn build_alchemist(&mut self, building: &(i32, i32, i32, i32), build_data: &mut BuilderMap) {
        // Place items
        let mut to_place: Vec<&str> = vec![ALCHEMIST, CHEMISTRY_SET, DEAD_THING, CHAIR, TABLE];
        self.random_building_spawn(building, build_data, &mut to_place, 0);
    }

    pub fn build_my_house(&mut self, building: &(i32, i32, i32, i32), build_data: &mut BuilderMap) {
        // Place items
        let mut to_place: Vec<&str> = vec![MOM, BED, CABINET, CHAIR, TABLE];
        self.random_building_spawn(building, build_data, &mut to_place, 0);
    }

    pub fn build_hovel(&mut self, building: &(i32, i32, i32, i32), build_data: &mut BuilderMap) {
        // Place items
        let mut to_place: Vec<&str> = vec![PEASANT, BED, CHAIR, TABLE];
        self.random_building_spawn(building, build_data, &mut to_place, 0);
    }

    pub fn build_abandoned_house(&mut self, building: &(i32, i32, i32, i32), build_data: &mut BuilderMap) {
        for y in building.1..building.1 + building.3 {
            for x in building.0..building.0 + building.2 {
                let idx = build_data.map.xy_idx(x, y);
                if build_data.map.tiles[idx].tile_type == TileType::WoodFloor
                    && idx != 0
                    && crate::rng::roll_dice(1, 2) == 1
                {
                    build_data.spawn_list.push((idx, "Rat".to_string()));
                }
            }
        }
    }
}

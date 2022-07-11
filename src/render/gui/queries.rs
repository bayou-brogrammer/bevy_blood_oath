use crate::prelude::*;

pub struct PlayerStatus {
    // pub current_hp: i32,
    // pub max_hp: i32,
    // pub property_damage: i32,
    // pub human_resources: i32,
    pub colony: ColonyInfo,
    // pub target: TargetInfo,
}

pub struct ColonyInfo {
    pub total_colonists: i32,
    pub colonists_on_layer: i32,
    pub located_alive: i32,
    pub located_dead: i32,
    pub died_in_rescue: i32,
    pub rescued: i32,
}

pub struct TargetInfo {
    pub target: Option<Entity>,
    pub color: Option<RGBA>,
    pub name: Option<String>,
    pub point: Option<Point>,
    pub probability: Option<u32>,
    pub range: Option<u32>,
}

impl PlayerStatus {
    pub fn query(ecs: &World, map_layer: usize) -> Self {
        let colony = PlayerStatus::colony_calculator(ecs, map_layer);
        // let (current_hp, max_hp) = PlayerStatus::health(ecs);
        // let property_damage = PlayerStatus::property_damage(ecs);
        // let human_resources = PlayerStatus::human_resources(&colony, property_damage);
        // let target = PlayerStatus::targeting_info(ecs);

        Self {
            // current_hp,
            // max_hp,
            // property_damage,
            // human_resources,
            colony,
            // target,
        }
    }

    fn colony_calculator(ecs: &World, current_layer: usize) -> ColonyInfo {
        let mut total_colonists = 0;
        let mut colonists_on_layer = 0;
        let mut located_alive = 0;
        let mut located_dead = 0;
        let mut died_in_rescue = 0;
        let mut rescued = 0;

        <(Entity, &Colonist, &Position, &ColonistStatus)>::query().for_each(
            ecs,
            |(entity, _, pos, status)| {
                if *status != ColonistStatus::StartedDead {
                    total_colonists += 1;
                }
                if pos.layer == current_layer
                    && *status != ColonistStatus::Rescued
                    && *status != ColonistStatus::DiedAfterStart
                    && *status != ColonistStatus::StartedDead
                {
                    colonists_on_layer += 1;
                }
                // if let Ok(entry) = ecs.entry_ref(*entity) {
                //     if let Ok(_) = entry.get_component::<Found>() {
                //         match *status {
                //             ColonistStatus::Alive => located_alive += 1,
                //             ColonistStatus::StartedDead => located_dead += 1,
                //             ColonistStatus::DiedAfterStart => died_in_rescue += 1,
                //             ColonistStatus::Rescued => rescued += 1,
                //         }
                //     }
                // }
            },
        );

        ColonyInfo {
            total_colonists,
            colonists_on_layer,
            located_alive,
            located_dead,
            died_in_rescue,
            rescued,
        }
    }
}

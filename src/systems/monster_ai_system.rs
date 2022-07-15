use super::*;

pub struct MonsterAISystem {}

impl<'a> System<'a> for MonsterAISystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Entities<'a>,
        WriteExpect<'a, Map>,
        ReadExpect<'a, Point>,
        ReadExpect<'a, Entity>,
        ReadExpect<'a, TurnState>,
        WriteStorage<'a, FieldOfView>,
        ReadStorage<'a, Monster>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, WantsToMelee>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            mut map,
            player_pos,
            player_entity,
            runstate,
            mut fov_storage,
            monster,
            mut position,
            mut wants_to_melee,
        ) = data;

        if *runstate != TurnState::MonsterTurn {
            return;
        }

        for (entity, mut fov, _monster, mut pos) in
            (&entities, &mut fov_storage, &monster, &mut position).join()
        {
            let distance = DistanceAlg::Pythagoras.distance2d(pos.0, *player_pos);

            if distance < 1.5 {
                wants_to_melee
                    .insert(
                        entity,
                        WantsToMelee {
                            target: *player_entity,
                        },
                    )
                    .expect("Unable to insert attack");
            } else if fov.visible_tiles.contains(&*player_pos) {
                let old_idx = map.point2d_to_index(pos.0);
                let new_idx = map.point2d_to_index(*player_pos);

                // Path to the player
                let path = a_star_search(old_idx, new_idx, &mut *map);

                if path.success && path.steps.len() > 1 {
                    let destination = map.index_to_point2d(path.steps[1]);
                    map.update_blocked(pos.0, destination);

                    pos.0 = destination;
                    fov.is_dirty = true;
                }
            }
        }
    }
}

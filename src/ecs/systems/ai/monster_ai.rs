use crate::prelude::*;

pub fn monster_ai(
    map: Res<Map>,
    state: Res<TurnState>,
    mut commands: Commands,
    mut attack_events: EventWriter<WantsToAttack>,
    mut move_events: EventWriter<WantsToMove>,
    player_pos_q: Query<(Entity, &Point), (With<Player>, Without<Monster>)>,
    mut monster_q: Query<
        (Entity, &Point, &FieldOfView, Option<&mut Confusion>),
        (With<Monster>, Without<Player>),
    >,
) {
    if *state != TurnState::AITurn {
        return;
    }

    let (player_ent, player_pos) = player_pos_q.single();

    for (entity, pos, fov, confused) in monster_q.iter_mut() {
        let mut can_act = true;

        if let Some(mut i_am_confused) = confused {
            can_act = false;
            i_am_confused.turns -= 1;

            if i_am_confused.turns < 1 {
                commands.entity(entity).remove::<Confusion>();
            }
        }

        if can_act {
            let distance = DistanceAlg::Pythagoras.distance2d(*pos, *player_pos);
            if distance < 1.5 {
                attack_events.send(WantsToAttack(entity, player_ent));
            } else if fov.visible_tiles.contains(player_pos) {
                // Path to the player
                let path =
                    a_star_search(map.point2d_to_index(*pos), map.point2d_to_index(*player_pos), &*map);

                if path.success && path.steps.len() > 1 && path.steps.len() < 15 {
                    let destination = map.index_to_point2d(path.steps[1]);
                    move_events.send(WantsToMove(entity, destination));
                }
            }
        }
    }
}

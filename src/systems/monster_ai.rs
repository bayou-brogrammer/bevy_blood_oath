use super::*;

pub fn monster_ai(
    state: Res<TurnState>,
    mut map: ResMut<Map>,
    mut attack_events: EventWriter<WantsToAttack>,
    mut move_events: EventWriter<WantsToMove>,
    player_pos_q: Query<(Entity, &Position), (With<Player>, Without<Monster>)>,
    mut monster_q: Query<(Entity, &Position, &FieldOfView), (With<Monster>, Without<Player>)>,
) {
    if *state != TurnState::AITurn {
        return;
    }

    let (player_ent, player_pos) = player_pos_q.single();

    for (entity, pos, fov) in monster_q.iter_mut() {
        let distance = DistanceAlg::Pythagoras.distance2d(pos.0, player_pos.0);
        if distance < 1.5 {
            attack_events.send(WantsToAttack { attacker: entity, victim: player_ent });
        } else if fov.visible_tiles.contains(&player_pos.0) {
            // Path to the player
            let path =
                a_star_search(map.point2d_to_index(pos.0), map.point2d_to_index(player_pos.0), &mut *map);

            if path.success && path.steps.len() > 1 && path.steps.len() < 15 {
                let destination = map.index_to_point2d(path.steps[1]);
                move_events.send(WantsToMove { entity, destination });
            }
        }
    }
}

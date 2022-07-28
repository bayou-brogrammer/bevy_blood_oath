use super::*;

pub fn monster_ai(
    mut map: ResMut<Map>,
    state: Res<StateStack<TurnState>>,
    mut commands: Commands,
    mut attack_events: EventWriter<WantsToAttack>,
    mut move_events: EventWriter<WantsToMove>,
    player_pos_q: Query<(Entity, &Position), (With<Player>, Without<Monster>)>,
    mut monster_q: Query<
        (Entity, &Position, &FieldOfView, Option<&mut Confusion>),
        (With<Monster>, Without<Player>),
    >,
) {
    if *state.current() != TurnState::AITurn {
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
            let distance = DistanceAlg::Pythagoras.distance2d(pos.0, player_pos.0);
            if distance < 1.5 {
                attack_events.send(WantsToAttack { attacker: entity, victim: player_ent });
            } else if fov.visible_tiles.contains(&player_pos.0) {
                // Path to the player
                let path = a_star_search(
                    map.point2d_to_index(pos.0),
                    map.point2d_to_index(player_pos.0),
                    &mut *map,
                );

                if path.success && path.steps.len() > 1 && path.steps.len() < 15 {
                    let destination = map.index_to_point2d(path.steps[1]);
                    move_events.send(WantsToMove { entity, destination });
                }
            }
        }
    }
}

use super::*;

pub fn player_input(
    mut commands: Commands,
    key: Res<Option<VirtualKeyCode>>,
    mut move_events: EventWriter<WantsToMove>,
    mut attack_events: EventWriter<WantsToAttack>,
    enemies_query: Query<(Entity, &Position), (With<Monster>, Without<Player>)>,
    mut player_query: Query<(Entity, &Position), (With<Player>, Without<Monster>)>,
) {
    if let Some(key) = key.as_ref() {
        let delta = match key {
            VirtualKeyCode::Left => Point::new(-1, 0),
            VirtualKeyCode::Right => Point::new(1, 0),
            VirtualKeyCode::Up => Point::new(0, -1),
            VirtualKeyCode::Down => Point::new(0, 1),
            _ => Point::new(0, 0),
        };

        let (entity, pos) = player_query.single_mut();
        let destination = pos.0 + delta;

        if delta.x != 0 || delta.y != 0 {
            let mut hit_something = false;

            // The Iterator#any API could also be conveniently used, although it's often assumed not
            // to have side effects, which is not the case here.
            for (entity, pos) in enemies_query.iter() {
                if pos.0 == destination {
                    hit_something = true;

                    attack_events.send(WantsToAttack {
                        attacker: entity,
                        victim: entity,
                    });
                }
            }

            if !hit_something {
                move_events.send(WantsToMove {
                    destination,
                    entity,
                });
            }
        }

        commands.insert_resource(TurnState::PlayerTurn);
        commands.remove_resource::<VirtualKeyCode>();
    }
}

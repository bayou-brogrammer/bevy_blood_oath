use crate::prelude::*;

pub fn bystander_ai(
    state: Res<TurnState>,
    mut move_events: EventWriter<WantsToMove>,
    bystander_q: Query<(Entity, &Point), With<Bystander>>,
) {
    if *state != TurnState::AITurn {
        return;
    }

    for (entity, pos) in bystander_q.iter() {
        // Try to move randomly
        let destination = match crate::rng::range(0, 4) {
            0 => Point::new(-1, 0),
            1 => Point::new(1, 0),
            2 => Point::new(0, -1),
            _ => Point::new(0, 1),
        } + *pos;

        move_events.send(WantsToMove(entity, destination));
    }
}

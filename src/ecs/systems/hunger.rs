use crate::prelude::*;

pub fn hunger_clock(
    state: Res<TurnState>,
    player_entity: Res<Entity>,
    mut hunger_q: Query<(Entity, &mut HungerClock)>,
) {
    for (entity, mut clock) in hunger_q.iter_mut() {
        match *state {
            TurnState::PlayerTurn => {
                if entity != *player_entity {
                    return;
                }
            }
            TurnState::AITurn => {
                if entity == *player_entity {
                    return;
                }
            }
            _ => {}
        }

        clock.duration -= 1;

        if clock.duration < 1 {
            match clock.state {
                HungerState::WellFed => {
                    clock.state = HungerState::Normal;
                    clock.duration = 200;
                    if entity == *player_entity {
                        bo_logging::Logger::new()
                            .color(ORANGE)
                            .append("You are no longer well fed")
                            .log();
                    }
                }
                HungerState::Normal => {
                    clock.state = HungerState::Hungry;
                    clock.duration = 200;
                    if entity == *player_entity {
                        bo_logging::Logger::new().color(ORANGE).append("You are hungry").log();
                    }
                }
                HungerState::Hungry => {
                    clock.state = HungerState::Starving;
                    clock.duration = 200;
                    if entity == *player_entity {
                        bo_logging::Logger::new().color(RED).append("You are starving!").log();
                    }
                }
                HungerState::Starving => {
                    // Inflict damage from hunger
                    if entity == *player_entity {
                        bo_logging::Logger::new()
                            .color(RED)
                            .append("Your hunger pangs are getting painful! You suffer 1 hp damage.")
                            .log();
                    }

                    add_effect(None, EffectType::Damage(1), Targets::Single(entity));
                }
            }
        }
    }
}

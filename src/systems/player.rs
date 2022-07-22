use super::*;
use bevy::app::AppExit;
use bracket_lib::prelude::VirtualKeyCode;

#[derive(Debug)]
pub enum PlayerInputResult {
    AppQuit,
    NoResult,
    TurnDone,
    // TryDescend,
    ShowInventory,
    ShowDropMenu,
    // ShowPickUpMenu,
    // ShowInventoryShortcut(GameKey),
    // ShowEquipmentShortcut(GameKey),
}

pub fn player_input(
    key: Res<Option<VirtualKeyCode>>,
    mut commands: Commands,
    // Events
    mut move_events: EventWriter<WantsToMove>,
    mut attack_events: EventWriter<WantsToAttack>,
    mut pickup_event: EventWriter<WantsToPickupItem>,
    // Queries
    items_query: Query<(Entity, &Position), With<Item>>,
    enemies_query: Query<(Entity, &Position), (With<Monster>, Without<Player>)>,
    mut player_query: Query<(Entity, &Position), (With<Player>, Without<Monster>)>,
) -> PlayerInputResult {
    if let Some(key) = key.as_ref() {
        let mut delta = Point::new(0, 0);
        let (player, pos) = player_query.single_mut();

        match key {
            VirtualKeyCode::Escape => return PlayerInputResult::AppQuit,
            VirtualKeyCode::Left => delta += Point::new(-1, 0),
            VirtualKeyCode::Right => delta += Point::new(1, 0),
            VirtualKeyCode::Up => delta += Point::new(0, -1),
            VirtualKeyCode::Down => delta += Point::new(0, 1),
            VirtualKeyCode::G => match try_pickup_item(pos.0, items_query) {
                None => {}
                Some(item) => {
                    pickup_event.send(WantsToPickupItem { item, collected_by: player });
                }
            },
            VirtualKeyCode::I => return PlayerInputResult::ShowInventory,
            VirtualKeyCode::D => return PlayerInputResult::ShowDropMenu,
            _ => {}
        }

        let destination = pos.0 + delta;
        if delta.x != 0 || delta.y != 0 {
            let mut hit_something = false;

            // The Iterator#any API could also be conveniently used, although it's often assumed not
            // to have side effects, which is not the case here.
            for (entity, pos) in enemies_query.iter() {
                if pos.0 == destination {
                    hit_something = true;

                    attack_events.send(WantsToAttack { attacker: player, victim: entity });
                }
            }

            if !hit_something {
                move_events.send(WantsToMove { destination, entity: player });
            }
        }

        commands.remove_resource::<VirtualKeyCode>();
        return PlayerInputResult::TurnDone;
    }

    PlayerInputResult::NoResult
}

pub fn player_turn_done(
    In(result): In<PlayerInputResult>,
    mut commands: Commands,
    mut stack: ResMut<StateStack<TurnState>>,
) {
    match result {
        PlayerInputResult::NoResult => {}
        PlayerInputResult::AppQuit => commands.insert_resource(AppExit),
        PlayerInputResult::TurnDone => {
            commands.insert_resource(StateStack::new(TurnState::PlayerTurn))
        }
        PlayerInputResult::ShowInventory => stack.set(TurnState::Inventory),
        PlayerInputResult::ShowDropMenu => stack.set(TurnState::ShowDropMenu),
    }
}

fn try_pickup_item(
    player_pos: Point,
    items_query: Query<(Entity, &Position), With<Item>>,
) -> Option<Entity> {
    for (entity, item_pos) in items_query.iter() {
        if item_pos.0 == player_pos {
            return Some(entity);
        }
    }

    None
}

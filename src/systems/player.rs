use super::*;
use bevy::app::AppExit;

#[derive(Debug)]
pub enum PlayerInputResult {
    AppQuit,
    NoResult,
    TurnDone,
    // TryDescend,
    ShowInventory,
    ShowDropMenu,
    ShowRemoveMenu,
    // ShowPickUpMenu,
    // ShowInventoryShortcut(GameKey),
    // ShowEquipmentShortcut(GameKey),
}

pub fn player_input(
    mut commands: Commands,
    key: Option<Res<VirtualKeyCode>>,
    // Events
    mut move_events: EventWriter<WantsToMove>,
    mut attack_events: EventWriter<WantsToAttack>,
    mut pickup_event: EventWriter<WantsToPickupItem>,
    // Queries
    items_query: Query<(Entity, &Position), With<Item>>,
    enemies_query: Query<(Entity, &Position), (With<Monster>, Without<Player>)>,
    mut player_query: Query<(Entity, &Position, &FieldOfView), (With<Player>, Without<Monster>)>,
) -> PlayerInputResult {
    if let Some(key) = key.as_deref() {
        let mut delta = Point::new(0, 0);
        let (player, pos, fov) = player_query.single_mut();

        match key {
            VirtualKeyCode::Escape => return PlayerInputResult::AppQuit,

            // Cardinals
            VirtualKeyCode::Left => delta += Point::new(-1, 0),
            VirtualKeyCode::Right => delta += Point::new(1, 0),
            VirtualKeyCode::Up => delta += Point::new(0, -1),
            VirtualKeyCode::Down => delta += Point::new(0, 1),

            // Diagonals
            VirtualKeyCode::Y => delta += Point::new(-1, -1),
            VirtualKeyCode::U => delta += Point::new(1, -1),
            VirtualKeyCode::B => delta += Point::new(-1, 1),
            VirtualKeyCode::N => delta += Point::new(1, 1),

            // Inventory
            VirtualKeyCode::G => match try_pickup_item(pos.0, items_query) {
                None => {}
                Some(item) => {
                    pickup_event.send(WantsToPickupItem(player, item));
                }
            },
            VirtualKeyCode::I => return PlayerInputResult::ShowInventory,
            VirtualKeyCode::D => return PlayerInputResult::ShowDropMenu,
            VirtualKeyCode::R => return PlayerInputResult::ShowRemoveMenu,

            // Skip Turn
            VirtualKeyCode::Numpad5 | VirtualKeyCode::Space => {
                let enemies = enemies_query.iter().map(|q| q.1 .0).collect::<Vec<_>>();

                let mut can_heal = true;
                fov.visible_tiles.iter().for_each(|pt| {
                    if enemies.contains(pt) {
                        can_heal = true
                    }
                });

                if can_heal {
                    add_effect(
                        Some(player),
                        EffectType::new_healing(1),
                        Targets::Single { target: player },
                    )
                }

                return PlayerInputResult::TurnDone;
            }
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
                    attack_events.send(WantsToAttack(player, entity));
                }
            }

            if !hit_something {
                move_events.send(WantsToMove(player, destination));
            }
        }

        commands.remove_resource::<VirtualKeyCode>();
        return PlayerInputResult::TurnDone;
    }

    PlayerInputResult::NoResult
}

pub fn player_turn_done(In(result): In<PlayerInputResult>, mut commands: Commands) {
    match result {
        PlayerInputResult::NoResult => {}
        PlayerInputResult::AppQuit => commands.insert_resource(AppExit),
        PlayerInputResult::TurnDone => commands.insert_resource(TurnState::PlayerTurn),
        PlayerInputResult::ShowInventory => commands.insert_resource(TurnState::Inventory),
        PlayerInputResult::ShowDropMenu => commands.insert_resource(TurnState::ShowDropMenu),
        PlayerInputResult::ShowRemoveMenu => commands.insert_resource(TurnState::ShowRemoveMenu),
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

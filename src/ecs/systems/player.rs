use crate::dungeon_mode::PlayerInputResult;

use super::*;

// #[derive(Debug)]
// pub enum PlayerInputResult {
//     AppQuit,
//     Descend,
//     TurnDone,
//     NoResult,
//     // Inventory
//     ShowDrop,
//     ShowRemove,
//     ShowInventory,
//     _ShowInventoryShortcut,
// }

pub fn player_input(
    mut commands: Commands,
    key: Option<Res<VirtualKeyCode>>,
    // Events
    mut move_events: EventWriter<WantsToMove>,
    mut attack_events: EventWriter<WantsToAttack>,
    mut pickup_event: EventWriter<WantsToPickupItem>,
    // Queries
    items_query: Query<(Entity, &Point), With<Item>>,
    enemies_query: Query<(Entity, &Point), (With<Monster>, Without<Player>)>,
    mut player_query: Query<(Entity, &Point, &FieldOfView), (With<Player>, Without<Monster>)>,
) -> PlayerInputResult {
    if let Some(control) = key.as_deref().get_key() {
        let mut delta = Point::new(0, 0);
        let (player, pos, fov) = player_query.single_mut();

        match control {
            GameKey::Escape => return PlayerInputResult::AppQuit,

            // Cardinals
            GameKey::Left => delta += Point::new(-1, 0),
            GameKey::Right => delta += Point::new(1, 0),
            GameKey::Up => delta += Point::new(0, -1),
            GameKey::Down => delta += Point::new(0, 1),
            // Diagonals
            GameKey::LeftUp => delta += Point::new(-1, -1),
            GameKey::RightUp => delta += Point::new(1, -1),
            GameKey::LeftDown => delta += Point::new(-1, 1),
            GameKey::RightDown => delta += Point::new(1, 1),

            // Inventory
            GameKey::Pickup => match try_pickup_item(*pos, items_query) {
                None => bo_logging::Logger::new().append("There is nothing here to pick up.").log(),
                Some(item) => {
                    pickup_event.send(WantsToPickupItem(player, item));
                }
            },
            GameKey::Inventory => return PlayerInputResult::ShowInventory,
            GameKey::Remove => return PlayerInputResult::ShowRemove,
            GameKey::Drop => return PlayerInputResult::ShowDrop,

            // Skip Turn
            GameKey::SkipTurn => {
                let enemies = enemies_query.iter().map(|q| q.1).collect::<Vec<_>>();

                let mut can_heal = true;
                fov.visible_tiles.iter().for_each(|pt| {
                    if enemies.contains(&pt) {
                        can_heal = true
                    }
                });

                if can_heal {
                    add_effect(Some(player), EffectType::Healing(1), Targets::Single(player))
                }

                return PlayerInputResult::TurnDone;
            }
            _ => return PlayerInputResult::NoResult,
        }

        let destination = *pos + delta;
        if delta.x != 0 || delta.y != 0 {
            let mut hit_something = false;

            // The Iterator#any API could also be conveniently used, although it's often assumed not
            // to have side effects, which is not the case here.
            for (entity, pos) in enemies_query.iter() {
                if *pos == destination {
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

fn try_pickup_item(player_pos: Point, items_query: Query<(Entity, &Point), With<Item>>) -> Option<Entity> {
    for (entity, item_pos) in items_query.iter() {
        if *item_pos == player_pos {
            return Some(entity);
        }
    }

    None
}

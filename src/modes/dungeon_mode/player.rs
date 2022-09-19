use crate::prelude::*;

#[derive(Debug)]
pub enum PlayerInputResult {
    AppQuit,
    Descend,
    TurnDone,
    NoResult,
    // Inventory
    ShowDrop,
    ShowRemove,
    ShowInventory,
    _ShowInventoryShortcut,
}

pub fn player_input(ctx: &mut BTerm, world: &mut World) -> PlayerInputResult {
    match ctx.get_key() {
        None => return PlayerInputResult::NoResult, // Nothing happened
        Some(key) => {
            let player_query = world.query_filtered::<(Entity, &Point), (With<Player>, Without<Monster>)>();

            match key {
                GameKey::Escape => return PlayerInputResult::AppQuit,

                GameKey::Left => try_move_player(world, Point::new(-1, 0), player_query),
                GameKey::Right => try_move_player(world, Point::new(1, 0), player_query),
                GameKey::Up => try_move_player(world, Point::new(0, -1), player_query),
                GameKey::Down => try_move_player(world, Point::new(0, 1), player_query),

                // Diagonals
                GameKey::RightUp => try_move_player(world, Point::new(1, -1), player_query),
                GameKey::LeftUp => try_move_player(world, Point::new(-1, -1), player_query),
                GameKey::RightDown => try_move_player(world, Point::new(1, 1), player_query),
                GameKey::LeftDown => try_move_player(world, Point::new(-1, 1), player_query),

                GameKey::Inventory => return PlayerInputResult::ShowInventory,
                GameKey::Remove => return PlayerInputResult::ShowRemove,
                GameKey::Drop => return PlayerInputResult::ShowDrop,
                GameKey::SkipTurn => return PlayerInputResult::TurnDone,

                GameKey::TakeStairs => {
                    if try_next_level(world) {
                        return PlayerInputResult::Descend;
                    } else {
                        bo_logging::Logger::new().append("There is no way down from here.").log();
                    }
                }

                GameKey::Pickup => try_pickup_item(world, player_query),

                _ => {}
            }
        }
    }

    PlayerInputResult::TurnDone
}

fn try_move_player(
    world: &mut World,
    delta: Point,
    mut player_q: QueryState<(Entity, &Point), (With<Player>, Without<Monster>)>,
) {
    let (player, pos) = player_q.single_mut(world);

    let destination = *pos + delta;
    if delta.x != 0 || delta.y != 0 {
        let mut hit_something = None;

        // The Iterator#any API could also be conveniently used, although it's often assumed not
        // to have side effects, which is not the case here.
        let mut enemy_query = world.query_filtered::<(Entity, &Point), (With<Monster>, Without<Player>)>();
        for (entity, pos) in enemy_query.iter(world) {
            if *pos == destination {
                hit_something = Some(entity);
            }
        }

        if let Some(entity) = hit_something {
            world.send_event(WantsToAttack(player, entity))
        } else {
            world.send_event(WantsToMove(player, destination))
        }

        world.remove_resource::<VirtualKeyCode>();
    }
}

fn try_next_level(world: &mut World) -> bool {
    world.resource_scope(|world, map: Mut<Map>| {
        let player_pos = world.resource::<Point>();
        map.tiles[map.point2d_to_index(*player_pos)].tile_type == TileType::DownStairs
    })
}

fn try_pickup_item(
    world: &mut World,
    mut player_query: QueryState<(Entity, &Point), (With<Player>, Without<Monster>)>,
) {
    let mut items_query = world.query_filtered::<(Entity, &Point), With<Item>>();
    let (player, pos) = player_query.single(world);

    for (entity, item_pos) in items_query.iter(world) {
        if *item_pos == *pos {
            return world.send_event(WantsToPickupItem(player, entity));
        }
    }

    bo_logging::Logger::new().append("There is nothing here to pick up.").log()
}

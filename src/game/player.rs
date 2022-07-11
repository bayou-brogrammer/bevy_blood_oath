use std::collections::HashSet;

use crate::prelude::*;

pub fn build_input_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(player_input_system())
        .add_system(update_fov_system())
        .build()
}

pub fn build_player_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(tile_triggers_system())
        .build()
}

#[system]
#[read_component(Player)]
#[write_component(Position)]
#[write_component(FieldOfView)]
#[read_component(Door)]
pub fn player_input(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] map: &mut Map,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] turn_state: &mut TurnState,
) {
    if let Some(key) = *key {
        let mut find_player =
            <(&mut Position, &mut FieldOfView)>::query().filter(component::<Player>());
        let mut result = TurnState::WaitingForInput;
        let mut doors_to_delete = HashSet::new();

        let delta = match key {
            VirtualKeyCode::Up | VirtualKeyCode::W => Point::new(0, -1),
            VirtualKeyCode::Down | VirtualKeyCode::A => Point::new(0, 1),
            VirtualKeyCode::Left | VirtualKeyCode::S => Point::new(-1, 0),
            VirtualKeyCode::Right | VirtualKeyCode::D => Point::new(1, 0),
            _ => Point::new(0, 0),
        };

        find_player.iter_mut(ecs).for_each(|(pos, fov)| {
            let new_pos = pos.pt + delta;
            let new_idx = map.get_current().point2d_to_index(new_pos);

            if !map.get_current().tiles[new_idx].blocked {
                pos.pt = new_pos;
                fov.is_dirty = true;
                result = TurnState::PlayerTurn;
            } else if map.get_current().is_door[new_idx] {
                map.get_current_mut().open_door(new_idx);
                doors_to_delete.insert(map.get_current().index_to_point2d(new_idx));
            }
        });

        if !doors_to_delete.is_empty() {
            let mut q = <(Entity, &Position, &Door)>::query();
            q.for_each(ecs, |(entity, pos, _)| {
                if pos.layer == map.current_layer && doors_to_delete.contains(&pos.pt) {
                    commands.remove(*entity);
                }
            });
        }

        *turn_state = result;
    }
}

#[system]
#[read_component(Player)]
#[read_component(Position)]
#[read_component(Player)]
#[read_component(TileTrigger)]
fn tile_triggers(ecs: &mut SubWorld, #[resource] turn_state: &mut TurnState) {
    let mut find_player = <&Position>::query().filter(component::<Player>());
    let player_pos = find_player.iter(ecs).copied().next().unwrap();

    let mut find_triggers = <(&TileTrigger, &Position)>::query();
    find_triggers
        .iter(ecs)
        .filter(|(_, pos)| **pos == player_pos)
        .for_each(|(tt, _)| match tt.0 {
            TriggerType::EndGame => *turn_state = TurnState::GameOverLeft,
        });
}

#[system]
#[read_component(Position)]
#[write_component(FieldOfView)]
pub fn update_fov(world: &mut SubWorld, #[resource] map: &mut Map) {
    // Build the player FOV

    let mut views = <(Entity, &Position, &mut FieldOfView)>::query();
    let mut player_q = <Entity>::query().filter(component::<Player>());
    let player = *player_q.iter(world).next().unwrap();

    views
        .iter_mut(world)
        .filter(|(_, _, fov)| fov.is_dirty)
        .for_each(|(entity, pos, mut fov)| {
            fov.visible_tiles = field_of_view_set(pos.pt, fov.radius, map.get_current());
            fov.is_dirty = false;

            if *entity == player {
                let current_layer = map.get_current_mut();
                current_layer.clear_visible();
                fov.visible_tiles.iter().for_each(|pt| {
                    if current_layer.in_bounds(*pt) {
                        let idx = current_layer.point2d_to_index(*pt);
                        current_layer.revealed[idx] = true;
                        current_layer.visible[idx] = true;
                    }
                });
            }
        });
}

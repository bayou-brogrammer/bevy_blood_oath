use super::*;

pub fn try_move_player(delta_pt: Point, ecs: &mut World) {
    let map = ecs.fetch::<Map>();
    let entities = ecs.entities();

    let players = ecs.read_storage::<Player>();
    let mut positions = ecs.write_storage::<Position>();
    let mut fovs = ecs.write_storage::<FieldOfView>();
    let combat_stats = ecs.read_storage::<CombatStats>();
    let mut wants_to_melee = ecs.write_storage::<WantsToMelee>();

    for (entity, _player, pos, fov) in (&entities, &players, &mut positions, &mut fovs).join() {
        let destination = pos.0 + delta_pt;
        if !map.in_bounds(destination) {
            return;
        }

        let destination_idx = map.point2d_to_index(destination);

        // for potential_target in map.tile_content[destination_idx].iter() {
        //     let target = combat_stats.get(*potential_target);
        //     if let Some(_target) = target {
        //         wants_to_melee
        //             .insert(
        //                 entity,
        //                 WantsToMelee {
        //                     target: *potential_target,
        //                 },
        //             )
        //             .expect("Add target failed");
        //         return;
        //     }
        // }

        if map.can_enter_tile(destination) {
            pos.0 = destination;
            fov.is_dirty = true;

            let mut ppos = ecs.write_resource::<Point>();
            *ppos = pos.0;
        }
    }
}

#[rustfmt::skip]
pub fn player_input(gs: &mut State, ctx: &mut BTerm) -> TurnState {
    // Player movement
    match ctx.key {
        None => return TurnState::AwaitingInput, // Nothing happened
        Some(key) => match key {
            VirtualKeyCode::Left | VirtualKeyCode::Numpad4 | VirtualKeyCode::H => { try_move_player(Point::new(-1, 0), &mut gs.world) }
            VirtualKeyCode::Right | VirtualKeyCode::Numpad6 | VirtualKeyCode::L => { try_move_player(Point::new(1, 0), &mut gs.world) }
            VirtualKeyCode::Up | VirtualKeyCode::Numpad8 | VirtualKeyCode::K => { try_move_player(Point::new(0, -1), &mut gs.world) }
            VirtualKeyCode::Down | VirtualKeyCode::Numpad2 | VirtualKeyCode::J => { try_move_player(Point::new(0, 1), &mut gs.world) }

            // Diagonals
            VirtualKeyCode::Numpad9 | VirtualKeyCode::U => try_move_player(Point::new(1, -1), &mut gs.world),
            VirtualKeyCode::Numpad7 | VirtualKeyCode::Y => try_move_player(Point::new(-1, -1), &mut gs.world),
            VirtualKeyCode::Numpad3 | VirtualKeyCode::N => try_move_player(Point::new(1, 1), &mut gs.world),
            VirtualKeyCode::Numpad1 | VirtualKeyCode::B => try_move_player(Point::new(-1, 1), &mut gs.world),

            _ => return TurnState::AwaitingInput,
        },
    }
    TurnState::PlayerTurn
}

use super::*;

fn random_start_position() -> (XStart, YStart) {
    let xroll = crate::rng::roll_dice(1, 3);
    let x = match xroll {
        1 => XStart::LEFT,
        2 => XStart::CENTER,
        _ => XStart::RIGHT,
    };

    let yroll = crate::rng::roll_dice(1, 3);
    let y = match yroll {
        1 => YStart::BOTTOM,
        2 => YStart::CENTER,
        _ => YStart::TOP,
    };

    (x, y)
}

fn random_shape_builder(builder: &mut BuilderChain) {
    let builder_roll = crate::rng::roll_dice(1, 16);
    match builder_roll {
        1 => builder.start_with(CellularAutomataBuilder::new()),
        2 => builder.start_with(DrunkardsWalkBuilder::open_area()),
        3 => builder.start_with(DrunkardsWalkBuilder::open_halls()),
        4 => builder.start_with(DrunkardsWalkBuilder::winding_passages()),
        5 => builder.start_with(DrunkardsWalkBuilder::fat_passages()),
        6 => builder.start_with(DrunkardsWalkBuilder::fearful_symmetry()),
        7 => builder.start_with(MazeBuilder::new()),
        8 => builder.start_with(DLABuilder::walk_inwards()),
        9 => builder.start_with(DLABuilder::walk_outwards()),
        10 => builder.start_with(DLABuilder::central_attractor()),
        11 => builder.start_with(DLABuilder::insectoid()),
        12 => builder.start_with(VoronoiCellBuilder::pythagoras()),
        13 => builder.start_with(VoronoiCellBuilder::manhattan()),
        _ => builder.start_with(PrefabBuilder::constant(prefab_levels::WFC_POPULATED)),
    }

    println!("Random shape start with: {}", builder_roll);

    // Set the start to the center and cull
    builder.with(AreaStartingPosition::new(XStart::CENTER, YStart::CENTER));
    builder.with(CullUnreachable::new());

    // // Now set the start to a random starting area
    let (start_x, start_y) = random_start_position();
    builder.with(AreaStartingPosition::new(start_x, start_y));

    // // Setup an exit and spawn mobs
    builder.with(VoronoiSpawning::new());
    builder.with(DistantExit::new());

    println!("random_shape_builder veroni");
}

fn random_room_builder(builder: &mut BuilderChain) {
    let build_roll = crate::rng::roll_dice(1, 3);
    match build_roll {
        1 => builder.start_with(SimpleMapBuilder::new()),
        2 => builder.start_with(BspDungeonBuilder::new()),
        _ => builder.start_with(BspInteriorBuilder::new()),
    }

    println!("Random Room start with: {}", build_roll);

    // BSP Interior still makes holes in the walls
    if build_roll != 3 {
        // Sort by one of the 5 available algorithms
        let sort_roll = crate::rng::roll_dice(1, 5);
        match sort_roll {
            1 => builder.with(RoomSorter::new(RoomSort::LEFTMOST)),
            2 => builder.with(RoomSorter::new(RoomSort::RIGHTMOST)),
            3 => builder.with(RoomSorter::new(RoomSort::TOPMOST)),
            4 => builder.with(RoomSorter::new(RoomSort::BOTTOMMOST)),
            _ => builder.with(RoomSorter::new(RoomSort::CENTRAL)),
        }

        builder.with(RoomDrawer::new());

        let corridor_roll = crate::rng::roll_dice(1, 4);
        match corridor_roll {
            1 => builder.with(DoglegCorridors::new()),
            2 => builder.with(NearestCorridors::new()),
            3 => builder.with(StraightLineCorridors::new()),
            _ => builder.with(BspCorridors::new()),
        }

        let cspawn_roll = crate::rng::roll_dice(1, 2);
        if cspawn_roll == 1 {
            builder.with(CorridorSpawner::new());
        }

        let modifier_roll = crate::rng::roll_dice(1, 6);
        match modifier_roll {
            1 => builder.with(RoomExploder::new()),
            2 => builder.with(RoomCornerRounder::new()),
            _ => {}
        }
    }

    let start_roll = crate::rng::roll_dice(1, 2);
    match start_roll {
        1 => builder.with(RoomBasedStartingPosition::new()),
        _ => {
            let (start_x, start_y) = random_start_position();
            builder.with(AreaStartingPosition::new(start_x, start_y));
        }
    }

    let exit_roll = crate::rng::roll_dice(1, 2);
    match exit_roll {
        1 => builder.with(RoomBasedStairs::new()),
        _ => builder.with(DistantExit::new()),
    }

    let spawn_roll = crate::rng::roll_dice(1, 2);
    match spawn_roll {
        1 => builder.with(RoomBasedSpawner::new()),
        _ => builder.with(VoronoiSpawning::new()),
    }

    println!("Random Room spawning: {}", spawn_roll);
}

pub fn random_builder(new_depth: i32, width: i32, height: i32) -> BuilderChain {
    let mut builder = BuilderChain::new(new_depth, width, height, "New Map");

    let type_roll = crate::rng::roll_dice(1, 2);
    match type_roll {
        1 => random_room_builder(&mut builder),
        _ => random_shape_builder(&mut builder),
    }

    if crate::rng::roll_dice(1, 3) == 1 {
        builder.with(WaveformCollapseBuilder::new());

        // Now set the start to a random starting area
        let (start_x, start_y) = random_start_position();
        builder.with(AreaStartingPosition::new(start_x, start_y));

        // Setup an exit and spawn mobs
        builder.with(VoronoiSpawning::new());
        builder.with(DistantExit::new());
    }

    builder.with(DoorPlacement::new());
    builder.with(WallBoundaries::new());
    builder.with(PrefabBuilder::vaults());

    builder
}

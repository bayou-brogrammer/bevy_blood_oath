use std::collections::HashSet;

use crate::prelude::*;

pub fn spawn_player(world: &mut World, start_pos: Point) -> Entity {
    world
        .spawn()
        .insert_bundle(RenderBundle {
            position: Position::new(start_pos),
            glyph: Glyph::new(
                to_cp437('@'),
                ColorPair::new(YELLOW, BLACK),
                RenderOrder::Actor,
            ),
        })
        .insert_bundle(EntityBundle {
            tag: Player,
            fov: FieldOfView::new(8),
            name: Name("SecBot".to_string()),
            stats: CombatStats::new(30, 30, 2, 5),
            description: Description("Everybody's favorite Bracket Corp SecBot".to_string()),
        })
        .id()
}

const MAX_MONSTERS: i32 = 4;
const MAX_ITEMS: i32 = 2;

pub fn spawn_room(world: &mut World, room: &Rect) {
    let mut monster_spawn_points: HashSet<Point> = HashSet::new();
    let mut rng = crate::rng::RNG.write();

    // Scope to keep the borrow checker happy
    // {
    //     let num_monsters = rng.roll_dice(1, MAX_MONSTERS + 2) - 3;

    //     for _i in 0..num_monsters {
    //         let mut added = false;
    //         while !added {
    //             let x = (room.x1 + rng.roll_dice(1, i32::abs(room.x2 - room.x1))) as usize;
    //             let y = (room.y1 + rng.roll_dice(1, i32::abs(room.y2 - room.y1))) as usize;
    //             let pt = Point::new(x, y);

    //             if !monster_spawn_points.contains(&pt) {
    //                 monster_spawn_points.insert(pt);
    //                 added = true;
    //             }
    //         }
    //     }
    // }

    monster_spawn_points.insert(room.center());

    monster_spawn_points
        .iter()
        .for_each(|pt| random_monster(world, &mut rng, *pt));
}

/// Spawns a random monster at a given location
pub fn random_monster(world: &mut World, rng: &mut RandomNumberGenerator, pt: Point) {
    let roll = rng.roll_dice(1, 2);
    match roll {
        1 => orc(world, pt),
        _ => goblin(world, pt),
    };
}

fn orc(world: &mut World, pt: Point) {
    monster(world, pt, to_cp437('o'), "Orc", "An ugly powerful orc");
}

fn goblin(world: &mut World, pt: Point) {
    monster(world, pt, to_cp437('g'), "Goblin", "An ugly goblin");
}

pub fn monster(world: &mut World, start_pos: Point, glyph: FontCharType, name: &str, desc: &str) {
    world
        .spawn()
        .insert_bundle(RenderBundle {
            glyph: Glyph::new(glyph, ColorPair::new(RED, BLACK), RenderOrder::Actor),
            position: Position::new(start_pos),
        })
        .insert_bundle(MonsterBundle {
            blocks: BlocksTile,
            monster: EntityBundle {
                tag: Monster,
                fov: FieldOfView::new(6),
                name: Name(name.to_string()),
                stats: CombatStats::new(16, 16, 1, 4),
                description: Description(desc.to_string()),
            },
        });
}

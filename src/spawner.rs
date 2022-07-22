use crate::prelude::*;
use bracket_lib::prelude::Rect;
use std::collections::HashSet;

pub fn spawn_player(world: &mut World, start_pos: Point) -> Entity {
    world
        .spawn()
        .insert_bundle(FighterBundle::new(
            EntityBundle::new(Player, "SecBot", "Everybody's favorite Bracket Corp SecBot"),
            FieldOfView::new(8),
            CombatStats::new(30, 30, 2, 5),
        ))
        .insert_bundle(RenderBundle {
            position: Position::new(start_pos),
            glyph: Glyph::new(to_cp437('@'), ColorPair::new(YELLOW, BLACK), RenderOrder::Actor),
        })
        .id()
}

const MAX_MONSTERS: i32 = 10;
const MAX_ITEMS: i32 = 2;

pub fn spawn_room(world: &mut World, room: &Rect) {
    let mut rng = crate::rng::RNG.lock();

    let num_monsters = i32::max(0, rng.roll_dice(1, MAX_MONSTERS + 2) - 3);
    let mut monster_spawn_points: HashSet<Point> = HashSet::new();
    (0..num_monsters).for_each(|_| loop {
        let x = (room.x1 + rng.roll_dice(1, i32::abs(room.x2 - room.x1))) as usize;
        let y = (room.y1 + rng.roll_dice(1, i32::abs(room.y2 - room.y1))) as usize;
        let pt = Point::new(x, y);

        if !monster_spawn_points.contains(&pt) {
            monster_spawn_points.insert(pt);
            break;
        }
    });

    monster_spawn_points.iter().for_each(|pt| random_monster(world, &mut rng, *pt));

    let num_items = rng.roll_dice(1, MAX_ITEMS + 2) - 3;
    let mut item_spawn_points: HashSet<Point> = HashSet::new();
    (0..num_items).for_each(|_| loop {
        let x = (room.x1 + rng.roll_dice(1, i32::abs(room.x2 - room.x1))) as usize;
        let y = (room.y1 + rng.roll_dice(1, i32::abs(room.y2 - room.y1))) as usize;
        let pt = Point::new(x, y);

        if !item_spawn_points.contains(&pt) {
            item_spawn_points.insert(pt);
            break;
        }
    });

    item_spawn_points.iter().for_each(|pt| health_potion(world, *pt));
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
    monster(world, pt, 157, "Orc", "An ugly powerful orc")
}

fn goblin(world: &mut World, pt: Point) {
    monster(world, pt, to_cp437('g'), "Goblin", "An ugly goblin")
}

pub fn monster(world: &mut World, start_pos: Point, glyph: FontCharType, name: &str, desc: &str) {
    world
        .spawn()
        .insert_bundle(MonsterBundle::new(FighterBundle::new(
            EntityBundle::new(Monster, name, desc),
            FieldOfView::new(6),
            CombatStats::new(16, 16, 1, 4),
        )))
        .insert_bundle(RenderBundle {
            position: Position::new(start_pos),
            glyph: Glyph::new(glyph, ColorPair::new(RED, BLACK), RenderOrder::Actor),
        });
}

pub fn health_potion(world: &mut World, pt: Point) {
    world
        .spawn()
        .insert_bundle(ItemBundle::new(
            EntityBundle::new(Item, "Health Potion", "A potion that restores health"),
            RenderBundle::new(to_cp437('¡'), ColorPair::new(MAGENTA, BLACK), RenderOrder::Item, pt),
        ))
        .insert(Consumable)
        .insert(ProvidesHealing(10));
}

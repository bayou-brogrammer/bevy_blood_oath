use crate::prelude::*;
use std::collections::HashSet;

pub fn spawn_player(mut commands: Commands, map: Res<Map>) {
    let start_pos = map.rooms[0].center();

    // Spawn Player
    commands
        .spawn()
        .insert_bundle(FighterBundle::new(
            EntityBundle::new(Player, "SecBot"),
            FieldOfView::new(8),
            CombatStats::new(30, 30, 2, 5),
        ))
        .insert_bundle(RenderBundle {
            position: Position::new(start_pos),
            glyph: Glyph::new(to_cp437('@'), ColorPair::new(YELLOW, BLACK), RenderOrder::Actor),
        })
        .insert(Description::new("A bot that can attack and move."));

    commands.insert_resource(start_pos);
    commands.insert_resource(camera::GameCamera::new(start_pos));

    spawner::confusion_scroll(&mut commands, start_pos);
    spawner::magic_missile_scroll(&mut commands, start_pos);
    spawner::fireball_scroll(&mut commands, start_pos);
}

pub fn spawn_enemies(mut commands: Commands, map: Res<Map>) {
    // Spawn Enemies
    map.rooms.iter().skip(1).for_each(|room| {
        spawner::spawn_room(&mut commands, room);
    });
}

const MAX_MONSTERS: i32 = 10;
const MAX_ITEMS: i32 = 2;

pub fn spawn_room(commands: &mut Commands, room: &Rect) {
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

    monster_spawn_points.iter().for_each(|pt| random_monster(commands, &mut rng, *pt));

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

    item_spawn_points.iter().for_each(|pt| random_item(commands, &mut rng, *pt));
}

/// Spawns a random monster at a given location
pub fn random_monster(commands: &mut Commands, rng: &mut RandomNumberGenerator, pt: Point) {
    let roll = rng.roll_dice(1, 2);
    match roll {
        1 => orc(commands, pt),
        _ => goblin(commands, pt),
    };
}

fn orc(commands: &mut Commands, pt: Point) {
    monster(commands, pt, 157, "Orc", "An ugly powerful orc")
}

fn goblin(commands: &mut Commands, pt: Point) {
    monster(commands, pt, to_cp437('g'), "Goblin", "An ugly goblin")
}

pub fn monster(commands: &mut Commands, start_pos: Point, glyph: FontCharType, name: &str, desc: &str) {
    commands
        .spawn()
        .insert_bundle(MonsterBundle::new(FighterBundle::new(
            EntityBundle::new(Monster, name),
            FieldOfView::new(6),
            CombatStats::new(16, 16, 1, 4),
        )))
        .insert_bundle(RenderBundle {
            position: Position::new(start_pos),
            glyph: Glyph::new(glyph, ColorPair::new(RED, BLACK), RenderOrder::Actor),
        })
        .insert(Description::new(desc));
}

fn random_item(commands: &mut Commands, rng: &mut RandomNumberGenerator, pt: Point) {
    let roll = rng.roll_dice(1, 2);
    match roll {
        1 => health_potion(commands, pt),
        2 => fireball_scroll(commands, pt),
        3 => confusion_scroll(commands, pt),
        _ => magic_missile_scroll(commands, pt),
    }
}

pub fn health_potion(commands: &mut Commands, pt: Point) {
    commands
        .spawn()
        .insert_bundle(ItemBundle::new(
            EntityBundle::new(Item, "Health Potion"),
            RenderBundle::new(to_cp437('¡'), ColorPair::new(MAGENTA, BLACK), RenderOrder::Item, pt),
        ))
        .insert(Consumable)
        .insert(ProvidesHealing(10));
}

pub fn magic_missile_scroll(commands: &mut Commands, pt: Point) {
    commands
        .spawn()
        .insert_bundle(ItemBundle::new(
            EntityBundle::new(Item, "Magic Missile Scroll"),
            RenderBundle::new(to_cp437(')'), ColorPair::new(CYAN, BLACK), RenderOrder::Item, pt),
        ))
        .insert(Consumable)
        .insert(Ranged { range: 6 })
        .insert(InflictsDamage { damage: 100 });
}

pub fn fireball_scroll(commands: &mut Commands, pt: Point) {
    commands
        .spawn()
        .insert_bundle(ItemBundle::new(
            EntityBundle::new(Item, "Fireball Scroll"),
            RenderBundle::new(to_cp437(')'), ColorPair::new(ORANGE, BLACK), RenderOrder::Item, pt),
        ))
        .insert(Consumable)
        .insert(Ranged { range: 6 })
        .insert(InflictsDamage { damage: 20 })
        .insert(AreaOfEffect { radius: 3 });
}

pub fn confusion_scroll(commands: &mut Commands, pt: Point) {
    commands
        .spawn()
        .insert_bundle(ItemBundle::new(
            EntityBundle::new(Item, "Confusion Scroll"),
            RenderBundle::new(to_cp437(')'), ColorPair::new(ORANGE, BLACK), RenderOrder::Item, pt),
        ))
        .insert(Consumable)
        .insert(Ranged { range: 6 })
        .insert(Confusion { turns: 4 });
}

pub struct SpawnerPlugin;
impl Plugin for SpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system_set(
            GameCondition::InGame,
            ConditionSet::new().with_system(spawn_player).with_system(spawn_enemies).into(),
        );

        app.add_exit_system(GameCondition::InGame, cleanup_system::<Position>);
    }
}

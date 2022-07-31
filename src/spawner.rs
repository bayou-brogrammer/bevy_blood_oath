use crate::prelude::*;
use std::collections::hash_map::Entry::Vacant;
use std::collections::HashMap;

pub fn spawn_player(mut commands: Commands, map: Res<Map>) {
    let start_pos = map.rooms[0].center();

    // Spawn Player
    let player = commands
        .spawn()
        .insert_bundle(PlayerBundle::new(FighterBundle::new(
            FieldOfView::new(8),
            CombatStats::new(30, 30, 2, 5),
        )))
        .insert_bundle(RenderBundle {
            position: Position::new(start_pos),
            glyph: Glyph::new(to_cp437('@'), ColorPair::new(YELLOW, BLACK), RenderOrder::Actor),
        })
        .insert(Naming("SecBot".to_string()))
        .insert(Description::new("A bot that can attack and move."))
        .insert(Blood(DARK_RED.into()))
        .id();

    commands.insert_resource(player);
    commands.insert_resource(start_pos);
    commands.insert_resource(camera::GameCamera::new(start_pos));

    spawner::confusion_scroll(&mut commands, start_pos);
    spawner::magic_missile_scroll(&mut commands, start_pos);
    spawner::fireball_scroll(&mut commands, start_pos);
}

fn room_table(map_depth: i32) -> RandomTable {
    RandomTable::new()
        .add("Goblin", 10)
        .add("Orc", 1 + map_depth)
        .add("Health Potion", 7)
        .add("Fireball Scroll", 2 + map_depth)
        .add("Confusion Scroll", 2 + map_depth)
        .add("Magic Missile Scroll", 4)
        .add("Dagger", 3)
        .add("Shield", 3)
        .add("Longsword", map_depth - 1)
        .add("Tower Shield", map_depth - 1)
}

pub fn spawn_enemies(mut commands: Commands, map: Res<Map>) {
    // Spawn Enemies
    map.rooms.iter().skip(1).for_each(|room| {
        spawner::spawn_room(&mut commands, room, map.depth);
    });

    dagger(&mut commands, map.rooms[0].center());
    shield(&mut commands, map.rooms[0].center());
}

const MAX_MONSTERS: i32 = 10;

pub fn spawn_room(commands: &mut Commands, room: &Rect, map_depth: i32) {
    let mut rng = bo_utils::rng::RNG.lock();
    let spawn_table = room_table(map_depth);
    let mut spawn_points: HashMap<Point, String> = HashMap::new();

    // Scope to keep the borrow checker happy
    {
        let num_spawns = rng.roll_dice(1, MAX_MONSTERS + 3) + (map_depth - 1) - 3;

        for _i in 0..num_spawns {
            let mut added = false;
            let mut tries = 0;

            while !added && tries < 20 {
                let x = (room.x1 + rng.roll_dice(1, i32::abs(room.x2 - room.x1))) as usize;
                let y = (room.y1 + rng.roll_dice(1, i32::abs(room.y2 - room.y1))) as usize;

                if let Vacant(e) = spawn_points.entry(Point::new(x, y)) {
                    e.insert(spawn_table.roll(&mut rng));
                    added = true;
                } else {
                    tries += 1;
                }

                tries += 1;
            }
        }
    }

    spawn_points.iter().for_each(|(pt, spawn_name)| match spawn_name.as_ref() {
        "Orc" => orc(commands, *pt),
        "Goblin" => goblin(commands, *pt),
        "Dagger" => dagger(commands, *pt),
        "Shield" => shield(commands, *pt),
        "Longsword" => longsword(commands, *pt),
        "Tower Shield" => tower_shield(commands, *pt),
        "Health Potion" => health_potion(commands, *pt),
        "Fireball Scroll" => fireball_scroll(commands, *pt),
        "Confusion Scroll" => confusion_scroll(commands, *pt),
        "Magic Missile Scroll" => magic_missile_scroll(commands, *pt),
        _ => {}
    });
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
            FieldOfView::new(8),
            CombatStats::new(16, 16, 1, 4),
        )))
        .insert_bundle(RenderBundle {
            position: Position::new(start_pos),
            glyph: Glyph::new(glyph, ColorPair::new(RED, BLACK), RenderOrder::Actor),
        })
        .insert(Naming(name.to_string()))
        .insert(Description::new(desc))
        .insert(Blood(DARK_GREEN.into()));
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

fn dagger(commands: &mut Commands, pt: Point) {
    commands
        .spawn()
        .insert_bundle(ItemBundle::new(
            EntityBundle::new(Item, "Dagger"),
            RenderBundle::new(to_cp437('/'), ColorPair::new(CYAN, BLACK), RenderOrder::Item, pt),
        ))
        .insert(Equippable { slot: EquipmentSlot::Melee })
        .insert(MeleePowerBonus::new(2));
}

fn shield(commands: &mut Commands, pt: Point) {
    commands
        .spawn()
        .insert_bundle(ItemBundle::new(
            EntityBundle::new(Item, "Shield"),
            RenderBundle::new(to_cp437('('), ColorPair::new(CYAN, BLACK), RenderOrder::Item, pt),
        ))
        .insert(Equippable { slot: EquipmentSlot::Melee })
        .insert(DefenseBonus::new(1));
}

fn longsword(commands: &mut Commands, pt: Point) {
    commands
        .spawn()
        .insert_bundle(ItemBundle::new(
            EntityBundle::new(Item, "Longsword"),
            RenderBundle::new(to_cp437('/'), ColorPair::new(YELLOW, BLACK), RenderOrder::Item, pt),
        ))
        .insert(Equippable { slot: EquipmentSlot::Melee })
        .insert(MeleePowerBonus::new(4));
}

fn tower_shield(commands: &mut Commands, pt: Point) {
    commands
        .spawn()
        .insert_bundle(ItemBundle::new(
            EntityBundle::new(Item, "Tower Shield"),
            RenderBundle::new(to_cp437('('), ColorPair::new(YELLOW, BLACK), RenderOrder::Item, pt),
        ))
        .insert(Equippable { slot: EquipmentSlot::Shield })
        .insert(DefenseBonus::new(3));
}

pub struct SpawnerPlugin;
impl Plugin for SpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system_set(
            GameCondition::InGame,
            ConditionSet::new().with_system(spawn_player).with_system(spawn_enemies).into(),
        );

        app.add_exit_system(GameCondition::InGame, clear_entities);
    }
}

#![allow(non_snake_case)]
#![allow(unused_variables)]

use super::*;
use crate::*;

use bevy::{
    ecs::system::EntityCommands,
    prelude::{Commands, Entity},
};

pub enum SpawnType {
    Carried(Entity),
    Equipped(Entity),
    AtPosition(Point),
}

pub fn spawn_position(pos: SpawnType, eb: &mut EntityCommands, tag: &str, raws: &RawMaster) {
    // Spawn in the specified location
    match pos {
        SpawnType::Equipped(by) => {}
        SpawnType::AtPosition(pt) => {
            eb.insert(pt);
        }
        SpawnType::Carried(by) => {
            eb.insert(InBackpack { owner: by });
        }
    }
}

pub fn spawn_base_entity<T: BaseRawComponent + Clone>(
    raws: &RawMaster,
    eb: &mut EntityCommands,
    entity_list: &[T],
    indexes: &HashMap<String, usize>,
    key: &str,
    pos: SpawnType,
) -> T {
    let entity_template = &entity_list[indexes[key]];

    // Spawn in the specified location
    spawn_position(pos, eb, key, raws);

    // Renderable
    if let Some(glyph) = &entity_template.glyph() {
        eb.insert(get_renderable_component(glyph));
    }

    // Name Component
    eb.insert(Naming(entity_template.name()));

    entity_template.clone()
}

macro_rules! apply_effects {
    ( $effects:expr, $eb:expr ) => {
        for effect in $effects.iter() {
        let default = "".to_string();
        let effect_name = effect.0.as_str();
        let effect_options = effect.1.as_ref().unwrap_or(&default);
            match effect_name {
                AREA_OF_EFFECT => $eb.insert(AreaOfEffect::new(effect_options.parse::<i32>().unwrap())),
                // CONFUSION => {
                //     $eb.insert(Confusion{});
                //     $eb.insert(Duration{ turns: effect.1.unwrap().parse::<i32>().unwrap() });
                // }
                DAMAGE => $eb.insert(InflictsDamage(effect_options.parse::<i32>().unwrap())),
                // "damage_over_time" => $eb = $eb.with( DamageOverTime { damage : effect.1.unwrap().parse::<i32>().unwrap() } ),
                // "duration" => $eb = $eb.with(Duration { turns: effect.1.unwrap().parse::<i32>().unwrap() }),
                FOOD => $eb.insert(ProvidesFood{}),
                // "identify" => $eb = $eb.with(ProvidesIdentification{}),
                MAGIC_MAPPING => $eb.insert(MagicMapper{}),
                PARTICLE => $eb.insert(parse_particle(effect_options)),
                PARTICLE_LINE => $eb.insert(parse_particle_line(effect_options)),
                PROVIDES_HEALING => $eb.insert(ProvidesHealing(effect_options.parse::<i32>().unwrap())),
                // "provides_mana" => $eb = $eb.with(ProvidesMana{ mana_amount: effect.1.unwrap().parse::<i32>().unwrap() }),
                RANGED => $eb.insert(Ranged(effect_options.parse::<i32>().unwrap())),
                // "remove_curse" => $eb = $eb.with(ProvidesRemoveCurse{}),
                SINGLE_ACTIVATION => $eb.insert(SingleActivation{}),
                // "slow" => $eb = $eb.with(Slow{ initiative_penalty : effect.1.unwrap().parse::<f32>().unwrap() }),
                // "target_self" => $eb = $eb.with( AlwaysTargetsSelf{} ),
                // "teach_spell" => $eb = $eb.with(TeachesSpell{ spell: effect.1.unwrap().to_string() }),
                // "town_portal" => $eb = $eb.with(TownPortal{}),
                _ => {println!("Warning: consumable effect {} not implemented.", effect_name); $eb}
            };
        }
    };
}

pub fn spawn_named_item(
    raws: &RawMaster,
    commands: &mut Commands,
    key: &str,
    pos: SpawnType,
) -> Option<Entity> {
    let mut eb = commands.spawn();
    let item_template = spawn_base_entity(raws, &mut eb, &raws.raws.items, &raws.item_index, key, pos);

    eb.insert(Item {});

    // Consumable
    if let Some(consumable) = &item_template.consumable {
        eb.insert(Consumable {});
        apply_effects!(consumable.effects, &mut eb);
    }

    // Weapon
    if let Some(weapon) = &item_template.weapon {
        eb.insert(Equippable::new(EquipmentSlot::Melee));
        eb.insert(MeleePowerBonus::new(weapon.power_bonus));
    }
    // Shield
    if let Some(shield) = &item_template.shield {
        eb.insert(Equippable::new(EquipmentSlot::Shield));
        eb.insert(DefenseBonus::new(shield.defense_bonus));
    }

    Some(eb.id())
}

pub fn spawn_named_mob(
    raws: &RawMaster,
    commands: &mut Commands,
    key: &str,
    pos: SpawnType,
) -> Option<Entity> {
    let mut eb = commands.spawn();
    let mob_template = spawn_base_entity(raws, &mut eb, &raws.raws.mobs, &raws.mob_index, key, pos);

    eb.insert(Monster {});
    if mob_template.blocks_tile {
        eb.insert(BlocksTile {});
    }
    eb.insert(CombatStats {
        max_hp: mob_template.stats.max_hp,
        hp: mob_template.stats.hp,
        power: mob_template.stats.power,
        defense: mob_template.stats.defense,
    });
    eb.insert(FieldOfView::new(mob_template.vision_range));

    Some(eb.id())
}

pub fn spawn_named_prop(
    raws: &RawMaster,
    commands: &mut Commands,
    key: &str,
    pos: SpawnType,
) -> Option<Entity> {
    let mut eb = commands.spawn();
    let prop_template = spawn_base_entity(raws, &mut eb, &raws.raws.props, &raws.prop_index, key, pos);

    // Hidden Trait
    if let Some(hidden) = prop_template.hidden {
        if hidden {
            eb.insert(Hidden {});
        }
    }
    // Blocks Visibility Trait
    if let Some(blocks_visibility) = prop_template.blocks_visibility {
        if blocks_visibility {
            eb.insert(BlocksVisibility {});
        }
    }
    // Door?
    if let Some(door_open) = prop_template.door_open {
        eb.insert(Door(door_open));
    }
    // Trigger Trait (Traps)
    if let Some(entry_trigger) = &prop_template.entry_trigger {
        eb.insert(EntryTrigger {});
        apply_effects!(entry_trigger.effects, &mut eb);
    }

    Some(eb.id())
}

pub fn spawn_named_entity(commands: &mut Commands, key: &str, pos: SpawnType) -> Option<Entity> {
    let raws = RAWS.lock();
    if raws.item_index.contains_key(key) {
        return spawn_named_item(&raws, commands, key, pos);
    } else if raws.mob_index.contains_key(key) {
        return spawn_named_mob(&raws, commands, key, pos);
    } else if raws.prop_index.contains_key(key) {
        return spawn_named_prop(&raws, commands, key, pos);
    }

    None
}

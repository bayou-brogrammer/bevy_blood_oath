use crate::prelude::*;
use std::fs;

macro_rules! deserialize_individually {
    ($ecs:expr, $de:expr, $data:expr, $( $type:ty),*) => {
        $(
        DeserializeComponents::<Infallible, _>::deserialize(
            &mut ( &mut $ecs.write_storage::<$type>(), ),
            &$data.0, // entities
            &mut $data.1, // marker
            &mut $data.2, // allocater
            &mut $de,
        )
        .unwrap();
        )*
    };
}

#[rustfmt::skip]
pub fn load_game(ecs: &mut World) -> Result<(), BoxedError> {

    // Delete everything
    #[cfg(target_arch = "wasm32")]
    let to_delete = ecs.entities().join().collect::<Vec<_>>();
    
    #[cfg(not(target_arch = "wasm32"))]
    let to_delete = ecs.entities().par_join().collect::<Vec<_>>();

    ecs.delete_entities(&to_delete)?;

    let data = fs::read_to_string(SAVE_FILENAME)?;
    let mut de = ron::de::Deserializer::from_str(&data).unwrap();

    {
        let mut d = (
            &mut ecs.entities(),
            &mut ecs.write_storage::<SimpleMarker<SerializeMe>>(),
            &mut ecs.write_resource::<SimpleMarkerAllocator<SerializeMe>>(),
        );

        deserialize_individually!(
            ecs, de, d,
            Player, Monster, Item, Consumable, BlocksTile, 
            Point, Glyph, FieldOfView, Name, Description, CombatStats, OtherLevelPosition,
            WantsToMelee, WantsToPickupItem, WantsToUseItem, WantsToDropItem,
            InBackpack, Ranged, InflictsDamage, AreaOfEffect, Confusion, ProvidesHealing,
            Equippable, Equipped, DefenseBonus, MeleePowerBonus, Blood, HungerClock, MagicMapper,
            Hidden, ProvidesFood, EntryTrigger, EntityMoved, SingleActivation,
            ParticleLifetime, SerializationHelper, DMSerializationHelper
        );
    }

    let mut deleteme: Option<Entity> = None;
    let mut deleteme2: Option<Entity> = None;
    
    let mut loaded_map: Option<Map> = None;
    let mut loaded_point: Option<Point> = None;
    let mut loaded_player: Option<Entity> = None;
    let mut loaded_dm: Option<MasterDungeonMap> = None;
    {
        let entities = ecs.entities();
        let player = ecs.read_storage::<Player>();
        let position = ecs.read_storage::<Point>();

        let helper = ecs.read_storage::<SerializationHelper>();
        let helper2 = ecs.read_storage::<DMSerializationHelper>();

        // Load the map
        for (e, h) in (&entities, &helper).join() {
            deleteme = Some(e);

            let local_map = h.map.clone();
            spatial::set_size(local_map.height, local_map.width);
            loaded_map = Some(local_map);
        }

        // Load Master Dungeon Map
        for (e, h) in (&entities, &helper2).join() {
            deleteme2 = Some(e);
            loaded_dm = Some(h.map.clone());
            bo_logging::restore_log(&mut h.log.clone());
            bo_logging::load_events(h.events.clone());
        }

        // Load player + position
        for (e, _p, pos) in (&entities, &player, &position).join() {
            loaded_player = Some(e);
            loaded_point = Some(*pos);
        }

    }

    ecs.insert(loaded_dm.unwrap());  // This should panic if the dm is not loaded.
    ecs.insert(loaded_map.unwrap());  // This should panic if the map is not loaded.
    ecs.insert(loaded_point.unwrap());  // This should panic if the point is not loaded
    ecs.insert(loaded_player.unwrap());  // This should panic if the player is not loaded.
    ecs.insert(loaded_player.unwrap());  // This should panic if the player is not loaded.

    // Cleanup serialization helper
    ecs.delete_entity(deleteme.unwrap())?;
    ecs.delete_entity(deleteme2.unwrap())?;

    Ok(())
}

use crate::prelude::*;
use crate::BoxedError;
use std::path::Path;

#[cfg(not(target_arch = "wasm32"))]
use specs::saveload::SerializeComponents;

#[cfg(not(target_arch = "wasm32"))]
use std::fs::File;

#[cfg(target_os = "emscripten")]
pub const SAVE_FILENAME: &str = "/ruggrogue/savegame.ron";

#[cfg(not(target_os = "emscripten"))]
pub const SAVE_FILENAME: &str = "savegame.ron";

///////////////////////////////////////////////////////////////////////////////
/// Utility
///////////////////////////////////////////////////////////////////////////////

pub fn delete_save() {
    if Path::new(SAVE_FILENAME).exists() {
        if let Err(e) = std::fs::remove_file(SAVE_FILENAME) {
            eprintln!("Warning: saveload::delete_save_file: {}", e);
        }
    }
}

pub fn does_save_exist() -> bool {
    Path::new(SAVE_FILENAME).exists()
}

#[cfg(not(target_arch = "wasm32"))]
macro_rules! serialize_individually {
    ($ecs:expr, $ser:expr, $data:expr, $( $type:ty),*) => {
        $(
        SerializeComponents::<Infallible, SimpleMarker<SerializeMe>>::serialize(
            &( $ecs.read_storage::<$type>(), ),
            &$data.0,
            &$data.1,
            &mut $ser,
        )
        .unwrap();
        )*
    };
}

///////////////////////////////////////////////////////////////////////////////
/// Saving
///////////////////////////////////////////////////////////////////////////////

#[cfg(target_arch = "wasm32")]
pub fn save_game(_ecs: &mut World) -> Result<(), BoxedError> {
    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
#[rustfmt::skip]
pub fn save_game(ecs: &mut World) -> Result<(), BoxedError> {
    use ron::Options;
    use specs::saveload::MarkedBuilder;
    use crate::{DMSerializationHelper, SerializationHelper};

    


    // Create helper
    let mapcopy = ecs.fetch_mut::<Map>().clone();
    let dungeon_master = ecs
        .get_mut::<MasterDungeonMap>()
        .unwrap()
        .clone();

    let savehelper = ecs
        .create_entity()
        .with(SerializationHelper::new(mapcopy))
        .marked::<SimpleMarker<SerializeMe>>()
        .build();

    let savehelper2 = ecs
        .create_entity()
        .with(DMSerializationHelper::new(dungeon_master,bo_logging::clone_log(),bo_logging::clone_events()))
        .marked::<SimpleMarker<SerializeMe>>()
        .build();

    // Actually serialize
    {
        let data = ( ecs.entities(), ecs.read_storage::<SimpleMarker<SerializeMe>>() );

        let writer = File::create(SAVE_FILENAME)?;
        let mut serializer = ron::ser::Serializer::with_options(writer, Default::default(), Options::default()).unwrap();

        serialize_individually!(ecs, serializer, data, 
            Player, Monster, Item, Consumable, BlocksTile, 
            Position, Glyph, FieldOfView, Name, Description, CombatStats, OtherLevelPosition,
            WantsToMelee, WantsToPickupItem, WantsToUseItem, WantsToDropItem,
            InBackpack, Ranged, InflictsDamage, AreaOfEffect, Confusion, ProvidesHealing,
            Equippable, Equipped, DefenseBonus, MeleePowerBonus, Blood, HungerClock, MagicMapper,
            Hidden, ProvidesFood, EntryTrigger, EntityMoved, SingleActivation,
            ParticleLifetime, SerializationHelper, DMSerializationHelper
        );
    }

    // Clean up
    ecs.delete_entity(savehelper)?;
    ecs.delete_entity(savehelper2)?;

    Ok(())
}

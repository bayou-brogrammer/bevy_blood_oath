use super::*;

mod collection_system;
mod drop_system;
mod equip_use;
mod remove_system;
mod use_system;

use collection_system::item_collection;
use drop_system::item_drop;
pub use equip_use::equip_use;
use remove_system::remove_item;
pub use use_system::item_use;

pub struct InventoryPlugin;
impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        // Inventory Events
        app.add_system_set_to_stage(
            CoreStage::Update,
            ConditionSet::new()
                .run_in_state(GameCondition::InGame)
                .run_on_event::<WantsToPickupItem>()
                .with_system(item_collection)
                .into(),
        )
        .add_system_set_to_stage(
            CoreStage::Update,
            ConditionSet::new()
                .run_in_state(GameCondition::InGame)
                .run_on_event::<WantsToDropItem>()
                .with_system(item_drop)
                .into(),
        )
        .add_system_set_to_stage(
            CoreStage::Update,
            ConditionSet::new()
                .run_in_state(GameCondition::InGame)
                .run_on_event::<WantsToRemoveItem>()
                .with_system(remove_item)
                .into(),
        )
        .add_system_set_to_stage(
            CoreStage::Update,
            ConditionSet::new()
                .run_in_state(GameCondition::InGame)
                .run_on_event::<WantsToEquipItem>()
                .with_system(equip_use)
                .into(),
        );
    }
}

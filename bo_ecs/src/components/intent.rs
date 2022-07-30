use crate::prelude::*;
use bo_utils::impl_new;
use bracket_geometry::prelude::Point;

#[derive(Debug, Component)]
pub struct WantsToUseItem {
    pub item: Entity,
    pub target: Option<Point>,
}

#[derive(Debug, Component)]
pub struct WantsToEquipItem {
    pub item: Entity,
}

impl_new!(WantsToEquipItem, item: Entity);
impl_new!(WantsToUseItem, item: Entity, target: Option<Point>);

use super::*;
use crate::{impl_raw, RenderOrder};

#[derive(Deserialize, Debug, Clone)]
pub struct RawItem {
    pub name: String,
    pub glyph: Option<RawGlyph>,
    pub weapon: Option<RawWeapon>,
    pub shield: Option<RawShield>,
    pub consumable: Option<RawConsumable>,
}
impl_raw!(RawItem);

#[derive(Deserialize, Debug, Clone)]
pub struct RawGlyph {
    pub fg: String,
    pub bg: String,
    pub glyph: String,
    pub order: RenderOrder,
}

#[derive(Deserialize, Debug, Clone)]
pub struct RawConsumable {
    pub effects: HashMap<String, Option<String>>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct RawWeapon {
    pub power_bonus: i32,
    pub range: Option<i32>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct RawShield {
    pub defense_bonus: i32,
}

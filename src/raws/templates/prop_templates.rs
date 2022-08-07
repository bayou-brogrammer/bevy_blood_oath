use crate::impl_raw;

use super::*;
use std::collections::HashMap;

#[derive(Deserialize, Debug, Clone)]
pub struct RawProp {
    pub name: String,
    pub hidden: Option<bool>,
    pub door_open: Option<bool>,
    pub glyph: Option<RawGlyph>,
    pub blocks_tile: Option<bool>,
    pub blocks_visibility: Option<bool>,
    pub entry_trigger: Option<RawEntryTrigger>,
}

impl_raw!(RawProp);

#[derive(Deserialize, Debug, Clone)]
pub struct RawEntryTrigger {
    pub effects: HashMap<String, Option<String>>,
}

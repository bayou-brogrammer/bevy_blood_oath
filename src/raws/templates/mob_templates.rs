use crate::impl_raw;

use super::*;

#[derive(Deserialize, Debug, Clone)]
pub struct RawMob {
    pub name: String,
    pub blocks_tile: bool,
    pub vision_range: i32,
    pub stats: RawMobStats,
    pub glyph: Option<RawGlyph>,
}
impl_raw!(RawMob);

#[derive(Deserialize, Debug, Clone)]
pub struct RawMobStats {
    pub hp: i32,
    pub power: i32,
    pub max_hp: i32,
    pub defense: i32,
}

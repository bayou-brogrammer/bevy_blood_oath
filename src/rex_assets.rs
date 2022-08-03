use crate::prelude::*;
use bracket_terminal::{embedded_resource, link_resource};

embedded_resource!(DEAD_SKULL, "../assets/xp/skull.xp");
embedded_resource!(WFC_DEMO_IMAGE1, "../assets/xp/wfc-demo1.xp");
embedded_resource!(WFC_POPULATED, "../assets/xp/wfc-populated.xp");
embedded_resource!(SMALL_DUNGEON, "../assets/xp/SmallDungeon_80x50.xp");

#[derive(Debug)]
pub struct RexAssets {
    pub menu: XpFile,
    pub skull: XpFile,
}

impl Default for RexAssets {
    fn default() -> Self {
        Self::new()
    }
}

impl RexAssets {
    pub fn new() -> RexAssets {
        link_resource!(DEAD_SKULL, "resources/skull.xp");
        link_resource!(SMALL_DUNGEON, "resources/SmallDungeon_80x50.xp");
        link_resource!(WFC_DEMO_IMAGE1, "resources/wfc-demo1.xp");
        link_resource!(WFC_POPULATED, "resources/wfc-populated.xp");

        RexAssets {
            skull: XpFile::from_resource("resources/skull.xp").unwrap(),
            menu: XpFile::from_resource("resources/SmallDungeon_80x50.xp").unwrap(),
        }
    }
}

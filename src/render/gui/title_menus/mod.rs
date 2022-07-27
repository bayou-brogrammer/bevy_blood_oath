use super::*;

mod game_over;
mod main_menu;

use game_over::*;
use main_menu::*;

pub struct TitleMenuPlugins;
impl PluginGroup for TitleMenuPlugins {
    fn build(&mut self, group: &mut bevy::app::PluginGroupBuilder) {
        group.add(MainMenuPlugin).add(GameOverPlugin);
    }
}

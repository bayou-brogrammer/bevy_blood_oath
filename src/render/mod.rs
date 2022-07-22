use crate::prelude::*;

pub mod camera;
mod gui;

pub struct RenderPlugin;
impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(gui::GUIPlugin);
        app.add_plugin(gui::MainMenuPlugin);
        app.add_plugin(camera::CameraPlugin);
    }
}

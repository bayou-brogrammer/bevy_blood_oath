use crate::prelude::*;

pub mod camera;
pub mod gui;

pub struct RenderPlugin;
impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(gui::GUIPlugin)
            .add_plugin(camera::CameraPlugin);
    }
}

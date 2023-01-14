use bevy::prelude::*;

#[cfg(feature = "debug")]
mod debug;

mod window;
use window::{window_settings, WindowUtilPlugin};

pub struct EnginePlugin;
impl Plugin for EnginePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins.set(window_settings()))
            .add_plugin(WindowUtilPlugin);

        #[cfg(feature = "debug")]
        debug::camera_controller::add(app);
    }
}

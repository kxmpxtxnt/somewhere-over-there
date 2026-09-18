pub mod config;
pub mod menu;
pub mod state;

use crate::menu::Menu;
use bevy::DefaultPlugins;
use bevy::prelude::{App, Startup};
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            Menu,
            EguiPlugin::default(),
            WorldInspectorPlugin::default(),
        ))
        .add_systems(Startup, config::load_config)
        .run();
}

use crate::menu::gui::{
    controller_observer, create_menu, highlight, hover_observer, navigate, setup,
    unfocus_on_leave,
};
use crate::menu::input::MenuAction;
use crate::menu::settings::Settings;
use crate::state::MenuState;
use bevy::app::App;
use bevy::input_focus::directional_navigation::DirectionalNavigationPlugin;
use bevy::prelude::{AppExtStates, OnEnter, Plugin, Startup, Update};
use leafwing_input_manager::prelude::InputManagerPlugin;

pub mod gui;
pub mod input;
pub mod settings;

pub struct Menu;

impl Plugin for Menu {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            DirectionalNavigationPlugin,
            InputManagerPlugin::<MenuAction>::default(),
            Settings
        ))
        .init_state::<MenuState>()
        .add_systems(Startup, setup)
        .add_systems(OnEnter(MenuState::Main), create_menu)
        .add_systems(Update, (navigate, highlight))
        .add_observer(controller_observer)
        .add_observer(hover_observer)
        .add_observer(unfocus_on_leave);
    }
}

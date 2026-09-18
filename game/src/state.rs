use bevy::prelude::States;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum MenuState {
    #[default]
    Main,
    Settings,
    //TODO: Saves
    //TODO: Friends
}
pub mod tabs;

use crate::menu::gui::{GAP, menu_button};
use crate::menu::settings::tabs::SettingsTab;
use crate::state::MenuState;
use bevy::app::App;
use bevy::input_focus::tab_navigation::TabGroup;
use bevy::prelude::{
    AlignItems, AppExtStates, Commands, Component, DespawnOnExit, FlexDirection, JustifyContent,
    NextState, Node, On, OnEnter, Plugin, PositionType, Query, Res, ResMut, State, Text, default,
    percent, px,
};
use bevy::ui_widgets::Activate;

pub struct Settings;

impl Plugin for Settings {
    fn build(&self, app: &mut App) {
        app.add_sub_state::<SettingsTab>()
            .add_systems(OnEnter(MenuState::Settings), init_settings_gui)
            .add_observer(on_tab_activate);

        for tab in SettingsTab::ALL {
            app.add_systems(OnEnter(tab), tab_content);
        }
    }
}

#[derive(Component, Clone, Copy)]
pub struct SettingsTabButton(pub SettingsTab);

pub fn init_settings_gui(mut commands: Commands) {
    commands
        .spawn((
            Node {
                width: percent(100),
                height: percent(100),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Start,
                row_gap: px(GAP),
                padding: bevy::ui::UiRect::all(px(GAP)),
                ..default()
            },
            TabGroup::default(),
            DespawnOnExit(MenuState::Settings),
        ))
        .with_children(|parent| {
            for tab in SettingsTab::ALL {
                parent.spawn(menu_button(tab.label(), SettingsTabButton(tab)));
            }
        });
}

pub fn tab_content(state: Res<State<SettingsTab>>, mut commands: Commands) {
    let tab = *state.get();
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            left: px(240),
            top: px(GAP),
            ..default()
        },
        Text::new(tab.label()),
        DespawnOnExit(tab),
    ));
}

pub fn on_tab_activate(
    event: On<Activate>,
    buttons: Query<&SettingsTabButton>,
    mut next: ResMut<NextState<SettingsTab>>,
) {
    if let Ok(button) = buttons.get(event.entity) {
        next.set(button.0);
    }
}

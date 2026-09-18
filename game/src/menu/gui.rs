use crate::menu::input::{MenuAction, input_map};
use crate::menu::settings::tabs::SettingsTab;
use crate::state::MenuState;
use MenuAction::*;
use bevy::input_focus::tab_navigation::{NavAction, TabGroup, TabIndex, TabNavigation};
use bevy::input_focus::{FocusCause, InputFocus};
use bevy::math::CompassOctant;
use bevy::prelude::{
    AlignItems, AppExit, BackgroundColor, BorderColor, Bundle, Camera2d, Color, Commands,
    Component, DespawnOnExit, Entity, FlexDirection, JustifyContent, MessageWriter, NextState,
    Node, On, Out, Over, Pointer, Query, Res, ResMut, Single, State, Text, UiRect, With, children,
    default, info, px,
};
use bevy::ui::auto_directional_navigation::{AutoDirectionalNavigation, AutoDirectionalNavigator};
use bevy::ui::percent;
use bevy::ui_widgets::{Activate, Button};
use leafwing_input_manager::prelude::ActionState;

pub const BUTTON_W: f32 = 200.0;
pub const BUTTON_H: f32 = 60.0;
pub const BUTTON_BORDER: f32 = 3.0;
pub const BUTTON_BG: Color = Color::srgb(0.3, 0.3, 0.35);
pub const FOCUS_BORDER: Color = Color::WHITE;
pub const GAP: f32 = 16.0;

#[derive(Component, Clone, Copy, Debug)]
pub enum MenuButton {
    Play,
    Settings,
    Quit,
}

pub fn menu_button(label: &str, marker: impl Bundle) -> impl Bundle {
    (
        Button,
        marker,
        Node {
            width: px(BUTTON_W),
            height: px(BUTTON_H),
            border: UiRect::all(px(BUTTON_BORDER)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(BUTTON_BG),
        BorderColor::DEFAULT,
        AutoDirectionalNavigation::default(),
        TabIndex(0),
        children![Text::new(label)],
    )
}

pub fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn(input_map());
}

pub fn create_menu(mut commands: Commands) {
    commands
        .spawn((
            Node {
                width: percent(100),
                height: percent(100),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                row_gap: px(GAP),
                ..default()
            },
            TabGroup::default(),
            DespawnOnExit(MenuState::Main),
        ))
        .with_children(|parent| {
            for (kind, label) in [
                (MenuButton::Play, "Play"),
                (MenuButton::Settings, "Settings"),
                (MenuButton::Quit, "Quit"),
            ] {
                parent.spawn(menu_button(label, kind));
            }
        });
}

#[expect(clippy::too_many_arguments)]
pub fn navigate(
    state: Single<&ActionState<MenuAction>>,
    mut nav: AutoDirectionalNavigator,
    tab: TabNavigation,
    buttons: Query<(), With<Button>>,
    mut commands: Commands,
    mut next_menu: ResMut<NextState<MenuState>>,
    tab_state: Option<Res<State<SettingsTab>>>,
    mut next_tab: ResMut<NextState<SettingsTab>>,
) {
    let any_nav = [Up, Down, Left, Right, TabNext, TabPrevious, Confirm, Back]
        .iter()
        .any(|action| state.just_pressed(action));

    if !any_nav {
        return;
    } 

    if state.just_pressed(&Back) && tab_state.is_some() {
        next_menu.set(MenuState::Main);
        return;
    }
    if let Some(current) = &tab_state {
        if state.just_pressed(&TabNext) {
            next_tab.set(current.get().next());
            return;
        }
        if state.just_pressed(&TabPrevious) {
            next_tab.set(current.get().prev());
            return;
        }
    }

    let focus = &mut nav.manual_directional_navigation.focus;
    let on_button = focus.get().is_some_and(|e| buttons.contains(e));
    if !on_button {
        focus.clear();
        if let Ok(first) = tab.navigate(focus, NavAction::Next) {
            focus.set(first, FocusCause::Navigated);
        }
        return;
    }

    let octant_option = [
        (Up, CompassOctant::North),
        (Down, CompassOctant::South),
        (Left, CompassOctant::West),
        (Right, CompassOctant::East),
    ]
    .into_iter()
    .find(|(action, _)| state.just_pressed(action))
    .map(|(_, octant)| octant);

    if let Some(octant) = octant_option {
        let _ = nav.navigate(octant);
    }

    let tab_action = if state.just_pressed(&TabNext) {
        Some(NavAction::Next)
    } else if state.just_pressed(&TabPrevious) {
        Some(NavAction::Previous)
    } else {
        None
    };
    let focus = &mut nav.manual_directional_navigation.focus;
    if let Some(action) = tab_action
        && let Ok(next) = tab.navigate(focus, action)
    {
        focus.set(next, FocusCause::Navigated);
    }

    if state.just_pressed(&Confirm)
        && let Some(entity) = focus.get()
    {
        commands.trigger(Activate { entity });
    }
}

pub fn highlight(
    focus: Res<InputFocus>,
    mut buttons: Query<(Entity, &mut BorderColor), With<Button>>,
) {
    for (entity, mut border) in &mut buttons {
        *border = if focus.get() == Some(entity) {
            BorderColor::all(FOCUS_BORDER)
        } else {
            BorderColor::DEFAULT
        };
    }
}

pub fn controller_observer(
    event: On<Activate>,
    buttons: Query<&MenuButton>,
    mut exit: MessageWriter<AppExit>,
    mut next: ResMut<NextState<MenuState>>,
) {
    match buttons.get(event.entity) {
        Ok(MenuButton::Quit) => {
            exit.write(AppExit::Success);
        }
        Ok(MenuButton::Settings) => next.set(MenuState::Settings),
        Ok(other) => info!("{other:?}"),
        Err(_) => {}
    }
}

pub fn hover_observer(
    event: On<Pointer<Over>>,
    buttons: Query<(), With<Button>>,
    mut focus: ResMut<InputFocus>,
) {
    if buttons.contains(event.entity) {
        focus.set(event.entity, FocusCause::Navigated);
    }
}

pub fn unfocus_on_leave(event: On<Pointer<Out>>, mut focus: ResMut<InputFocus>) {
    if focus.get() == Some(event.entity) {
        focus.clear()
    }
}

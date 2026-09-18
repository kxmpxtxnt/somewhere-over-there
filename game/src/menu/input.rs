use bevy::prelude::GamepadButton;
use bevy::prelude::KeyCode;
use bevy::prelude::Reflect;
use leafwing_input_manager::Actionlike;
use leafwing_input_manager::prelude::{GamepadControlDirection, InputMap};
use leafwing_input_manager::user_input::ButtonlikeChord;

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum MenuAction {
    Up,
    Down,
    Left,
    Right,
    Confirm,
    Back,
    TabNext,
    TabPrevious,
    Debug,
}

pub fn input_map() -> InputMap<MenuAction> {
    InputMap::default()
        .with(MenuAction::Up, KeyCode::ArrowUp)
        .with(MenuAction::Down, KeyCode::ArrowDown)
        .with(MenuAction::Left, KeyCode::ArrowLeft)
        .with(MenuAction::Right, KeyCode::ArrowRight)
        .with(MenuAction::Back, KeyCode::Escape)
        .with(MenuAction::TabNext, KeyCode::Tab)
        .with(
            MenuAction::TabPrevious,
            ButtonlikeChord::new([KeyCode::ShiftLeft, KeyCode::Tab]),
        )
        .with(
            MenuAction::Debug,
            ButtonlikeChord::new([KeyCode::ControlLeft, KeyCode::AltLeft, KeyCode::F1]),
        )
        .with(MenuAction::Up, GamepadButton::DPadUp)
        .with(MenuAction::Down, GamepadButton::DPadDown)
        .with(MenuAction::Left, GamepadButton::DPadLeft)
        .with(MenuAction::Right, GamepadButton::DPadRight)
        .with(MenuAction::Confirm, GamepadButton::East)
        .with(MenuAction::Back, GamepadButton::South)
        .with(MenuAction::TabNext, GamepadButton::RightTrigger)
        .with(MenuAction::TabPrevious, GamepadButton::LeftTrigger)
        .with(
            MenuAction::Up,
            GamepadControlDirection::LEFT_UP.threshold(0.5),
        )
        .with(
            MenuAction::Down,
            GamepadControlDirection::LEFT_DOWN.threshold(0.5),
        )
        .with(
            MenuAction::Left,
            GamepadControlDirection::LEFT_LEFT.threshold(0.5),
        )
        .with(
            MenuAction::Right,
            GamepadControlDirection::LEFT_RIGHT.threshold(0.5),
        )
        .with(
            MenuAction::Debug,
            ButtonlikeChord::new([GamepadButton::LeftThumb, GamepadButton::RightThumb]),
        )
}

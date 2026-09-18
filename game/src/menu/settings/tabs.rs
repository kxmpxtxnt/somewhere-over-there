use crate::state::MenuState;
use bevy::prelude::SubStates;

#[derive(Debug, Default, SubStates, Clone, Copy, PartialEq, Eq, Hash)]
#[source(MenuState = MenuState::Settings)]
pub enum SettingsTab {
    #[default]
    Graphics,
    Audio,
    Controls,
}

impl SettingsTab {
    pub const ALL: [Self; 3] = [Self::Graphics, Self::Audio, Self::Controls];

    pub fn label(self) -> &'static str {
        match self {
            Self::Graphics => "Graphics",
            Self::Audio => "Audio",
            Self::Controls => "Controls",
        }
    }

    pub fn next(self) -> Self {
        let i = Self::ALL.iter().position(|t| *t == self).unwrap_or(0);
        Self::ALL[(i + 1) % Self::ALL.len()]
    }

    pub fn prev(self) -> Self {
        let i = Self::ALL.iter().position(|t| *t == self).unwrap_or(0);
        Self::ALL[(i + Self::ALL.len() - 1) % Self::ALL.len()]
    }
}

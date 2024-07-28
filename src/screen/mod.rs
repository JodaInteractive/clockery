//! The game's main screen states and transitions between them.

mod credits;
pub mod leaderboard;
mod loading;
mod playing;
mod splash;
mod title;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.init_state::<Screen>();
    app.enable_state_scoped_entities::<Screen>();

    app.add_plugins((
        splash::plugin,
        loading::plugin,
        title::plugin,
        credits::plugin,
        playing::plugin,
        leaderboard::plugin,
    ));
}

/// The game's main screen states.
#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
pub enum Screen {
    #[default]
    Splash,
    Loading,
    Title,
    Credits,
    Playing,
    Leaderboard,
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum PlayingState {
    Playing,
    GameOver,
    Disabled,
}

//! The screen state for the main game loop.

use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use super::{title::TitleAction, PlayingState, Screen};
use crate::game::{
    assets::{HandleMap, ImageKey, SoundtrackKey},
    audio::{sfx::StopAllLoopingSfx, soundtrack::PlaySoundtrack},
    spawn::level::{Scoresource, SpawnLevel},
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Playing), enter_playing);
    app.add_systems(OnEnter(PlayingState::GameOver), game_over);
    app.add_systems(OnExit(Screen::Playing), exit_playing);
    app.add_systems(OnExit(PlayingState::GameOver), exit_gameover);

    app.add_systems(
        Update,
        return_to_title_screen
            .run_if(in_state(Screen::Playing).and_then(input_just_pressed(KeyCode::Escape))),
    );

    app.insert_state(PlayingState::Playing);
}

fn enter_playing(mut commands: Commands, mut next_state: ResMut<NextState<PlayingState>>) {
    commands.trigger(SpawnLevel);
    commands.trigger(PlaySoundtrack::Key(SoundtrackKey::Gameplay));
    next_state.set(PlayingState::Playing);
}

fn exit_playing(mut commands: Commands, mut next_state: ResMut<NextState<PlayingState>>) {
    commands.trigger(StopAllLoopingSfx);
    next_state.set(PlayingState::Disabled);
}

fn exit_gameover(mut commands: Commands, mut next_state: ResMut<NextState<PlayingState>>) {
    next_state.set(PlayingState::Disabled);
}

fn return_to_title_screen(mut next_screen: ResMut<NextState<Screen>>, mut commands: Commands) {
    commands.trigger(PlaySoundtrack::Key(SoundtrackKey::Menu));
    next_screen.set(Screen::Title);
}

fn game_over(
    mut next_state: ResMut<NextState<PlayingState>>,
    mut commands: Commands,
    scoresource: Res<Scoresource>,
    images: Res<HandleMap<ImageKey>>,
) {
    commands.trigger(StopAllLoopingSfx);
    commands.trigger(PlaySoundtrack::Key(SoundtrackKey::Credits));
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 500.0)),
            sprite: Sprite {
                custom_size: Some(Vec2::new(1280.0, 720.0)),
                color: Color::linear_rgba(0.0, 0.0, 0.0, 0.85),
                ..default()
            },
            ..default()
        },
        StateScoped(PlayingState::GameOver),
        StateScoped(Screen::Playing),
    ));

    commands.spawn((
        TextBundle {
            text: Text::from_section(
                "Game Over",
                TextStyle {
                    font_size: 100.0,
                    color: Color::WHITE,
                    ..default()
                },
            ),
            style: Style {
                justify_self: JustifySelf::Center,
                ..default()
            },
            ..default()
        },
        StateScoped(PlayingState::GameOver),
        StateScoped(Screen::Playing),
    ));

    commands.spawn((
        TextBundle {
            text: Text::from_section(
                "Tickery Tockery ran out of oil!",
                TextStyle {
                    font_size: 30.0,
                    color: Color::WHITE,
                    ..default()
                },
            ),
            style: Style {
                justify_self: JustifySelf::Center,
                margin: UiRect {
                    top: Val::Px(120.0),
                    ..default()
                },
                ..default()
            },
            ..default()
        },
        StateScoped(PlayingState::GameOver),
        StateScoped(Screen::Playing),
    ));

    let score_string = format!("Your final score: {:.0}", scoresource.0);
    commands.spawn((
        TextBundle {
            text: Text::from_section(
                score_string,
                TextStyle {
                    font_size: 60.0,
                    color: Color::WHITE,
                    ..default()
                },
            ),
            style: Style {
                justify_self: JustifySelf::Center,
                margin: UiRect {
                    top: Val::Px(200.0),
                    ..default()
                },
                ..default()
            },
            ..default()
        },
        StateScoped(PlayingState::GameOver),
        StateScoped(Screen::Playing),
    ));

    commands.spawn((
        ButtonBundle {
            style: Style {
                width: Val::Px(213.0),
                height: Val::Px(63.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                align_self: AlignSelf::Center,
                justify_self: JustifySelf::Center,
                margin: UiRect {
                    top: Val::Px(-80.0),
                    ..default()
                },
                ..default()
            },
            image: UiImage {
                texture: images[&ImageKey::StartButton].clone_weak(),
                ..default()
            },
            ..default()
        },
        TitleAction::Menu,
        StateScoped(PlayingState::GameOver),
        StateScoped(Screen::Playing),
    ));

    commands.spawn((
        ButtonBundle {
            style: Style {
                width: Val::Px(213.0),
                height: Val::Px(63.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                align_self: AlignSelf::Center,
                justify_self: JustifySelf::Center,
                margin: UiRect {
                    top: Val::Px(80.0),
                    ..default()
                },
                ..default()
            },
            image: UiImage {
                texture: images[&ImageKey::CreditsButton].clone_weak(),
                ..default()
            },
            ..default()
        },
        TitleAction::Credits,
        StateScoped(PlayingState::GameOver),
        StateScoped(Screen::Playing),
    ));
}

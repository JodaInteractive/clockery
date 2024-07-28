//! A loading screen during which game assets are loaded.
//! This reduces stuttering, especially for audio on WASM.

use bevy::prelude::*;
use bevy_http_client::prelude::{TypedRequest, TypedResponse};

use super::{
    credits::CreditsAction,
    playing::{get_scores, LeaderboardRecord},
    PlayingState, Screen,
};
use crate::{
    game::{
        assets::{HandleMap, ImageKey, SoundtrackKey},
        audio::soundtrack::PlaySoundtrack,
    },
    ui::prelude::*,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Leaderboard), enter_leaderboard);
    app.add_systems(
        Update,
        handle_credits_action.run_if(in_state(Screen::Leaderboard)),
    );

    app.add_systems(
        Update,
        handle_response.run_if(in_state(Screen::Leaderboard)),
    );
}

fn enter_leaderboard(
    mut commands: Commands,
    mut ev_request: EventWriter<TypedRequest<Vec<LeaderboardRecord>>>,
    image_handles: Res<HandleMap<ImageKey>>,
) {
    get_scores(ev_request);

    commands.spawn((
        SpriteBundle {
            texture: image_handles[&ImageKey::TitleBackground].clone_weak(),
            transform: Transform {
                translation: Vec3::new(0.0, -110.0, -100.0),
                ..default()
            },
            sprite: Sprite {
                custom_size: Some(Vec2::new(1280.0, 1280.0)),
                ..default()
            },
            ..default()
        },
        StateScoped(Screen::Leaderboard),
    ));

    commands.spawn((
        TextBundle {
            text: Text::from_section(
                "Leaderboard",
                TextStyle {
                    font_size: 30.0,
                    color: Color::WHITE,
                    ..default()
                },
            ),
            style: Style {
                justify_self: JustifySelf::Center,
                margin: UiRect {
                    top: Val::Px(50.0),
                    ..default()
                },
                ..default()
            },
            ..default()
        },
        StateScoped(Screen::Leaderboard),
    ));

    commands
        .ui_root()
        .insert(StateScoped(Screen::Leaderboard))
        .with_children(|children| {
            children
                .button("Back")
                .insert(CreditsAction::Back)
                .insert(Style {
                    margin: UiRect {
                        top: Val::Px(600.0),
                        ..default()
                    },
                    padding: UiRect {
                        top: Val::Px(10.0),
                        bottom: Val::Px(10.0),
                        left: Val::Px(20.0),
                        right: Val::Px(20.0),
                    },
                    ..default()
                });
        });
}

fn handle_credits_action(
    mut commands: Commands,
    mut next_screen: ResMut<NextState<Screen>>,
    mut button_query: InteractionQuery<&CreditsAction>,
) {
    for (interaction, action) in &mut button_query {
        if matches!(interaction, Interaction::Pressed) {
            match action {
                CreditsAction::Back => {
                    next_screen.set(Screen::Title);
                    commands.trigger(PlaySoundtrack::Key(SoundtrackKey::Menu));
                }
            }
        }
    }
}

fn handle_response(
    mut commands: Commands,
    mut ev_response: EventReader<TypedResponse<Vec<LeaderboardRecord>>>,
) {
    for res in ev_response.read() {
        for (i, score) in res.iter().take(25).enumerate() {
            let text = format!("{}. {} - {:.2}", i + 1, score.name, score.score);
            commands.spawn((
                TextBundle {
                    text: Text::from_section(
                        text,
                        TextStyle {
                            font_size: 20.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    ),
                    style: Style {
                        justify_self: JustifySelf::Center,
                        margin: UiRect {
                            top: Val::Px(90.0 + (i as f32 * 20.0)),
                            ..default()
                        },
                        ..default()
                    },
                    ..default()
                },
                StateScoped(PlayingState::GameOver),
                StateScoped(Screen::Leaderboard),
            ));
        }
    }
}

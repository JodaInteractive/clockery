use bevy::{
    input::{
        common_conditions::input_just_pressed,
        keyboard::{self, KeyboardInput},
        ButtonState,
    },
    prelude::*,
};
use bevy_http_client::{
    prelude::{HttpTypedRequestTrait, TypedRequest, TypedResponse},
    HttpClient,
};
use serde::{Deserialize, Serialize};

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

    app.insert_resource(NameResource(None));

    app.add_systems(Update, name_input.run_if(in_state(PlayingState::GameOver)));

    app.insert_state(PlayingState::Playing);

    app.register_request_type::<Vec<LeaderboardRecord>>();
    app.register_request_type::<LeaderboardBody>();
    app.add_systems(Update, handle_response);
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

#[derive(Serialize, Deserialize, Debug)]
pub struct LeaderboardBody {
    name: String,
    score: f32,
}

#[derive(Component)]
struct NameInput;

#[derive(Resource)]
pub struct NameResource(pub Option<String>);

#[derive(Component)]
pub struct SubmitScoreButton;

fn game_over(
    mut commands: Commands,
    scoresource: Res<Scoresource>,
    images: Res<HandleMap<ImageKey>>,
    name: Res<NameResource>,
    mut ev_request: EventWriter<TypedRequest<Vec<LeaderboardRecord>>>,
) {
    get_scores(ev_request);

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
                    top: Val::Px(110.0),
                    ..default()
                },
                ..default()
            },
            ..default()
        },
        StateScoped(PlayingState::GameOver),
        StateScoped(Screen::Playing),
    ));

    let score_string = format!("Your final score: {:.2}", scoresource.0);
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
                    top: Val::Px(500.0),
                    ..default()
                },
                ..default()
            },
            ..default()
        },
        StateScoped(PlayingState::GameOver),
        StateScoped(Screen::Playing),
    ));

    let t = if name.0.is_none() {
        "Type your name".to_string()
    } else {
        name.0.clone().unwrap()
    };
    commands.spawn((
        TextBundle {
            text: Text::from_section(
                t,
                TextStyle {
                    font_size: 30.0,
                    color: Color::WHITE,
                    ..default()
                },
            ),
            style: Style {
                justify_self: JustifySelf::Center,
                margin: UiRect {
                    top: Val::Px(450.0),
                    left: Val::Px(-250.0),
                    ..default()
                },
                ..default()
            },
            ..default()
        },
        StateScoped(PlayingState::GameOver),
        StateScoped(Screen::Playing),
        NameInput,
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
                    top: Val::Px(200.0),
                    left: Val::Px(250.0),
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
        TitleAction::SubmitScore,
        StateScoped(PlayingState::GameOver),
        StateScoped(Screen::Playing),
        SubmitScoreButton,
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
                    top: Val::Px(500.0),
                    left: Val::Px(-250.0),
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
                    top: Val::Px(500.0),
                    right: Val::Px(-250.0),
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
                    top: Val::Px(150.0),
                    ..default()
                },
                ..default()
            },
            ..default()
        },
        StateScoped(PlayingState::GameOver),
        StateScoped(Screen::Playing),
    ));
}

#[derive(Serialize, Deserialize, Debug)]
struct LeaderboardRecord {
    id: String,
    name: String,
    score: f32,
}

pub fn submit_score(
    name: String,
    score: f32,
    mut ev_request: &mut EventWriter<TypedRequest<LeaderboardBody>>,
) {
    let body = LeaderboardBody { name, score };
    ev_request.send(
        HttpClient::new()
            .post("https://sr5t5qmb4c.execute-api.us-east-1.amazonaws.com/prod/leaderboard")
            .json(&body)
            .with_type::<LeaderboardBody>(),
    );
}

fn get_scores(mut ev_request: EventWriter<TypedRequest<Vec<LeaderboardRecord>>>) {
    ev_request.send(
        HttpClient::new()
            .get("https://sr5t5qmb4c.execute-api.us-east-1.amazonaws.com/prod/leaderboard")
            .with_type::<Vec<LeaderboardRecord>>(),
    );
}

fn handle_response(
    mut commands: Commands,
    mut ev_response: EventReader<TypedResponse<Vec<LeaderboardRecord>>>,
) {
    for res in ev_response.read() {
        for (i, score) in res.iter().take(10).enumerate() {
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
                            top: Val::Px(190.0 + (i as f32 * 20.0)),
                            ..default()
                        },
                        ..default()
                    },
                    ..default()
                },
                StateScoped(PlayingState::GameOver),
                StateScoped(Screen::Playing),
            ));
        }
    }
}

fn name_input(
    mut evr_kbd: EventReader<KeyboardInput>,
    mut query: Query<(&NameInput, &mut Text)>,
    mut name: ResMut<NameResource>,
) {
    for ev in evr_kbd.read() {
        if ev.state == ButtonState::Released {
            continue;
        }

        if name.0.is_none() {
            name.0 = Some("".to_string());
        }

        match &ev.logical_key {
            keyboard::Key::Backspace => {
                let mut n = name.0.clone().unwrap();
                n.pop();
                if n.is_empty() {
                    name.0 = None;
                } else {
                    name.0 = Some(n);
                }
            }
            keyboard::Key::Character(c) => {
                if c.chars().any(|c| c.is_control()) {
                    continue;
                }
                let mut n = name.0.clone().unwrap();
                n.push_str(c);
                name.0 = Some(n);
            }
            _ => {}
        }
    }

    for (_, mut text) in query.iter_mut() {
        if name.0.is_none() {
            text.sections[0].value = "Type your name".to_string();
        } else {
            text.sections[0].value = name.0.clone().unwrap();
        }
    }
}

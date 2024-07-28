//! The title screen that appears when the game starts.

use bevy::{prelude::*, window::PrimaryWindow};
use bevy_http_client::prelude::TypedRequest;

use super::{
    playing::{submit_score, LeaderboardBody, NameResource, SubmitScoreButton},
    PlayingState, Screen,
};
use crate::{
    game::{
        assets::{FontKey, HandleMap, ImageKey},
        spawn::level::Scoresource,
    },
    ui::prelude::*,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Title), enter_title);

    app.register_type::<TitleAction>();
    app.add_systems(Update, handle_title_action.run_if(in_state(Screen::Title)));
    app.add_systems(
        Update,
        handle_title_action.run_if(in_state(PlayingState::GameOver)),
    );
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Reflect)]
#[reflect(Component)]
pub enum TitleAction {
    Play,
    Credits,
    /// Exit doesn't work well with embedded applications.
    #[cfg(not(target_family = "wasm"))]
    Exit,
    Menu,
    SubmitScore,
    Leaderboard,
}

#[derive(Component)]
struct TitleHand;

#[derive(Component)]
struct Gear;

fn enter_title(
    mut commands: Commands,
    images: Res<HandleMap<ImageKey>>,
    fonts: Res<HandleMap<FontKey>>,
) {
    commands.spawn((
        TextBundle {
            text: Text {
                sections: vec![TextSection {
                    value: "Tickery Tockery".to_string(),
                    style: TextStyle {
                        font: fonts[&FontKey::Arrancar].clone_weak(),
                        font_size: 100.0,
                        color: Color::linear_rgb(1.0, 0.75, 0.1),
                    },
                }],
                ..default()
            },
            style: Style {
                justify_self: JustifySelf::Center,
                margin: UiRect {
                    top: Val::Px(40.0),
                    ..default()
                },
                ..default()
            },
            ..default()
        },
        StateScoped(Screen::Title),
    ));

    commands.spawn((
        TextBundle {
            text: Text {
                sections: vec![TextSection {
                    value: "Panic at the Clockery".to_string(),
                    style: TextStyle {
                        font: fonts[&FontKey::Guavine].clone_weak(),
                        font_size: 60.0,
                        color: Color::linear_rgb(0.9, 0.65, 0.05),
                    },
                }],
                ..default()
            },
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
        StateScoped(Screen::Title),
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
        TitleAction::Play,
        StateScoped(Screen::Title),
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
        StateScoped(Screen::Title),
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
                    top: Val::Px(240.0),
                    ..default()
                },
                ..default()
            },
            image: UiImage {
                texture: images[&ImageKey::LeaderboardButton].clone_weak(),
                ..default()
            },
            ..default()
        },
        TitleAction::Leaderboard,
        StateScoped(Screen::Title),
    ));

    commands.spawn((
        SpriteBundle {
            texture: images[&ImageKey::TitleBackground].clone_weak(),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, -10.0)),
            sprite: Sprite {
                custom_size: Some(Vec2::new(1280.0, 720.0)),
                ..default()
            },
            ..default()
        },
        StateScoped(Screen::Title),
    ));

    commands.spawn((
        SpriteBundle {
            texture: images[&ImageKey::TitleHand].clone_weak(),
            transform: Transform::from_translation(Vec3::new(-370.0, -50.0, 0.0)),
            sprite: Sprite {
                custom_size: Some(Vec2::new(350.0, 350.0)),
                ..default()
            },
            ..default()
        },
        TitleHand,
        StateScoped(Screen::Title),
    ));

    commands.spawn((
        SpriteBundle {
            texture: images[&ImageKey::Gear].clone_weak(),
            transform: Transform::from_translation(Vec3::new(-700.0, -420.0, 0.0)),
            sprite: Sprite {
                custom_size: Some(Vec2::new(700.0, 700.0)),
                ..default()
            },
            ..default()
        },
        Gear,
        StateScoped(Screen::Title),
    ));

    // commands
    //     .ui_root()
    //     .insert(StateScoped(Screen::Title))
    //     .with_children(|children| {
    //         // children.button("Play").insert(TitleAction::Play);
    //         // children.button("Credits").insert(TitleAction::Credits);

    //         // #[cfg(not(target_family = "wasm"))]
    //         // children.button("Exit").insert(TitleAction::Exit);
    //     });
}

fn handle_title_action(
    mut commands: Commands,
    time: Res<Time>,
    mut next_screen: ResMut<NextState<Screen>>,
    mut button_query: InteractionQuery<&TitleAction>,
    mut hand_query: Query<&mut Transform, With<TitleHand>>,
    #[cfg(not(target_family = "wasm"))] mut app_exit: EventWriter<AppExit>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    mut gears: Query<&mut Transform, (With<Gear>, Without<TitleHand>)>,
    name: Res<NameResource>,
    scoresource: Res<Scoresource>,
    submit_score_button: Query<Entity, With<SubmitScoreButton>>,
    mut ev_request: EventWriter<TypedRequest<LeaderboardBody>>,
) {
    for mut gear in gears.iter_mut() {
        gear.rotate_z(0.1 * time.delta_seconds());
    }

    let hand = hand_query.get_single_mut();
    if hand.is_ok() {
        let mut hand = hand.unwrap();
        let window = q_window.get_single();
        if window.is_ok() {
            let camera = q_camera.get_single();
            if camera.is_ok() {
                let (camera, camera_transform) = camera.unwrap();
                let pos = window
                    .unwrap()
                    .cursor_position()
                    .and_then(|cursor| {
                        camera
                            .viewport_to_world(camera_transform, cursor)
                            .map(|ray| ray.origin.truncate())
                    })
                    .unwrap_or(Vec2::ZERO);
                let to_cursor = (pos - hand.translation.xy()).normalize();
                let angle = Quat::from_rotation_arc(Vec3::Y, to_cursor.extend(0.));
                hand.rotation = angle;
            }
        }
    }

    for (interaction, action) in &mut button_query {
        if matches!(interaction, Interaction::Pressed) {
            match action {
                TitleAction::Play => {
                    next_screen.set(Screen::Playing);
                }
                TitleAction::Credits => next_screen.set(Screen::Credits),
                TitleAction::Menu => next_screen.set(Screen::Title),

                #[cfg(not(target_family = "wasm"))]
                TitleAction::Exit => {
                    app_exit.send(AppExit::Success);
                }
                TitleAction::SubmitScore => {
                    let button = submit_score_button.get_single();
                    if button.is_ok() {
                        let e = commands.get_entity(button.unwrap());
                        if let Some(e) = e {
                            e.despawn_recursive();
                        }
                    }
                    submit_score(name.0.clone().unwrap(), scoresource.0, &mut ev_request);
                }
                TitleAction::Leaderboard => {
                    next_screen.set(Screen::Leaderboard);
                }
            }
        }
    }
}

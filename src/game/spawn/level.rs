//! Spawn the main level by triggering other observers.

use bevy::prelude::*;

use crate::{
    game::assets::{HandleMap, ImageKey},
    screen::{PlayingState, Screen},
};

use super::clock::{Positions, SpawnClock, SpawnMainClock};
use super::player::SpawnPlayer;

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_level);
    app.observe(spawn_table);
    app.observe(spawn_oil);
    app.observe(spawn_score);
    app.observe(spawn_clock_table);
    app.observe(spawn_oil_table);
    app.observe(spawn_background);

    app.insert_resource(Scoresource(0.0));
}

#[derive(Event, Debug)]
pub struct SpawnLevel;

#[derive(Event, Debug)]
pub struct SpawnTable;

#[derive(Event, Debug)]
pub struct SpawnOil;

#[derive(Event, Debug)]
pub struct SpawnScore;

#[derive(Event, Debug)]
pub struct SpawnClockTable;

#[derive(Event, Debug)]
pub struct SpawnOilTable;

#[derive(Component)]
pub struct Score(pub f32);

#[derive(Resource)]
pub struct Scoresource(pub f32);

#[derive(Event, Debug)]
pub struct SpawnBackground;

fn spawn_background(
    _trigger: Trigger<SpawnBackground>,
    mut commands: Commands,
    image_handles: Res<HandleMap<ImageKey>>,
) {
    commands.spawn((
        SpriteBundle {
            texture: image_handles[&ImageKey::Background].clone_weak(),
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
        StateScoped(Screen::Playing),
    ));
}

fn spawn_level(
    _trigger: Trigger<SpawnLevel>,
    mut commands: Commands,
    mut scoresource: ResMut<Scoresource>,
) {
    commands.trigger(SpawnBackground);
    commands.trigger(SpawnPlayer);
    commands.trigger(SpawnTable);
    commands.trigger(SpawnMainClock);
    commands.trigger(SpawnClock);
    commands.trigger(SpawnOil);
    commands.trigger(SpawnScore);
    commands.trigger(SpawnClockTable);
    commands.trigger(SpawnOilTable);
    scoresource.0 = 0.0;
}

fn spawn_table(
    _trigger: Trigger<SpawnTable>,
    mut commands: Commands,
    image_handles: Res<HandleMap<ImageKey>>,
) {
    commands.spawn((
        Name::new("Table"),
        SpriteBundle {
            texture: image_handles[&ImageKey::Table].clone_weak(),
            transform: Transform {
                translation: Vec3::new(0.0, -320.0, 10.0),
                ..Default::default()
            },
            sprite: Sprite {
                custom_size: Some(Vec2::new(768.0, 256.0)),
                ..default()
            },
            ..default()
        },
        StateScoped(Screen::Playing),
    ));
}

fn spawn_oil(
    _trigger: Trigger<SpawnOil>,
    positions: Res<Positions>,
    mut commands: Commands,
    image_handles: Res<HandleMap<ImageKey>>,
) {
    commands.spawn((
        Name::new("Oil"),
        SpriteBundle {
            texture: image_handles[&ImageKey::OilCan].clone_weak(),
            transform: Transform {
                translation: positions.oil_can.extend(10.0),
                ..default()
            },
            sprite: Sprite {
                custom_size: Some(Vec2::new(128.0, 128.0)),
                ..default()
            },
            ..default()
        },
        StateScoped(Screen::Playing),
    ));
}

fn spawn_score(_trigger: Trigger<SpawnScore>, mut commands: Commands) {
    commands.spawn((
        Name::new("Score"),
        TextBundle {
            text: Text {
                sections: vec![TextSection {
                    value: "0".to_string(),
                    style: TextStyle {
                        font_size: 50.0,
                        color: Color::WHITE,
                        ..default()
                    },
                }],
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(-500.0, -300.0, 10.0),
                ..default()
            },
            ..default()
        },
        Score(0.0),
        StateScoped(Screen::Playing),
    ));
}

fn spawn_clock_table(
    _trigger: Trigger<SpawnClockTable>,
    positions: Res<Positions>,
    mut commands: Commands,
    image_handles: Res<HandleMap<ImageKey>>,
) {
    let translation = Vec3::new(positions.clock_spawn.x, -340.0, 10.0);
    commands.spawn((
        Name::new("ClockTable"),
        SpriteBundle {
            texture: image_handles[&ImageKey::ClockTable].clone_weak(),
            transform: Transform {
                translation,
                ..default()
            },
            sprite: Sprite {
                custom_size: Some(Vec2::new(256.0, 256.0)),
                ..default()
            },
            ..default()
        },
        StateScoped(Screen::Playing),
    ));
}

fn spawn_oil_table(
    _trigger: Trigger<SpawnOilTable>,
    positions: Res<Positions>,
    mut commands: Commands,
    image_handles: Res<HandleMap<ImageKey>>,
) {
    let translation = Vec3::new(positions.oil_can.x, -340.0, 10.0);
    commands.spawn((
        Name::new("OilTable"),
        SpriteBundle {
            texture: image_handles[&ImageKey::OilTable].clone_weak(),
            transform: Transform {
                translation,
                ..default()
            },
            sprite: Sprite {
                custom_size: Some(Vec2::new(256.0, 256.0)),
                ..default()
            },
            ..default()
        },
        StateScoped(Screen::Playing),
    ));
}

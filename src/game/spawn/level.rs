//! Spawn the main level by triggering other observers.

use bevy::prelude::*;

use crate::{
    game::assets::{HandleMap, ImageKey},
    screen::Screen,
};

use super::clock::{Positions, SpawnClock, SpawnMainClock};
use super::player::SpawnPlayer;

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_level);
    app.observe(spawn_table);
    app.observe(spawn_oil);
    app.observe(spawn_score);
}

#[derive(Event, Debug)]
pub struct SpawnLevel;

#[derive(Event, Debug)]
pub struct SpawnTable;

#[derive(Event, Debug)]
pub struct SpawnOil;

#[derive(Event, Debug)]
pub struct SpawnScore;

#[derive(Component)]
pub struct Score(pub f32);

fn spawn_level(_trigger: Trigger<SpawnLevel>, mut commands: Commands) {
    commands.trigger(SpawnPlayer);
    commands.trigger(SpawnTable);
    commands.trigger(SpawnMainClock);
    commands.trigger(SpawnClock);
    commands.trigger(SpawnClock);
    commands.trigger(SpawnClock);
    commands.trigger(SpawnClock);
    commands.trigger(SpawnClock);
    commands.trigger(SpawnOil);
    commands.trigger(SpawnScore);
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

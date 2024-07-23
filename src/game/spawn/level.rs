//! Spawn the main level by triggering other observers.

use bevy::prelude::*;

use crate::{
    game::assets::{HandleMap, ImageKey},
    screen::Screen,
};

use super::clock::{SpawnClock, SpawnMainClock};
use super::player::SpawnPlayer;

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_level);
    app.observe(spawn_table);
}

#[derive(Event, Debug)]
pub struct SpawnLevel;

#[derive(Event, Debug)]
pub struct SpawnTable;

fn spawn_level(_trigger: Trigger<SpawnLevel>, mut commands: Commands) {
    commands.trigger(SpawnPlayer);
    commands.trigger(SpawnTable);
    commands.trigger(SpawnMainClock);
    commands.trigger(SpawnClock);
    commands.trigger(SpawnClock);
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
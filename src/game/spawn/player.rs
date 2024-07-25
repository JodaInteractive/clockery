use bevy::prelude::*;

use crate::{
    game::{
        assets::{HandleMap, ImageKey},
        movement::MovementController,
    },
    screen::Screen,
};

use super::clock::ClockController;

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_player);
    app.register_type::<Player>();
}

#[derive(Event, Debug)]
pub struct SpawnPlayer;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Player;

fn spawn_player(
    _trigger: Trigger<SpawnPlayer>,
    mut commands: Commands,
    image_handles: Res<HandleMap<ImageKey>>,
) {
    commands.spawn((
        Name::new("Player"),
        Player,
        SpriteBundle {
            texture: image_handles[&ImageKey::Tockery].clone_weak(),
            transform: Transform {
                scale: Vec3::splat(1.0),
                translation: Vec3::new(-330.0, -120.0, 0.0),
                ..default()
            },
            sprite: Sprite {
                custom_size: Some(Vec2::new(168.3, 383.35)),
                ..default()
            },
            ..default()
        },
        MovementController::default(),
        ClockController {
            index: 1,
            ..default()
        },
        StateScoped(Screen::Playing),
    ));
}

use bevy::prelude::*;

use crate::{
    game::{
        assets::{HandleMap, ImageKey},
        movement::MovementController,
    },
    screen::Screen,
};

use super::clock::{ClockController, Positions};

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_player);
    app.register_type::<Player>();
    app.add_systems(Update, oil_leak.run_if(in_state(Screen::Playing)));
    app.add_systems(Update, oil_drink.run_if(in_state(Screen::Playing)));
}

#[derive(Event, Debug)]
pub struct SpawnPlayer;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Player;

#[derive(Component)]
pub struct OilMeter;

fn oil_drink(
    time: Res<Time>,
    mut control_query: Query<&mut ClockController>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if !input.pressed(KeyCode::Space) {
        return;
    }

    let mut controller = control_query.single_mut();

    if controller.index == 6 {
        controller.oil_level += time.delta_seconds() * 10.0;
        if controller.oil_level > 100.0 {
            controller.oil_level = 100.0;
        }
    }
}

fn oil_leak(
    mut controller: Query<&mut ClockController, With<Player>>,
    mut query: Query<(&mut Handle<Image>, &mut Sprite), With<OilMeter>>,
    time: Res<Time>,
    images: Res<HandleMap<ImageKey>>,
) {
    let mut controller = controller.single_mut();
    if controller.oil_level <= 0.0 {
        println!("Game over!");
        return;
    }

    controller.oil_level -= time.delta_seconds() * 1.6;

    let (mut image, mut sprite) = query.single_mut();
    match controller.oil_level {
        5.0..=15.0 => {
            *image = images[&ImageKey::Oil10].clone_weak();
        }
        15.0..=25.0 => {
            *image = images[&ImageKey::Oil20].clone_weak();
        }
        25.0..=35.0 => {
            *image = images[&ImageKey::Oil30].clone_weak();
        }
        35.0..=45.0 => {
            *image = images[&ImageKey::Oil40].clone_weak();
        }
        45.0..=55.0 => {
            *image = images[&ImageKey::Oil50].clone_weak();
        }
        55.0..=65.0 => {
            *image = images[&ImageKey::Oil60].clone_weak();
        }
        65.0..=75.0 => {
            *image = images[&ImageKey::Oil70].clone_weak();
        }
        75.0..=85.0 => {
            *image = images[&ImageKey::Oil80].clone_weak();
        }
        85.0..=95.0 => {
            *image = images[&ImageKey::Oil90].clone_weak();
        }
        95.0..=100.0 => {
            *image = images[&ImageKey::OilFull].clone_weak();
        }
        _ => {
            // *image = images[&ImageKey::OilEmpty].clone_weak();
            sprite.custom_size = Some(Vec2::new(1.0, 1.0));
        }
    }
}

fn spawn_player(
    _trigger: Trigger<SpawnPlayer>,
    mut commands: Commands,
    image_handles: Res<HandleMap<ImageKey>>,
) {
    commands
        .spawn((
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
                oil_level: 100.0,
                ..default()
            },
            StateScoped(Screen::Playing),
        ))
        .with_children(|parent| {
            parent.spawn((
                SpriteBundle {
                    texture: image_handles[&ImageKey::OilFull].clone_weak(),
                    transform: Transform {
                        translation: Vec3::new(5.0, 140.0, -1.0),
                        ..default()
                    },
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(64.0, 100.0)),
                        ..default()
                    },
                    ..default()
                },
                OilMeter,
            ));
        });
}

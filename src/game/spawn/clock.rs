use bevy::{prelude::*, sprite::Anchor};

use crate::{
    game::{
        assets::{HandleMap, ImageKey, SfxKey},
        audio::sfx::{PlayLoopingSfx, PlaySfx, StopLoopingSfx},
    },
    screen::{PlayingState, Screen},
    AppSet,
};

use super::level::{Score, Scoresource};

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_interact_clock);
    app.observe(spawn_main_clock);
    app.add_systems(
        FixedUpdate,
        (tick_clocks, score_clocks)
            .chain()
            .in_set(AppSet::FixedUpdate)
            .run_if(in_state(Screen::Playing))
            .run_if(in_state(PlayingState::Playing)),
    );
    app.add_systems(
        Update,
        record_clock_controller
            .in_set(AppSet::RecordInput)
            .run_if(in_state(Screen::Playing))
            .run_if(in_state(PlayingState::Playing)),
    );
    app.add_systems(
        FixedUpdate,
        apply_clock_control
            .in_set(AppSet::Update)
            .run_if(in_state(Screen::Playing))
            .run_if(in_state(PlayingState::Playing)),
    );
    app.insert_resource(Positions {
        clock_spawn: Vec2::new(-550.0, -185.0),
        clock_1: Vec2::new(-330.0, -220.0),
        clock_2: Vec2::new(-180.0, -220.0),
        clock_3: Vec2::new(-30.0, -220.0),
        clock_4: Vec2::new(120.0, -220.0),
        clock_5: Vec2::new(270.0, -220.0),
        oil_can: Vec2::new(550.0, -200.0),
    });

    app.insert_resource(Clocks {
        clocks: vec![
            ClockData {
                time_left: 0.0,
                audio: SfxKey::Ticking2,
            },
            ClockData {
                time_left: 0.0,
                audio: SfxKey::Ticking3,
            },
            ClockData {
                time_left: 0.0,
                audio: SfxKey::Ticking4,
            },
            ClockData {
                time_left: 0.0,
                audio: SfxKey::Ticking5,
            },
            ClockData {
                time_left: 0.0,
                audio: SfxKey::Ticking6,
            },
        ],
    });
}

#[derive(Event, Debug)]
pub struct SpawnClock;

#[derive(Event, Debug)]
pub struct SpawnMainClock;

#[derive(Component)]
pub struct Clock {
    pub is_main: bool,
    pub time_left: f32,
    pub audio: SfxKey,
}

#[derive(Component)]
pub enum ClockHandType {
    Hour,
    Minute,
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct ClockController {
    pub held_clock: Option<Entity>,
    pub index: usize,
    pub setting: bool,
    pub time_setting: f32,
    pub winding: bool,
    pub time_winding: f32,
    pub direction: Vec2,
    pub oil_level: f32,
    pub oil_leak: f32,
}

#[derive(Resource)]
pub struct Positions {
    pub clock_spawn: Vec2,
    pub clock_1: Vec2,
    pub clock_2: Vec2,
    pub clock_3: Vec2,
    pub clock_4: Vec2,
    pub clock_5: Vec2,
    pub oil_can: Vec2,
}

#[derive(Component)]
pub struct Interactable;

#[derive(Resource)]
pub struct Clocks {
    clocks: Vec<ClockData>,
}

pub struct ClockData {
    pub time_left: f32,
    pub audio: SfxKey,
}

fn record_clock_controller(
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
    mut controller_query: Query<&mut ClockController>,
) {
    for mut controller in &mut controller_query {
        if input.pressed(KeyCode::KeyS) || input.pressed(KeyCode::ArrowDown) {
            if controller.setting {
                controller.time_setting += time.delta_seconds();
            } else {
                controller.setting = true;
                controller.time_setting = 0.0;
            }
        } else {
            controller.setting = false;
            controller.time_setting = 0.0;
        }

        if input.pressed(KeyCode::KeyW) || input.pressed(KeyCode::ArrowUp) {
            if controller.winding {
                controller.time_winding += time.delta_seconds();
            } else {
                controller.winding = true;
                controller.time_winding = 0.0;
            }
        } else {
            controller.winding = false;
            controller.time_winding = 0.0;
        }
    }
}

fn apply_clock_control(
    time: Res<Time>,
    mut commands: Commands,
    mut control_query: Query<&mut ClockController, Without<Interactable>>,
    mut clocks: Query<(Entity, &mut Clock, &Transform, &Children), With<Interactable>>,
    mut q_child: Query<
        (&mut Transform, &ClockHandType),
        (Without<Interactable>, Without<ClockController>),
    >,
) {
    let result = control_query.get_single_mut();
    if result.is_err() {
        return;
    }
    let mut controller = result.unwrap();
    if controller.held_clock.is_none() {
        return;
    }

    if controller.winding {
        controller.time_winding += time.delta_seconds();
        controller.time_winding = controller.time_winding.min(3.0);
    }

    if controller.setting {
        controller.time_setting += time.delta_seconds();
        controller.time_setting = controller.time_setting.min(3.0);
    }

    let children = clocks
        .iter_mut()
        .find(|t| controller.held_clock.is_some() && t.0 == controller.held_clock.unwrap());
    if children.is_none() {
        return;
    }
    let mut children = children.unwrap();

    if controller.winding {
        let zero = children.1.time_left == 0.0;
        if zero {
            commands.trigger(PlayLoopingSfx::Key(children.1.audio));
        }
        children.1.time_left += time.delta_seconds() * 6.0;
        commands.trigger(PlayLoopingSfx::Key(SfxKey::Setting1));
    } else {
        commands.trigger(StopLoopingSfx::Key(SfxKey::Setting1));
    }

    if controller.setting {
        for &child in children.3.iter() {
            let child_result = q_child.get_mut(child);

            match controller.time_setting {
                0.0..=0.7 => commands.trigger(PlayLoopingSfx::Key(SfxKey::Setting1)),
                0.7..=1.4 => {
                    commands.trigger(StopLoopingSfx::Key(SfxKey::Setting1));
                    commands.trigger(PlayLoopingSfx::Key(SfxKey::Setting2))
                }
                1.4..=2.1 => {
                    commands.trigger(StopLoopingSfx::Key(SfxKey::Setting2));
                    commands.trigger(PlayLoopingSfx::Key(SfxKey::Setting3))
                }
                2.1..=3.0 => {
                    commands.trigger(StopLoopingSfx::Key(SfxKey::Setting3));
                    commands.trigger(PlayLoopingSfx::Key(SfxKey::Setting4))
                }
                _ => {}
            }

            if let Ok((mut transform, hand_type)) = child_result {
                match hand_type {
                    ClockHandType::Hour => {
                        transform.rotate_z(
                            time.delta_seconds() * -0.008726646 * 100.0 * controller.time_setting,
                        );
                    }
                    ClockHandType::Minute => {
                        transform.rotate_z(
                            time.delta_seconds() * -0.1047198 * 100.0 * controller.time_setting,
                        );
                    }
                }
            }
        }
    } else {
        // commands.trigger(StopLoopingSfx::Key(SfxKey::Setting1));
        commands.trigger(StopLoopingSfx::Key(SfxKey::Setting2));
        commands.trigger(StopLoopingSfx::Key(SfxKey::Setting3));
        commands.trigger(StopLoopingSfx::Key(SfxKey::Setting4));
        commands.trigger(StopLoopingSfx::Key(SfxKey::Setting5));
        commands.trigger(StopLoopingSfx::Key(SfxKey::Setting6));
    }
}

fn tick_clocks(
    mut commands: Commands,
    time: Res<Time>,
    mut q_parent: Query<(&mut Clock, &Children)>,
    mut q_child: Query<(&mut Transform, &ClockHandType), Without<Clock>>,
) {
    let hour_speed = time.delta_seconds() * -0.008726646 * 2.0;
    let minute_speed = time.delta_seconds() * -0.1047198 * 2.0;
    for (mut clock, children) in q_parent.iter_mut() {
        if !clock.is_main {
            let above = clock.time_left > 0.0;

            clock.time_left -= time.delta_seconds();
            clock.time_left = clock.time_left.max(0.0);

            if above && clock.time_left == 0.0 {
                commands.trigger(StopLoopingSfx::Key(clock.audio));
            }

            if clock.time_left == 0.0 {
                continue;
            }
        }

        for &child in children.iter() {
            let child_result = q_child.get_mut(child);

            if let Ok((mut transform, hand_type)) = child_result {
                match hand_type {
                    ClockHandType::Hour => {
                        transform.rotate_z(hour_speed);
                    }
                    ClockHandType::Minute => {
                        transform.rotate_z(minute_speed);
                    }
                }
            }
        }
    }
}

fn score_clocks(
    mut commands: Commands,
    time: Res<Time>,
    mut score: Query<(&mut Score, &mut Text)>,
    clocks: Query<(&Clock, &Children)>,
    clock_children: Query<(&Transform, &ClockHandType)>,
    mut scoresource: ResMut<Scoresource>,
) {
    let main = clocks.iter().find(|(clock, _)| clock.is_main).unwrap();
    let main_rotations = get_clock_rotations(main.1, &clock_children);
    let (mut score, mut text) = score.single_mut();

    for (clock, children) in clocks.iter() {
        if clock.is_main || clock.time_left <= 0.0 {
            continue;
        }
        score.0 += 1.0 * time.delta_seconds();

        let clock_rotation = get_clock_rotations(children, &clock_children);

        let hour_diff = main_rotations.hour.angle_between(clock_rotation.hour);
        let minute_diff = main_rotations.minute.angle_between(clock_rotation.minute);

        if hour_diff < 0.1 && minute_diff < 0.1 {
            score.0 += 1.0 * time.delta_seconds();
        }
    }

    let clock_count = clocks.iter().count() - 1;

    match clock_count {
        1 => {
            if score.0 > 25.0 {
                commands.trigger(SpawnClock);
            }
        }
        2 => {
            if score.0 > 100.0 {
                commands.trigger(SpawnClock);
            }
        }
        3 => {
            if score.0 > 250.0 {
                commands.trigger(SpawnClock);
            }
        }
        4 => {
            if score.0 > 500.0 {
                commands.trigger(SpawnClock);
            }
        }
        _ => {}
    }
    text.sections[0].value = format!("{:.2}", score.0);
    scoresource.0 = score.0;
}

struct ClockRotations {
    hour: Quat,
    minute: Quat,
}

fn get_clock_rotations(
    clock_children: &Children,
    all_children: &Query<(&Transform, &ClockHandType)>,
) -> ClockRotations {
    let mut hour: Quat = Quat::IDENTITY;
    let mut minute: Quat = Quat::IDENTITY;

    let hour_hand = clock_children.first().unwrap();
    let hour_hand_result = all_children.get(*hour_hand);
    if let Ok((transform, _)) = hour_hand_result {
        hour = transform.rotation.normalize();
    }

    let minute_hand = clock_children.get(1).unwrap();
    let minute_hand_result = all_children.get(*minute_hand);
    if let Ok((transform, _)) = minute_hand_result {
        minute = transform.rotation.normalize();
    }

    ClockRotations { hour, minute }
}

fn spawn_main_clock(
    _trigger: Trigger<SpawnMainClock>,
    mut commands: Commands,
    image_handles: Res<HandleMap<ImageKey>>,
) {
    commands.trigger(PlayLoopingSfx::Key(SfxKey::Ticking1));
    commands
        .spawn((
            Name::new("MainClock"),
            SpriteBundle {
                texture: image_handles[&ImageKey::MainClock].clone_weak(),
                transform: Transform {
                    translation: Vec3::new(0.0, 80.0, -30.0),
                    ..Default::default()
                },
                sprite: Sprite {
                    custom_size: Some(Vec2::new(512.0, 512.0)),
                    ..default()
                },
                ..default()
            },
            Clock {
                is_main: true,
                time_left: 0.0,
                audio: SfxKey::Ticking1,
            },
            StateScoped(Screen::Playing),
        ))
        .with_children(|parent| {
            parent.spawn((
                SpriteBundle {
                    texture: image_handles[&ImageKey::MainClockHour].clone_weak(),
                    transform: Transform {
                        translation: Vec3::new(0.0, 0.0, 1.0),
                        ..default()
                    },
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(360.0, 360.0)),
                        ..default()
                    },
                    ..default()
                },
                ClockHandType::Hour,
            ));

            parent.spawn((
                SpriteBundle {
                    texture: image_handles[&ImageKey::MainClockMinute].clone_weak(),
                    transform: Transform {
                        translation: Vec3::new(0.0, 0.0, 2.0),
                        ..default()
                    },
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(360.0, 360.0)),
                        ..default()
                    },
                    ..default()
                },
                ClockHandType::Minute,
            ));
        });
}

fn spawn_interact_clock(
    _trigger: Trigger<SpawnClock>,
    positions: Res<Positions>,
    clock_data: Res<Clocks>,
    mut commands: Commands,
    image_handles: Res<HandleMap<ImageKey>>,
    clocks: Query<(&Clock, &Transform), With<Interactable>>,
) {
    let r = rand::random::<f32>();
    if r < 0.25 {
        commands.trigger(PlaySfx::Key(SfxKey::ClockSpawn1));
    } else if r < 0.5 {
        commands.trigger(PlaySfx::Key(SfxKey::ClockSpawn2));
    } else if r < 0.75 {
        commands.trigger(PlaySfx::Key(SfxKey::ClockSpawn3));
    } else {
        commands.trigger(PlaySfx::Key(SfxKey::ClockSpawn4));
    }
    let clock_count = clocks.iter().count();
    let translation = positions.clock_spawn;
    let clock_data = &clock_data.clocks[clock_count];

    commands
        .spawn((
            Name::new("Clock"),
            SpriteBundle {
                texture: image_handles[&ImageKey::Clock].clone_weak(),
                transform: Transform {
                    translation: Vec3::new(translation.x, translation.y, 300.0),
                    ..Default::default()
                },
                sprite: Sprite {
                    anchor: Anchor::Custom(Vec2::new(-0.01, 0.0)),
                    custom_size: Some(Vec2::new(128.0, 128.0)),
                    ..default()
                },
                ..default()
            },
            StateScoped(Screen::Playing),
            Clock {
                is_main: false,
                time_left: clock_data.time_left,
                audio: clock_data.audio,
            },
            Interactable,
        ))
        .with_children(|parent| {
            parent.spawn((
                SpriteBundle {
                    texture: image_handles[&ImageKey::ClockHour].clone_weak(),
                    transform: Transform {
                        translation: Vec3::new(0.0, 0.0, 10.0),
                        ..default()
                    },
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(90.0, 90.0)),
                        ..default()
                    },
                    ..default()
                },
                ClockHandType::Hour,
            ));

            parent.spawn((
                SpriteBundle {
                    texture: image_handles[&ImageKey::ClockMinute].clone_weak(),
                    transform: Transform {
                        translation: Vec3::new(0.0, 0.0, 10.0),
                        ..default()
                    },
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(90.0, 90.0)),
                        ..default()
                    },
                    ..default()
                },
                ClockHandType::Minute,
            ));
        });
}

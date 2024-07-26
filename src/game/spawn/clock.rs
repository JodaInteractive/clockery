use bevy::{prelude::*, sprite::Anchor};

use crate::{
    game::assets::{HandleMap, ImageKey},
    screen::Screen,
    AppSet,
};

use super::level::Score;

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_interact_clock);
    app.observe(spawn_main_clock);
    app.add_systems(
        FixedUpdate,
        (tick_clocks, score_clocks)
            .chain()
            .in_set(AppSet::FixedUpdate)
            .run_if(in_state(Screen::Playing)),
    );
    app.add_systems(
        Update,
        record_clock_controller
            .in_set(AppSet::RecordInput)
            .run_if(in_state(Screen::Playing)),
    );
    app.add_systems(
        FixedUpdate,
        apply_clock_control
            .in_set(AppSet::Update)
            .run_if(in_state(Screen::Playing)),
    );
    app.insert_resource(Positions {
        clock_spawn: Vec2::new(-550.0, -220.0),
        clock_1: Vec2::new(-330.0, -190.0),
        clock_2: Vec2::new(-180.0, -220.0),
        clock_3: Vec2::new(-30.0, -220.0),
        clock_4: Vec2::new(120.0, -220.0),
        clock_5: Vec2::new(270.0, -220.0),
        oil_can: Vec2::new(490.0, -220.0),
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
}

#[derive(Component)]
enum ClockHandType {
    Hour,
    Minute,
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct ClockController {
    pub index: usize,
    pub setting: bool,
    pub time_setting: f32,
    pub winding: bool,
    pub time_winding: f32,
    pub direction: Vec2,
    pub oil_level: f32,
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

fn record_clock_controller(
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
    mut controller_query: Query<&mut ClockController>,
) {
    for mut controller in &mut controller_query {
        if input.pressed(KeyCode::Space) {
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

        if input.pressed(KeyCode::KeyW) {
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
    positions: Res<Positions>,
    mut control_query: Query<&mut ClockController, Without<Interactable>>,
    mut clocks: Query<(&mut Clock, &Transform, &Children), With<Interactable>>,
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
    if !controller.winding && !controller.setting {
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

    let position = match controller.index {
        0 => positions.clock_spawn.x,
        1 => positions.clock_1.x,
        2 => positions.clock_2.x,
        3 => positions.clock_3.x,
        4 => positions.clock_4.x,
        5 => positions.clock_5.x,
        6 => positions.oil_can.x,
        _ => panic!("Invalid index"),
    };

    let children = clocks.iter_mut().find(|t| t.1.translation.x == position);
    if children.is_none() {
        return;
    }
    let mut children = children.unwrap();

    if controller.winding {
        children.0.time_left += time.delta_seconds() * 1.0;
    }

    if controller.setting {
        for &child in children.2.iter() {
            let child_result = q_child.get_mut(child);

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
    }
}

fn tick_clocks(
    time: Res<Time>,
    mut q_parent: Query<(&mut Clock, &Children)>,
    mut q_child: Query<(&mut Transform, &ClockHandType), Without<Clock>>,
) {
    let hour_speed = time.delta_seconds() * -0.008726646 * 2.0;
    let minute_speed = time.delta_seconds() * -0.1047198 * 2.0;
    for (mut clock, children) in q_parent.iter_mut() {
        if !clock.is_main {
            clock.time_left -= time.delta_seconds();
            if clock.time_left <= 0.0 {
                clock.time_left = 0.0;
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
    time: Res<Time>,
    mut score: Query<(&mut Score, &mut Text)>,
    clocks: Query<(&Clock, &Children)>,
    clock_children: Query<(&Transform, &ClockHandType)>,
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
            println!("Score! {} {}", hour_diff, minute_diff);
        }
    }

    text.sections[0].value = format!("{:.0}", score.0);
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
    mut commands: Commands,
    image_handles: Res<HandleMap<ImageKey>>,
    clocks: Query<(&Clock, &Transform)>,
) {
    let clock_count = clocks.iter().count();
    let translation = match clock_count {
        1 => positions.clock_1,
        2 => positions.clock_2,
        3 => positions.clock_3,
        4 => positions.clock_4,
        5 => positions.clock_5,
        _ => positions.clock_spawn,
    };

    commands
        .spawn((
            Name::new("Clock"),
            SpriteBundle {
                texture: image_handles[&ImageKey::Clock].clone_weak(),
                transform: Transform {
                    translation: Vec3::new(translation.x, translation.y, 20.0),
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
                time_left: 60.0,
            },
            Interactable,
        ))
        .with_children(|parent| {
            parent.spawn((
                SpriteBundle {
                    texture: image_handles[&ImageKey::ClockHour].clone_weak(),
                    transform: Transform {
                        translation: Vec3::new(0.0, 0.0, 30.0),
                        ..Default::default()
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
                        translation: Vec3::new(0.0, 0.0, 40.0),
                        ..Default::default()
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

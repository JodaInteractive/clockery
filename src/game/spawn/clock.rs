use bevy::{prelude::*, sprite::Anchor};

use crate::{
    game::assets::{HandleMap, ImageKey},
    screen::Screen,
    AppSet,
};

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_clock);
    app.observe(spawn_main_clock);
    app.add_systems(
        FixedUpdate,
        (tick_clocks, score_clocks)
            .chain()
            .in_set(AppSet::FixedUpdate)
            .run_if(in_state(Screen::Playing)),
    );
    app.add_systems(Update, record_clock_controller.in_set(AppSet::RecordInput));
    app.add_systems(FixedUpdate, apply_clock_control.in_set(AppSet::Update));
}

#[derive(Event, Debug)]
pub struct SpawnClock;

#[derive(Event, Debug)]
pub struct SpawnMainClock;

#[derive(Component)]
struct Clock;

#[derive(Component)]
struct MainClock;

#[derive(Component)]
pub struct InteractClock;

#[derive(Component)]
enum ClockHandType {
    Hour,
    Minute,
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct ClockController(pub bool);

fn record_clock_controller(
    input: Res<ButtonInput<KeyCode>>,
    mut controller_query: Query<&mut ClockController>,
) {
    for mut controller in &mut controller_query {
        controller.0 = input.pressed(KeyCode::Space);
    }
}

fn apply_clock_control(
    time: Res<Time>,
    mut control_query: Query<(&ClockController, &Transform), Without<InteractClock>>,
    mut clocks: Query<(Entity, &Transform, &Children), With<InteractClock>>,
    mut q_child: Query<
        (&mut Transform, &ClockHandType),
        (Without<InteractClock>, Without<ClockController>),
    >,
) {
    // if there are no clocks, short-circuit
    let clock_count = clocks.iter().count();
    if clock_count == 1 {
        return;
    }

    // get all clocks and their positions
    let clock_positions = clocks
        .iter()
        .map(|(e, t, c)| (e, t.translation.x, c))
        .collect::<Vec<(Entity, f32, &Children)>>();

    // get the player's current position
    let result = control_query.get_single_mut();
    if result.is_err() {
        return;
    }
    let (controller, transform) = result.unwrap();
    let character_position = transform.translation.x;

    // get the current clock and it's position
    let current_clock = clock_positions
        .iter()
        .enumerate()
        .find(|(_, &x)| x.1 == character_position)
        .unwrap();

    // tick the current clock
    if controller.0 {
        let (_, _, children) = current_clock.1;
        for &child in children.iter() {
            let child_result = q_child.get_mut(child);

            if let Ok((mut transform, hand_type)) = child_result {
                match hand_type {
                    ClockHandType::Hour => {
                        transform.rotate_z(time.delta_seconds() * -0.008726646 * 5.0);
                    }
                    ClockHandType::Minute => {
                        transform.rotate_z(time.delta_seconds() * -0.1047198 * 5.0);
                    }
                }
            }
        }
    }
}

fn tick_clocks(
    time: Res<Time>,
    q_parent: Query<(&Clock, &Children)>,
    mut q_child: Query<(&mut Transform, &ClockHandType)>,
) {
    for (_, children) in q_parent.iter() {
        for &child in children.iter() {
            let child_result = q_child.get_mut(child);

            if let Ok((mut transform, hand_type)) = child_result {
                match hand_type {
                    ClockHandType::Hour => {
                        transform.rotate_z(time.delta_seconds() * -0.008726646);
                    }
                    ClockHandType::Minute => {
                        transform.rotate_z(time.delta_seconds() * -0.1047198);
                    }
                }
            }
        }
    }
}

fn score_clocks(
    main_clock: Query<(&MainClock, &Children)>,
    clocks: Query<(&Clock, &Children), Without<MainClock>>,
    clock_children: Query<(&Transform, &ClockHandType)>,
) {
    let (_, main_hands) = main_clock.single();
    let main_rotations = get_clock_rotations(main_hands, &clock_children);

    for (_, children) in clocks.iter() {
        let clock_rotation = get_clock_rotations(children, &clock_children);

        let hour_diff = main_rotations.hour.angle_between(clock_rotation.hour);
        let minute_diff = main_rotations.minute.angle_between(clock_rotation.minute);

        // println!("Clock diff: {:#?} {:#?}", hour_diff, minute_diff);

        if hour_diff < 0.1 && minute_diff < 0.1 {
            // println!("Score!");
        }
    }
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
                texture: image_handles[&ImageKey::Clock].clone_weak(),
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, -30.0),
                    ..Default::default()
                },
                sprite: Sprite {
                    anchor: Anchor::Custom(Vec2::new(-0.01, 0.0)),
                    custom_size: Some(Vec2::new(512.0, 512.0)),
                    ..default()
                },
                ..default()
            },
            StateScoped(Screen::Playing),
            Clock,
            MainClock,
        ))
        .with_children(|parent| {
            parent.spawn((
                SpriteBundle {
                    texture: image_handles[&ImageKey::ClockHour].clone_weak(),
                    transform: Transform {
                        translation: Vec3::new(0.0, 0.0, 1.0),
                        rotation: Quat::from_rotation_z(3.8),
                        ..default()
                    },
                    sprite: Sprite {
                        anchor: Anchor::Custom(Vec2::new(0.125, 0.125)),
                        custom_size: Some(Vec2::new(160.0, 160.0)),
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
                        translation: Vec3::new(0.0, 0.0, 2.0),
                        rotation: Quat::from_rotation_z(3.8),
                        ..default()
                    },
                    sprite: Sprite {
                        anchor: Anchor::Custom(Vec2::new(0.1875, 0.227)),
                        custom_size: Some(Vec2::new(220.0, 220.0)),
                        ..default()
                    },
                    ..default()
                },
                ClockHandType::Minute,
            ));
        });
}

fn spawn_clock(
    _trigger: Trigger<SpawnClock>,
    mut commands: Commands,
    image_handles: Res<HandleMap<ImageKey>>,
    clocks: Query<(&InteractClock, &Transform)>,
) {
    let clock_count = clocks.iter().count();
    let x = -330.0 + (clock_count as f32 * 150.0);
    let first_clock = clock_count == 0;

    let y = if first_clock { -190.0 } else { -230.0 };
    commands
        .spawn((
            Name::new("Clock"),
            SpriteBundle {
                texture: image_handles[&ImageKey::Clock].clone_weak(),
                transform: Transform {
                    translation: Vec3::new(x, y, 20.0),
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
            Clock,
            InteractClock,
        ))
        .with_children(|parent| {
            parent.spawn((
                SpriteBundle {
                    texture: image_handles[&ImageKey::ClockHour].clone_weak(),
                    transform: Transform {
                        translation: Vec3::new(0.0, 0.0, 30.0),
                        rotation: Quat::from_rotation_z(3.8),
                        ..Default::default()
                    },
                    sprite: Sprite {
                        anchor: Anchor::Custom(Vec2::new(0.125, 0.125)),
                        custom_size: Some(Vec2::new(40.0, 40.0)),
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
                        rotation: Quat::from_rotation_z(3.8),
                        ..Default::default()
                    },
                    sprite: Sprite {
                        anchor: Anchor::Custom(Vec2::new(0.1875, 0.227)),
                        custom_size: Some(Vec2::new(55.0, 55.0)),
                        ..default()
                    },
                    ..default()
                },
                ClockHandType::Minute,
            ));
        });
}

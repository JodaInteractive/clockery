//! Handle player input and translate it into movement.
//! Note that the approach used here is simple for demonstration purposes.
//! If you want to move the player in a smoother way,
//! consider using a [fixed timestep](https://github.com/bevyengine/bevy/blob/latest/examples/movement/physics_in_fixed_timestep.rs).

use bevy::prelude::*;

use crate::AppSet;

use super::spawn::clock::InteractClock;

pub(super) fn plugin(app: &mut App) {
    // Record directional input as movement controls.
    app.register_type::<MovementController>();
    app.add_systems(
        Update,
        record_movement_controller.in_set(AppSet::RecordInput),
    );

    app.add_systems(FixedUpdate, (apply_movement).chain().in_set(AppSet::Update));
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct MovementController(pub Vec2);

fn record_movement_controller(
    input: Res<ButtonInput<KeyCode>>,
    mut controller_query: Query<&mut MovementController>,
) {
    let mut intent = Vec2::ZERO;
    if input.pressed(KeyCode::KeyA) || input.pressed(KeyCode::ArrowLeft) {
        intent.x -= 1.0;
    }
    if input.pressed(KeyCode::KeyD) || input.pressed(KeyCode::ArrowRight) {
        intent.x += 1.0;
    }

    let intent = intent.normalize_or_zero();

    for mut controller in &mut controller_query {
        controller.0 = intent;
    }
}

fn apply_movement(
    mut movement_query: Query<(&MovementController, &mut Transform), Without<InteractClock>>,
    mut clocks: Query<(Entity, &mut Transform), With<InteractClock>>,
) {
    // if there are no clocks, short-circuit
    let clock_count = clocks.iter().count();
    if clock_count == 1 {
        return;
    }

    // get all clocks and their positions
    let clock_positions = clocks
        .iter()
        .map(|(e, t)| (e, t.translation.x))
        .collect::<Vec<(Entity, f32)>>();

    // get the player's current position
    let result = movement_query.get_single_mut();
    if result.is_err() {
        return;
    }
    let (movement, mut transform) = result.unwrap();
    let character_position = transform.translation.x;

    // get the current clock and it's position
    let current_clock = clock_positions
        .iter()
        .enumerate()
        .find(|(_, &x)| x.1 == character_position)
        .unwrap();

    // if the player wants to move right
    if movement.0.x > 0.0 {
        // check for clocks to the right
        let right_clocks = clock_positions
            .iter()
            .filter(|&(_, x)| x > &character_position)
            .count();
        // if there are none, short-circuit
        if right_clocks == 0 {
            return;
        }
        // move the player to the next clock
        transform.translation.x = clock_positions[current_clock.0 + 1].1;

        // lower the clock the player was on
        clocks.get_mut(current_clock.1 .0).unwrap().1.translation.y = -230.0;
        // raise the clock the player is on
        clocks
            .get_mut(clock_positions[current_clock.0 + 1].0)
            .unwrap()
            .1
            .translation
            .y = -190.0;
    } else if movement.0.x < 0.0 {
        let left_clocks = clock_positions
            .iter()
            .filter(|&(_, x)| x < &character_position)
            .count();
        if left_clocks == 0 {
            return;
        }
        transform.translation.x = clock_positions[current_clock.0 - 1].1;

        clocks.get_mut(current_clock.1 .0).unwrap().1.translation.y = -230.0;
        clocks
            .get_mut(clock_positions[current_clock.0 - 1].0)
            .unwrap()
            .1
            .translation
            .y = -190.0;
    }
}

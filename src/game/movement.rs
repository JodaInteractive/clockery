//! Handle player input and translate it into movement.
//! Note that the approach used here is simple for demonstration purposes.
//! If you want to move the player in a smoother way,
//! consider using a [fixed timestep](https://github.com/bevyengine/bevy/blob/latest/examples/movement/physics_in_fixed_timestep.rs).

use bevy::prelude::*;

use super::spawn::clock::{Clock, ClockController, Interactable, Positions};
use crate::{screen::Screen, AppSet};

pub(super) fn plugin(app: &mut App) {
    // Record directional input as movement controls.
    app.register_type::<MovementController>();
    app.add_systems(
        Update,
        movement
            .in_set(AppSet::RecordInput)
            .run_if(in_state(Screen::Playing)),
    );
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct MovementController(pub Vec2);

fn movement(
    input: Res<ButtonInput<KeyCode>>,
    mut controller_query: Query<(&mut ClockController, &mut Transform), Without<Clock>>,
    mut clocks: Query<(&mut Transform, &mut Clock), With<Interactable>>,
    positions: Res<Positions>,
) {
    let mut intent = Vec2::ZERO;
    if input.just_pressed(KeyCode::KeyA) || input.just_pressed(KeyCode::ArrowLeft) {
        intent.x -= 1.0;
    }
    if input.just_pressed(KeyCode::KeyD) || input.just_pressed(KeyCode::ArrowRight) {
        intent.x += 1.0;
    }

    let intent = intent.normalize_or_zero();

    let (mut controller, mut transform) = controller_query.get_single_mut().unwrap();
    controller.direction = intent;

    if controller.direction == Vec2::ZERO
        || controller.index == 0 && controller.direction.x < 0.0
        || controller.index == 6 && controller.direction.x > 0.0
    {
        return;
    }

    let prev_x = transform.translation.x;
    controller.index = (controller.index as i32 + controller.direction.x as i32) as usize;
    let position = match controller.index {
        0 => positions.clock_spawn,
        1 => positions.clock_1,
        2 => positions.clock_2,
        3 => positions.clock_3,
        4 => positions.clock_4,
        5 => positions.clock_5,
        6 => positions.oil_can,
        _ => panic!("Invalid index"),
    };

    transform.translation.x = position.x;

    let current_clock = clocks.iter_mut().find(|(t, _)| t.translation.x == prev_x);
    if current_clock.is_some() {
        let mut clock = current_clock.unwrap();
        if !clock.1.is_main {
            clock.0.translation.y = -220.0;
        }
    }

    let target_clock = clocks
        .iter_mut()
        .find(|(t, _)| t.translation.x == position.x);
    if target_clock.is_some() {
        let mut clock = target_clock.unwrap();
        if !clock.1.is_main {
            clock.0.translation.y = -190.0;
        }
    }
}

//! Handle player input and translate it into movement.
//! Note that the approach used here is simple for demonstration purposes.
//! If you want to move the player in a smoother way,
//! consider using a [fixed timestep](https://github.com/bevyengine/bevy/blob/latest/examples/movement/physics_in_fixed_timestep.rs).

use bevy::prelude::*;

use super::{
    assets::SfxKey,
    audio::sfx::PlaySfx,
    spawn::clock::{Clock, ClockController, Interactable, Positions},
};
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
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    mut controller_query: Query<(&mut ClockController, &mut Transform), Without<Clock>>,
    mut clocks: Query<(Entity, &mut Transform, &mut Clock), With<Interactable>>,
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
    if controller.index == 0 && controller.direction.x < 0.0 {
        controller.index = 0;
    } else {
        controller.index = (controller.index as i32 + controller.direction.x as i32) as usize;
        controller.index = controller.index.min(6);
    }
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

    // pick up clock
    if input.just_pressed(KeyCode::Space) {
        if controller.held_clock.is_some() {
            let clock_count = clocks
                .iter_mut()
                .filter(|(_, t, _)| t.translation.x == position.x)
                .count();
            if clock_count == 1 {
                let clock = clocks
                    .iter_mut()
                    .find(|(e, _, _)| *e == controller.held_clock.unwrap());
                if clock.is_some() {
                    let mut clock = clock.unwrap();
                    clock.1.translation.y = position.y;
                    controller.held_clock = None;
                }
            }
            let r = rand::random::<f32>();
            if r < 0.25 {
                commands.trigger(PlaySfx::Key(SfxKey::ClockDown1));
            } else if r < 0.5 {
                commands.trigger(PlaySfx::Key(SfxKey::ClockDown2));
            } else if r < 0.75 {
                commands.trigger(PlaySfx::Key(SfxKey::ClockDown3));
            } else {
                commands.trigger(PlaySfx::Key(SfxKey::ClockDown4));
            }
        } else {
            let target_clock = clocks
                .iter_mut()
                .find(|(_, t, _)| t.translation.x == position.x);
            if target_clock.is_some() {
                let mut clock = target_clock.unwrap();
                clock.1.translation.y = position.y + 30.0;
                controller.held_clock = Some(clock.0);
            }
        }
    }

    // move and move held clock
    if controller.direction != Vec2::ZERO {
        transform.translation.x = position.x;
        let held_clock: Option<(Entity, Mut<Transform>, Mut<Clock>)> =
            clocks.iter_mut().find(|(e, _, _)| {
                if controller.held_clock.is_some() {
                    return controller.held_clock.unwrap() == *e;
                }
                false
            });

        if held_clock.is_some() {
            let mut clock = held_clock.unwrap();
            clock.1.translation.x = transform.translation.x;
        }
    }
}

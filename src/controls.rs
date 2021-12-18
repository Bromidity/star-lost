use bevy::{app::ManualEventReader, input::mouse::MouseMotion, prelude::*};

use crate::ship::{AngularImpulse, Impulse};

/// Marks an entity as controlled by the player, meaning [ship_translational_movement_system]
/// and [ship_rotational_movement_system] will attempt to apply [Impulse] and [AngularImpulse]
/// on them according to keyboard and mouse inputs.
#[derive(Component)]
pub struct PlayerControlled;

pub struct ControlsPlugin;

impl Plugin for ControlsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(ship_translational_movement_system)
            .add_system(ship_rotational_movement_system)
            .add_startup_system(initial_grab_cursor)
            .init_resource::<ManualEventReader<MouseMotion>>();
    }
}

fn initial_grab_cursor(mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();
    window.set_cursor_lock_mode(true);
    //window.set_cursor_visibility(false);
    window.set_cursor_position(Vec2::from_slice(&[
        window.width() / 2.0,
        window.height() / 2.0,
    ]));
}

/// Set entities with [PlayerControlled] component's [Impulse] component values based on user input.
/// Default: `W`, `A`, `S`, `D` for strafe impulses, `LShift` and `LControl` for forwards/backwards acceleration respectively.
pub fn ship_translational_movement_system(
    windows: Res<Windows>,
    keys: Res<Input<KeyCode>>,
    mut query: Query<(&mut Impulse, &Transform), With<PlayerControlled>>,
) {
    let window = windows.get_primary().unwrap();
    if window.cursor_locked() {
        for (mut impulse, transform) in query.iter_mut() {
            let mut new_impulse = Vec3::ZERO;
            for key in keys.get_pressed() {
                new_impulse += match key {
                    KeyCode::W => Vec3::from_slice(&[0.0, 1.0, 0.0]),
                    KeyCode::A => Vec3::from_slice(&[-1.0, 0.0, 0.0]),
                    KeyCode::S => Vec3::from_slice(&[0.0, -1.0, 0.0]),
                    KeyCode::D => Vec3::from_slice(&[1.0, 0.0, 0.0]),
                    KeyCode::LShift => Vec3::from_slice(&[0.0, 0.0, -5.0]),
                    KeyCode::LControl => Vec3::from_slice(&[0.0, 0.0, 20.0]),
                    _ => Vec3::ZERO,
                };
            }

            *impulse = Impulse(transform.rotation * new_impulse * 10.0);
        }
    }
}
/// Set entities with [PlayerControlled] component's [AngularImpulse] component values based on user input.
pub fn ship_rotational_movement_system(
    windows: Res<Windows>,
    keys: Res<Input<KeyCode>>,
    mut query: Query<(&mut AngularImpulse, &Transform), With<PlayerControlled>>,
) {
    let window = windows.get_primary().unwrap();

    for (mut impulse, transform) in query.iter_mut() {
        if window.cursor_locked() {
            if let Some(pos) = window.physical_cursor_position() {
                let pos = pos.as_vec2();

                // Translate position into (-0.5, 0.5) space
                let relative_pos = pos
                    / Vec2::from_slice(&[window.width() as f32, window.height() as f32])
                    - Vec2::from_slice(&[0.5, 0.5]);

                let mut imp = Vec3::from_slice(&[
                    if relative_pos.y.abs() > 0.1 {
                        relative_pos.y
                    } else {
                        0.0
                    },
                    -if relative_pos.x.abs() > 0.1 {
                        relative_pos.x
                    } else {
                        0.0
                    },
                    0.0,
                ]);

                for key in keys.get_pressed() {
                    imp += match key {
                        KeyCode::Q => Vec3::from_slice(&[0.0, 0.0, 1.0]),
                        KeyCode::E => Vec3::from_slice(&[0.0, 0.0, -1.0]),
                        _ => Vec3::ZERO,
                    };
                }

                // Translate the impulse into world space
                *impulse = AngularImpulse(transform.rotation * imp);
            }
        } else {
            *impulse = AngularImpulse::default();
        }
    }
}

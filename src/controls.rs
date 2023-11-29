use bevy::prelude::*;

use crate::impulse::{AngularImpulse, Impulse};

/// Marks an entity as controlled by the player, meaning [ship_translational_movement_system]
/// and [ship_rotational_movement_system] will attempt to apply [Impulse] and [AngularImpulse]
/// on them according to keyboard and mouse inputs.
#[derive(Component)]
pub struct PlayerControlled;

pub struct ControlsPlugin;

impl Plugin for ControlsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, ship_translational_movement_system)
            .add_systems(FixedUpdate, ship_rotational_movement_system);
        //.add_system(animate_ship_camera_effects)
        //.add_startup_system(initial_grab_cursor)
        //.init_resource::<ManualEventReader<MouseMotion>>();
    }
}

/// Set entities with [PlayerControlled] component's [Impulse] component values based on user input.
/// Default: `W` and `S` for acceleration impulses.
fn ship_translational_movement_system(
    keys: Res<Input<KeyCode>>,
    mut query: Query<(&mut Impulse, &Transform), With<PlayerControlled>>,
) {
    for (mut impulse, transform) in query.iter_mut() {
        let mut new_impulse = Vec3::ZERO;
        for key in keys.get_pressed() {
            new_impulse += match key {
                KeyCode::W => Vec3::from_slice(&[0.0, 0.0, -1.0]),
                KeyCode::S => Vec3::from_slice(&[0.0, 0.0, 1.0]),
                _ => Vec3::ZERO,
            };
        }

        *impulse = Impulse(transform.rotation * new_impulse * 10.0);
    }
}

/// Rotates the [PlayerControlled] ship around its own axis when A or D is pressed.
fn ship_rotational_movement_system(
    keys: Res<Input<KeyCode>>,
    mut query: Query<&mut AngularImpulse, With<PlayerControlled>>,
) {
    for mut impulse in query.iter_mut() {
        let mut new_impulse = Vec3::ZERO;
        for key in keys.get_pressed() {
            new_impulse += match key {
                KeyCode::A => Vec3::from_slice(&[0.0, 1.0, 0.0]),
                KeyCode::D => Vec3::from_slice(&[0.0, -1.0, 0.0]),
                _ => Vec3::ZERO,
            };
        }

        *impulse = AngularImpulse(new_impulse * 10.0);
    }
}

/*
/// Set entities with [PlayerControlled] component's [AngularImpulse] component values based on user input.
pub fn ship_rotational_movement_system(
    window_query: Query<&Window, With<PrimaryWindow>>,
    keys: Res<Input<KeyCode>>,
    mut query: Query<(&mut AngularImpulse, &Transform), With<PlayerControlled>>,
) {
    if let Some(window) = window_query.get_single().ok() {
        for (mut impulse, transform) in query.iter_mut() {

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
}
 */
/*
pub fn animate_ship_camera_effects(
    mut cameras: Query<(&mut Transform, &Parent), With<WorldCamera>>,
    ship: Query<(&Transform, &Velocity, &AngularVelocity), Without<WorldCamera>>,
) {
    for (mut cam_trans, parent) in cameras.iter_mut() {
        if let Ok((ship_trans, velocity, angular_velocity)) = ship.get(parent.get()) {
            let translation_from_speed = {
                let local_velocity = ship_trans.rotation.inverse() * velocity.0;
                let vel = local_velocity.length();
                if vel.is_normal() {
                    -local_velocity
                } else {
                    Vec3::ZERO
                }
            };

            let translation_from_spin = {
                let angular_velocity = ship_trans.rotation.inverse() * angular_velocity.0;
                let ang_vel = angular_velocity.length();
                if ang_vel.is_normal() {
                    Vec3::from_slice(&[-angular_velocity.y, angular_velocity.x, 0.0])
                } else {
                    Vec3::ZERO
                }
            } * 10.0;

            let offset = {
                let offset = translation_from_speed + translation_from_spin;
                let len = offset.length();
                if len.is_normal() {
                    offset.normalize() * len.powf(0.90) * 0.03
                } else {
                    Vec3::ZERO
                }
            };

            cam_trans.translation = Vec3::from_slice(&[0.0, 0.05, 0.6]) + offset;
        }
    }
}
 */

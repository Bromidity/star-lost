use bevy::{app::ManualEventReader, input::mouse::MouseMotion, prelude::*};

use crate::ship::{AngularImpulse, Impulse};

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

/// Grabs the cursor when game first starts
fn initial_grab_cursor(mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();
    window.set_cursor_lock_mode(true);
    //window.set_cursor_visibility(false);
    window.set_cursor_position(Vec2::from_slice(&[
        window.width() / 2.0,
        window.height() / 2.0,
    ]));
}

pub fn ship_translational_movement_system(
    windows: Res<Windows>,
    keys: Res<Input<KeyCode>>,
    mut query: Query<&mut Impulse, With<PlayerControlled>>,
) {
    let window = windows.get_primary().unwrap();
    if window.cursor_locked() {
        for mut impulse in query.iter_mut() {
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

            *impulse = Impulse(new_impulse);
        }
    }
}

pub fn ship_rotational_movement_system(
    windows: Res<Windows>,
    mut query: Query<&mut AngularImpulse, With<PlayerControlled>>,
) {
    let window = windows.get_primary().unwrap();

    for mut impulse in query.iter_mut() {
        if window.cursor_locked() {
            let pos = window.physical_cursor_position().unwrap().as_vec2();
            let relative_pos = pos
                / Vec2::from_slice(&[window.width() as f32, window.height() as f32])
                - Vec2::from_slice(&[0.5, 0.5]);

            // Dead zone in the middle
            if relative_pos.length() > 0.1 {
                let imp = Vec3::from_slice(&[relative_pos.y, -relative_pos.x, 0.0])
                    .clamp(-Vec3::ONE, Vec3::ONE)
                    .normalize();

                *impulse = AngularImpulse(imp);
            } else {
                *impulse = AngularImpulse::default();
            }
        } else {
            *impulse = AngularImpulse::default();
        }
    }
}

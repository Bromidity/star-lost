use bevy::prelude::*;

use crate::ship::Impulse;

#[derive(Component)]
pub struct PlayerControlled;

pub fn ship_movement_system(
    keys: Res<Input<KeyCode>>,
    mut query: Query<&mut Impulse, With<PlayerControlled>>,
) {
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

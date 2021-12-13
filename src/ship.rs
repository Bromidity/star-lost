use bevy::math::{EulerRot, Vec3};
use bevy::prelude::*;

use crate::physics::{Acceleration, AngularAcceleration, PhysicsBundle};
use crate::tracking::{Target, TargetEntity};

#[derive(Default, Component)]
pub struct Impulse(pub Vec3);

#[derive(Default, Component)]
pub struct AngularImpulse(pub Vec3);

#[derive(Component)]
pub struct ThrustCharacteristics {
    pub min: Vec3,
    pub max: Vec3,
    // Rotational thrust characteristics are symmetric
    pub rot: Vec3,
}

impl Default for ThrustCharacteristics {
    fn default() -> Self {
        Self {
            min: Vec3::from_slice(&[-1.0, -5.0, -1.0]),
            max: Vec3::from_slice(&[1.0, 2.0, 1.0]),
            rot: Vec3::from_slice(&[1.0, 1.0, 1.0]),
        }
    }
}

#[derive(Default, Bundle)]
pub struct ShipBundle {
    pub impulse: Impulse,
    pub angular_impulse: AngularImpulse,
    pub thrust_characteristics: ThrustCharacteristics,
    #[bundle]
    pub physics: PhysicsBundle,
}

pub struct ShipPlugin;

impl Plugin for ShipPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(impulse_system)
            .add_system(angular_impulse_system);
    }
}

pub fn impulse_system(mut query: Query<(&mut Acceleration, &Impulse, &ThrustCharacteristics)>) {
    for (mut acceleration, impulse, thrust) in query.iter_mut() {
        acceleration.0 = if impulse.0.length_squared().is_normal() {
            let l = (thrust.max / impulse.0).abs();
            let h = (thrust.min / impulse.0).abs();

            let smallest_factor = [l.x, l.y, l.z, h.x, h.y, h.z, impulse.0.length()]
                .iter()
                .cloned()
                .filter(|f| f.is_normal())
                .reduce(f32::min)
                .unwrap_or(0.0);

            impulse.0 * smallest_factor
        } else {
            Vec3::ZERO
        }
    }
}

pub fn angular_impulse_system(
    mut query: Query<(
        &mut AngularAcceleration,
        &AngularImpulse,
        &ThrustCharacteristics,
    )>,
) {
    for (mut acceleration, impulse, thrust) in query.iter_mut() {
        acceleration.0 = if impulse.0.length_squared().is_normal() {
            let l = (thrust.rot / impulse.0).abs();
            let h = (-thrust.rot / impulse.0).abs();

            let smallest_factor = [l.x, l.y, l.z, h.x, h.y, h.z, impulse.0.length()]
                .iter()
                .cloned()
                .filter(|f| f.is_finite() && f > &0.0)
                .reduce(f32::min)
                .unwrap_or(0.0);

            impulse.0 * smallest_factor
        } else {
            Vec3::ZERO
        }
    }
}

pub fn spawn_ships(mut commands: Commands, asset_server: Res<AssetServer>) {
    let model = asset_server.load("ship.glb#Scene0");

    let id = {
        commands
            .spawn_bundle(ShipBundle {
                impulse: Impulse(Vec3::from_slice(&[0.0, 0.0, -0.5])),
                angular_impulse: AngularImpulse(Vec3::from_slice(&[0.0, 0.5, 0.0])),
                thrust_characteristics: ThrustCharacteristics {
                    min: Vec3::from_slice(&[-1.0, -2.0, -1.0]),
                    max: Vec3::from_slice(&[1.0, 2.0, 1.0]),
                    rot: Vec3::from_slice(&[0.5, 0.5, 0.5]),
                },
                physics: PhysicsBundle {
                    transform: Transform::from_xyz(0.0, 5.0, 0.0),
                    ..Default::default()
                },
                ..Default::default()
            })
            .with_children(|parent| {
                parent.spawn_scene(model.clone());
            })
            .id()
    };

    commands
        .spawn_bundle(ShipBundle {
            impulse: Impulse(Vec3::from_slice(&[0.0, 0.0, -0.5])),
            thrust_characteristics: ThrustCharacteristics {
                min: Vec3::from_slice(&[-1.0, -1.0, -1.0]),
                max: Vec3::from_slice(&[1.0, 2.0, 1.0]),
                rot: Vec3::from_slice(&[5.0, 5.0, 5.0]),
            },
            physics: PhysicsBundle {
                transform: Transform::from_xyz(5.0, -0.0, -0.0).with_rotation(Quat::from_euler(
                    EulerRot::XYZ,
                    0.0,
                    1.0,
                    0.0,
                )),
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_scene(model.clone());
        })
        .insert(TargetEntity(id))
        .insert(Target(Vec3::from_slice(&[0.0, 0.0, 0.0])));
}

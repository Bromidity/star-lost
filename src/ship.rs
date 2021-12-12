use bevy::math::{EulerRot, Vec3};
use bevy::prelude::*;

use crate::physics::{Acceleration, AngularAcceleration, PhysicsBundle};
use crate::tracking::{Target, TargetEntity};

#[derive(Default, Component)]
pub struct Impulse(pub Vec3);

#[derive(Default, Component)]
pub struct AngularImpulse(pub Vec3);

/*
struct ThrustCharacteristics {
    min: Vec3,
    max: Vec3,
    // Rotational thrust characteristics are symmetric
    rot: Vec3,
}
 */

#[derive(Default, Bundle)]
pub struct ShipBundle {
    impulse: Impulse,
    angular_impulse: AngularImpulse,
    #[bundle]
    physics: PhysicsBundle,
}

pub struct ShipPlugin;

impl Plugin for ShipPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(impulse_system)
            .add_system(angular_impulse_system);
    }
}

pub fn impulse_system(mut query: Query<(&mut Acceleration, &Impulse)>) {
    for (mut acceleration, impulse) in query.iter_mut() {
        acceleration.0 = impulse.0
    }
}

pub fn angular_impulse_system(mut query: Query<(&mut AngularAcceleration, &AngularImpulse)>) {
    for (mut acceleration, impulse) in query.iter_mut() {
        acceleration.0 = impulse.0
    }
}

pub fn spawn_ships(mut commands: Commands, asset_server: Res<AssetServer>) {
    let scene = asset_server.load("ship.gltf#Scene0");

    let id = {
        commands
            .spawn_bundle((
                Transform::from_xyz(0.0, 5.0, 0.0),
                GlobalTransform::identity(),
            ))
            .with_children(|parent| {
                parent.spawn_scene(scene.clone());
            })
            .insert_bundle(ShipBundle {
                impulse: Impulse(Vec3::from_slice(&[0.0, 0.0, -0.5])),
                angular_impulse: AngularImpulse(Vec3::from_slice(&[0.0, 0.1, 0.0])),
                ..Default::default()
            })
            .id()
    };

    commands
        .spawn_bundle((
            Transform::from_xyz(5.0, -0.0, -0.0).with_rotation(Quat::from_euler(
                EulerRot::XYZ,
                0.0,
                1.0,
                0.0,
            )),
            GlobalTransform::identity(),
        ))
        .with_children(|parent| {
            parent.spawn_scene(scene.clone());
        })
        .insert_bundle(ShipBundle {
            impulse: Impulse(Vec3::from_slice(&[0.0, 0.0, -0.5])),
            ..Default::default()
        })
        .insert(TargetEntity(id))
        .insert(Target(Vec3::from_slice(&[0.0, 0.0, 0.0])));
}

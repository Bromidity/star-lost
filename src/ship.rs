use bevy::math::{EulerRot, Vec3};
use bevy::prelude::*;

use crate::physics::PhysicsBundle;
use crate::tracking::{Target, TargetEntity};

struct ThrustCharacteristics {
    min: Vec3,
    max: Vec3,
    // Rotational thrust characteristics are symmetric
    rot: Vec3,
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
            .insert_bundle(PhysicsBundle::default())
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
        .insert_bundle(PhysicsBundle::default())
        .insert(TargetEntity(id))
        .insert(Target(Vec3::from_slice(&[0.0, 0.0, 0.0])));
}

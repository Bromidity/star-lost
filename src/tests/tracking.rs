use bevy::{math::EulerRot, prelude::*};

use crate::{
    debug::{AddDebugArrow, AddDebugValue, DebugVector},
    impulse::*,
    physics::*,
    tracking::*,
};

#[allow(dead_code)]
pub fn spawn_tracking_ships(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let model = asset_server.load("models/ship.glb#Scene0");

    let id = {
        // Leader
        commands
            .spawn_bundle(ShipBundle {
                thrust_characteristics: ThrustCharacteristics {
                    min: Vec3::from_slice(&[-1.0, -2.0, -1.0]),
                    max: Vec3::from_slice(&[1.0, 2.0, 1.0]),
                    rot: Vec3::from_slice(&[0.01, 0.01, 0.01]),
                },
                physics: PhysicsBundle {
                    velocity: Velocity(Vec3::from_slice(&[0.0, 0.0, -5.0])),
                    transform: Transform::from_xyz(0.0, 5.0, 20.0),
                    ..Default::default()
                },
                ..Default::default()
            })
            .debug_vector::<Acceleration>(asset_server)
            .with_children(|parent| {
                parent.spawn_bundle(SceneBundle {
                    scene: model.clone(),
                    ..Default::default()
                });
                parent.spawn_bundle((DebugVector::<Acceleration>::default(),));
            })
            .id()
    };

    // Follower
    commands
        .spawn_bundle(ShipBundle {
            impulse: Impulse(Vec3::from_slice(&[0.0, 0.0, -0.5])),
            thrust_characteristics: ThrustCharacteristics {
                min: Vec3::from_slice(&[-1.0, -1.0, -10.0]),
                max: Vec3::from_slice(&[1.0, 1.0, 1.0]),
                rot: Vec3::from_slice(&[100.0, 100.0, 100.0]),
            },
            physics: PhysicsBundle {
                transform: Transform::from_xyz(50.0, -0.0, -0.0).with_rotation(Quat::from_euler(
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
            parent.spawn_bundle(SceneBundle {
                scene: model.clone(),
                ..Default::default()
            });
        })
        .debug_vector::<Acceleration>(asset_server)
        .debug_value::<Acceleration>("acceleration")
        .insert(TargetEntity(id))
        .insert(Target(Vec3::from_slice(&[0.0, 0.0, 0.0])));
}

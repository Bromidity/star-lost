use bevy::prelude::*;

use crate::{physics::*, ship::*};

#[allow(dead_code)]
pub fn spawn_player_ship(mut commands: Commands, asset_server: Res<AssetServer>) {
    let model = asset_server.load("ship.glb#Scene0");
    commands
        .spawn_bundle(ShipBundle {
            physics: PhysicsBundle {
                drag: Drag(0.2),
                ..Default::default()
            },
            thrust_characteristics: ThrustCharacteristics {
                min: Vec3::from_slice(&[-1.0, -5.0, -1.0]),
                max: Vec3::from_slice(&[1.0, 10.0, 1.0]),
                rot: Vec3::from_slice(&[5.0, 5.0, 5.0]),
            },
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_scene(model.clone());
            parent.spawn_bundle(PerspectiveCameraBundle {
                transform: Transform::from_xyz(10.0, 5.0, 10.0)
                    .looking_at(Vec3::new(0.0, 1.0, 0.0), Vec3::Y),
                ..Default::default()
            });
        });
}

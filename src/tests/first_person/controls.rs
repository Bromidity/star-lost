use bevy::{core_pipeline::tonemapping::Tonemapping, prelude::*};

use crate::{impulse::*, physics::*};

#[allow(dead_code)]
pub fn spawn_player_ship(mut commands: Commands, asset_server: Res<AssetServer>) {
    let model = asset_server.load("models/ship_small_thrust.glb#Scene0");
    commands
        .spawn(ShipBundle {
            physics: PhysicsBundle {
                drag: Drag(0.1),
                ..Default::default()
            },
            spatial: SpatialBundle {
                transform: Transform::from_translation(Vec3::from_slice(&[0.0, -0.0, 0.0])),
                ..Default::default()
            },
            thrust_characteristics: ThrustCharacteristics {
                min: Vec3::from_slice(&[-1.0, -1.0, -5.0]),
                max: Vec3::from_slice(&[1.0, 1.0, 1.0]),
                rot: Vec3::from_slice(&[5.0, 5.0, 5.0]),
            },
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn(SceneBundle {
                scene: model.clone(),
                ..Default::default()
            });
            parent.spawn(Camera3dBundle {
                tonemapping: Tonemapping::TonyMcMapface,
                transform: Transform::from_xyz(0.0, 5.0, 0.0)
                    .looking_at(Vec3::new(0.0, 0.1, 0.0), Vec3::Y)
                    ,
                ..Default::default()
            });
        });
}

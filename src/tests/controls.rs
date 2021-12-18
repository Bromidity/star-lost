use bevy::prelude::*;

use crate::{
    controls::PlayerControlled, debug::AddDebugValue, impulse::*, physics::*, ui::WorldCamera,
};

#[allow(dead_code)]
pub fn spawn_player_ship(commands: &mut Commands, asset_server: Res<AssetServer>) {
    let model = asset_server.load("models/ship_small_thrust.glb#Scene0");
    commands
        .spawn_bundle(ShipBundle {
            physics: PhysicsBundle {
                transform: Transform::from_translation(Vec3::from_slice(&[35.0, 10.0, 35.0])),
                drag: Drag(0.5),
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
            parent.spawn_scene(model.clone());
            parent
                .spawn_bundle(PerspectiveCameraBundle {
                    transform: Transform::from_xyz(0.0, 0.1, 0.6)
                        .looking_at(Vec3::new(0.0, 0.1, 0.0), Vec3::Y),
                    ..Default::default()
                })
                .insert(WorldCamera);
        })
        .insert(PlayerControlled)
        .debug_value::<Impulse>("trans")
        .debug_value::<AngularImpulse>("rot");
}

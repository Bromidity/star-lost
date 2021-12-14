use crate::{physics::AngularVelocity, station::*};
use bevy::prelude::*;

fn donut() -> impl StationPart {
    let arm = Straight {
        forward: Straight {
            forward: Split {
                rotation: 0,
                left: Straight {
                    forward: LargeLeftCorner {
                        left: Straight {
                            forward: (),
                            back: (),
                        },
                        back: (),
                    },
                    back: (),
                },
                right: (),
                back: (),
            },
            back: (),
        },
        back: (),
    };

    Cross {
        left: arm,
        forward: arm,
        right: arm,
        back: arm,
    }
}

#[allow(dead_code)]
pub fn spawn_station(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    position: Vec3,
    rotspeed: f32,
) {
    // Just rotate the station a bit, so its rotation axis is
    // slightly wonky just because it looks cool
    let rot = Quat::from_rotation_x(-rotspeed * 1.5);

    commands
        .spawn_bundle(StationBundle {
            transform: Transform::from_translation(position).with_rotation(rot),
            angular_velocity: AngularVelocity(
                rot.mul_vec3(Vec3::from_slice(&[0.0, rotspeed, 0.0])),
            ),
            ..Default::default()
        })
        .with_children(|parent| {
            donut().build(parent, asset_server, Vec3::ZERO, 0);
        });
}

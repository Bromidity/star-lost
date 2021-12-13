use crate::station::*;
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
pub fn spawn_stations(commands: &mut Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(StationBundle::default())
        .with_children(|parent| {
            donut().build(parent, &asset_server, Vec3::ZERO, 0);
        });
}

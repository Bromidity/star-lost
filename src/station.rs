use bevy::prelude::*;

type SourceDirection = u8;

const OFFSETS: [[f32; 3]; 4] = [
    [-1.0, 0.0, 0.0],
    [0.0, 0.0, 1.0],
    [1.0, 0.0, 0.0],
    [0.0, 0.0, -1.0],
];

trait StationPart {
    fn build(
        &self,
        parent: &mut ChildBuilder,
        asset_server: &Res<AssetServer>,
        offset: Vec3,
        source: SourceDirection,
    );
}

impl StationPart for () {
    fn build(
        &self,
        _parent: &mut ChildBuilder,
        _asset_server: &Res<AssetServer>,
        _offset: Vec3,
        _source: SourceDirection,
    ) {
    }
}

impl<T: StationPart> StationPart for Option<T>
where
    T: StationPart,
{
    fn build(
        &self,
        parent: &mut ChildBuilder,
        asset_server: &Res<AssetServer>,
        offset: Vec3,
        source: SourceDirection,
    ) {
        if let Some(part) = self {
            part.build(parent, asset_server, offset, source);
        }
    }
}

fn quick_build(parent: &mut ChildBuilder, model: Handle<Scene>, offset: Vec3, rotation: u8) {
    parent
        .spawn_bundle((
            Transform::from_translation(offset)
                .with_rotation(Quat::from_rotation_y(1.5708 * rotation as f32)),
            GlobalTransform::identity(),
        ))
        .with_children(|segment| {
            segment.spawn_scene(model);
        });
}

struct Cross<A = (), B = (), C = (), D = ()>
where
    A: StationPart,
    B: StationPart,
    C: StationPart,
    D: StationPart,
{
    left: A,
    forward: B,
    right: C,
    back: D,
}

impl<A, B, C, D> StationPart for Cross<A, B, C, D>
where
    A: StationPart,
    B: StationPart,
    C: StationPart,
    D: StationPart,
{
    fn build(
        &self,
        parent: &mut ChildBuilder,
        asset_server: &Res<AssetServer>,
        offset: Vec3,
        source: SourceDirection,
    ) {
        let offset = offset + Vec3::from_slice(&OFFSETS[(source as usize) % 4]);

        quick_build(
            parent,
            asset_server.load("pipe_cross.glb#Scene0"),
            offset,
            source,
        );

        self.left.build(parent, asset_server, offset, source + 1);
        self.forward.build(parent, asset_server, offset, source + 2);
        self.right.build(parent, asset_server, offset, source + 3);
        self.back.build(parent, asset_server, offset, source);
    }
}

struct Straight<A = (), B = ()>
where
    A: StationPart,
    B: StationPart,
{
    forward: A,
    back: B,
}

impl<A, B> StationPart for Straight<A, B>
where
    A: StationPart,
    B: StationPart,
{
    fn build(
        &self,
        parent: &mut ChildBuilder,
        asset_server: &Res<AssetServer>,
        offset: Vec3,
        source: SourceDirection,
    ) {
        let offset = offset + Vec3::from_slice(&OFFSETS[(source as usize) % 4]);

        quick_build(
            parent,
            asset_server.load("pipe_straight.glb#Scene0"),
            offset,
            source + 1,
        );

        self.forward.build(parent, asset_server, offset, source);
        self.back.build(parent, asset_server, offset, source + 2);
    }
}

struct LeftCorner<A = (), B = ()>
where
    A: StationPart,
    B: StationPart,
{
    left: A,
    back: B,
}

impl<A, B> StationPart for LeftCorner<A, B>
where
    A: StationPart,
    B: StationPart,
{
    fn build(
        &self,
        parent: &mut ChildBuilder,
        asset_server: &Res<AssetServer>,
        offset: Vec3,
        source: SourceDirection,
    ) {
        let offset = offset + Vec3::from_slice(&OFFSETS[(source as usize) % 4]);

        quick_build(
            parent,
            asset_server.load("pipe_corner_round.glb#Scene0"),
            offset,
            source + 2,
        );

        self.left.build(parent, asset_server, offset, source + 1);
        self.back.build(parent, asset_server, offset, source + 2);
    }
}

pub fn spawn_stations(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle((
            Transform::from_xyz(0.0, 0.0, 0.0),
            GlobalTransform::identity(),
        ))
        .with_children(|parent| {
            Cross {
                left: Straight {
                    forward: LeftCorner {
                        left: Straight {
                            forward: LeftCorner {
                                left: Straight {
                                    forward: LeftCorner {
                                        left: Straight {
                                            forward: (),
                                            back: (),
                                        },
                                        back: (),
                                    },
                                    back: (),
                                },
                                back: (),
                            },
                            back: (),
                        },
                        back: (),
                    },
                    back: (),
                },
                forward: (),
                right: (),
                back: (),
            }
            .build(parent, &asset_server, Vec3::ZERO, 0);
        });
}

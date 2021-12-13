use bevy::prelude::*;

type SourceDirection = u8;

const OFFSETS: [[f32; 3]; 4] = [
    [-1.0, 0.0, 0.0],
    [0.0, 0.0, 1.0],
    [1.0, 0.0, 0.0],
    [0.0, 0.0, -1.0],
];

pub trait StationPart {
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
            Transform::from_translation(offset).with_rotation(Quat::from_rotation_y(
                std::f32::consts::FRAC_PI_2 * rotation as f32,
            )),
            GlobalTransform::identity(),
        ))
        .with_children(|segment| {
            segment.spawn_scene(model);
        });
}

#[derive(Copy, Clone)]
pub struct Cross<A = (), B = (), C = (), D = ()>
where
    A: StationPart,
    B: StationPart,
    C: StationPart,
    D: StationPart,
{
    pub left: A,
    pub forward: B,
    pub right: C,
    pub back: D,
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

#[derive(Copy, Clone)]
pub struct Straight<A = (), B = ()>
where
    A: StationPart,
    B: StationPart,
{
    pub forward: A,
    pub back: B,
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

#[derive(Copy, Clone)]
pub struct LeftCorner<A = (), B = ()>
where
    A: StationPart,
    B: StationPart,
{
    pub left: A,
    pub back: B,
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

#[derive(Copy, Clone)]
pub struct LargeLeftCorner<A = (), B = ()>
where
    A: StationPart,
    B: StationPart,
{
    pub left: A,
    pub back: B,
}

impl<A, B> StationPart for LargeLeftCorner<A, B>
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
        let offset = offset + Vec3::from_slice(&OFFSETS[(source as usize) % 4]) * 2.0;

        quick_build(
            parent,
            asset_server.load("pipe_corner_round_large.glb#Scene0"),
            offset,
            source + 2,
        );

        self.left.build(
            parent,
            asset_server,
            offset + Vec3::from_slice(&OFFSETS[(source as usize + 1) % 4]),
            source + 1,
        );
        self.back.build(
            parent,
            asset_server,
            offset - Vec3::from_slice(&OFFSETS[(source as usize) % 4]) * 2.0,
            source,
        );
    }
}

#[derive(Copy, Clone)]
pub struct Split<A, B, C>
where
    A: StationPart,
    B: StationPart,
    C: StationPart,
{
    pub rotation: u8,
    pub left: A,
    pub right: B,
    pub back: C,
}

impl<A, B, C> StationPart for Split<A, B, C>
where
    A: StationPart,
    B: StationPart,
    C: StationPart,
{
    fn build(
        &self,
        parent: &mut ChildBuilder,
        asset_server: &Res<AssetServer>,
        offset: Vec3,
        source: SourceDirection,
    ) {
        let offset = offset + Vec3::from_slice(&OFFSETS[source as usize % 4]);

        quick_build(
            parent,
            asset_server.load("pipe_split.glb#Scene0"),
            offset,
            source + self.rotation + 3,
        );

        self.left
            .build(parent, asset_server, offset, source + self.rotation + 1);
        self.back
            .build(parent, asset_server, offset, source + self.rotation);
        self.right
            .build(parent, asset_server, offset, source + self.rotation + 3);
    }
}

#[derive(Bundle, Default)]
pub struct StationBundle {
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

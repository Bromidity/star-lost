use std::{marker::PhantomData, ops::Deref};

use bevy::prelude::*;

#[derive(Component)]
pub struct DebugArrow<T: Component> {
    _data: PhantomData<T>,
}

impl<T: Component> Default for DebugArrow<T> {
    fn default() -> Self {
        Self {
            _data: PhantomData::default(),
        }
    }
}

pub trait AddDebugArrow {
    fn debug_vector<T: Component + Deref<Target = Vec3>>(
        &mut self,
        _asset_server: &Res<AssetServer>,
    ) {
    }
}

impl AddDebugArrow for ChildBuilder<'_, '_, '_> {
    fn debug_vector<T: Component + Deref<Target = Vec3>>(
        &mut self,
        asset_server: &Res<AssetServer>,
    ) {
        self.spawn_bundle((
            Transform::default(),
            GlobalTransform::default(),
            DebugArrow::<T>::default(),
        ))
        .with_children(|arrow| {
            arrow.spawn_scene(asset_server.load("arrow.glb#Scene0"));
        });
    }
}

pub fn debug_arrow_system<T: Component + Deref<Target = Vec3>>(
    mut query: Query<(&mut Transform, &Parent), With<DebugArrow<T>>>,
    value: Query<&T>,
) {
    for (mut transform, parent) in query.iter_mut() {
        if let Ok(debugged_value) = value.get_component::<T>(parent.0) {
            transform.rotation = Quat::from_rotation_arc(Vec3::X, debugged_value.normalize());
            transform.scale = Vec3::from_slice(&[1.0, debugged_value.length() * 2.0, 1.0]);
        }
    }
}
/*
pub fn debug_arrow<T: Component + Deref<Target = Vec3>>(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
) {
}
 */

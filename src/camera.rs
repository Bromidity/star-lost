use bevy::{
    app::{Plugin, Update},
    ecs::{
        component::Component,
        entity::Entity,
        query::{With, Without},
        system::Query,
    },
    math::Vec3,
    reflect::Reflect,
    transform::components::{GlobalTransform, Transform},
};

#[derive(Debug, Component, Reflect)]
pub struct WorldCamera;

#[derive(Debug, Component, Reflect)]
pub struct TrackedByCamera {
    pub camera: Entity,
}

pub struct TrackingCameraPlugin;

impl Plugin for TrackingCameraPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, camera_movement_system)
            .register_type::<TrackedByCamera>()
            .register_type::<WorldCamera>();
    }
}

fn camera_movement_system(
    target_entities: Query<(&GlobalTransform, &TrackedByCamera), Without<WorldCamera>>,
    mut camera: Query<(Entity, &mut Transform), With<WorldCamera>>,
) {
    for (camera_id, mut camera_pos) in camera.iter_mut() {
        let mut targets = target_entities
            .iter()
            .filter(|(_, tracker)| tracker.camera == camera_id);

        if let Some((position, tracker)) = targets.next() {
            if tracker.camera == camera_id {
                camera_pos.translation = position.translation() + Vec3::new(0.0, 5.0, 0.0)
            }
        }

        if targets.next().is_some() {
            panic!("Multiple entities contain TrackedByCamera components with the same camera id {camera_id:#?}");
        }
    }
}

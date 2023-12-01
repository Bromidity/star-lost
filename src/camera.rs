use bevy::{
    app::{FixedUpdate, Plugin},
    ecs::{
        component::Component,
        entity::Entity,
        event::EventReader,
        query::{With, Without},
        schedule::IntoSystemConfigs,
        system::{Query, Res},
    },
    input::mouse::MouseWheel,
    math::Vec3,
    reflect::Reflect,
    time::Time,
    transform::components::{GlobalTransform, Transform},
};

#[derive(Debug, Component, Reflect)]
pub struct WorldCamera;

#[derive(Debug, Component, Reflect)]
pub struct TrackedByCamera {
    pub camera: Entity,
    pub height: f32,
}

pub struct TrackingCameraPlugin;

impl Plugin for TrackingCameraPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            FixedUpdate,
            camera_movement_system.after(crate::physics::velocity_system),
        )
        .add_systems(FixedUpdate, camera_zoom_control)
        .register_type::<TrackedByCamera>()
        .register_type::<WorldCamera>();
    }
}

fn camera_movement_system(
    time: Res<Time>,
    target_entities: Query<(&GlobalTransform, &TrackedByCamera), Without<WorldCamera>>,
    mut camera: Query<(Entity, &mut Transform), With<WorldCamera>>,
) {
    for (camera_id, mut camera_pos) in camera.iter_mut() {
        let mut targets = target_entities
            .iter()
            .filter(|(_, tracker)| tracker.camera == camera_id);

        if let Some((position, tracker)) = targets.next() {
            if tracker.camera == camera_id {
                camera_pos.translation = camera_pos.translation.lerp(
                    position.translation() + Vec3::new(0.0, tracker.height, 0.0),
                    time.delta_seconds() * 50.0,
                );
            }
        }

        if targets.next().is_some() {
            panic!("Multiple entities contain TrackedByCamera components with the same camera id {camera_id:#?}");
        }
    }
}

fn camera_zoom_control(
    mut scroll_evr: EventReader<MouseWheel>,
    mut trackers: Query<&mut TrackedByCamera>,
) {
    use bevy::input::mouse::MouseScrollUnit;

    let mut scroll_distance = 0.0;

    for ev in scroll_evr.read() {
        match ev.unit {
            MouseScrollUnit::Line => {
                scroll_distance += ev.y * 0.1;
            }
            MouseScrollUnit::Pixel => {
                scroll_distance += ev.y;
            }
        }
    }

    for mut tracker in trackers.iter_mut() {
        tracker.height *= 1.0 - scroll_distance;
    }
}

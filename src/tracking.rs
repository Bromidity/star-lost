use bevy::prelude::*;

use crate::{
    physics::{AngularVelocity, Velocity},
    ship::{AngularImpulse, Impulse},
};

#[derive(Component)]
pub struct TargetEntity(pub Entity);

#[derive(Component)]
pub struct Target(pub Vec3);

#[derive(Bundle)]
pub struct TargetingBundle {
    target_entity: TargetEntity,
    target: Target,
}

pub struct TrackingPlugin;

impl Plugin for TrackingPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(targeting_entity_system)
            .add_system(angular_targeting_system)
            .add_system(approach_system);
    }
}

pub fn targeting_entity_system(
    mut query: Query<(&mut Target, &TargetEntity)>,
    positions: Query<&Transform>,
) {
    for (mut target, target_entity) in query.iter_mut() {
        if let Ok(target_transform) = positions.get_component::<Transform>(target_entity.0) {
            *target = Target(target_transform.translation);
        }
    }
}

pub fn angular_targeting_system(
    mut query: Query<(&mut AngularImpulse, &AngularVelocity, &Transform, &Target)>,
) {
    for (mut angular_impulse, angular_velocity, transform, target) in query.iter_mut() {
        // point_at is the target Orientation
        let mut point_at = Transform::from_translation(transform.translation);
        point_at.look_at(target.0, Vec3::Y);

        // Quaternions can flip direction apparently at some odd angles.
        let diff = if point_at.rotation.dot(transform.rotation) <= 0.0 {
            -(point_at.rotation * transform.rotation.inverse()).normalize()
        } else {
            (point_at.rotation * transform.rotation.inverse()).normalize()
        };

        // Get the difference between target Orientation and current Orientation, expressed
        let (diff, _) = diff.to_axis_angle();
        angular_impulse.0 = (diff - angular_velocity.0 * 3.0).normalize()
    }
}

pub fn approach_system(mut query: Query<(&mut Impulse, &Velocity, &Transform, &Target)>) {
    for (mut impulse, velocity, transform, target) in query.iter_mut() {
        let mut point_at = Transform::from_translation(transform.translation);
        point_at.look_at(target.0, Vec3::Y);

        // If we're pointing more or less directly at the target, accelerate
        // otherwise, slow down.
        let dist_diff = point_at.rotation.angle_between(transform.rotation);
        let speed = if dist_diff < 1.0 {
            Vec3::from_slice(&[0.0, 0.0, 1.0 - dist_diff]) + transform.rotation.mul_vec3(velocity.0)
        } else {
            velocity.0 * 50.0
        };
        //println!("{}", speed);
        impulse.0 = -speed;
    }
}

use bevy::prelude::*;

use crate::{
    impulse::{AngularImpulse, Impulse},
    physics::{Acceleration, AngularVelocity, Velocity},
};

/// This marker component enables the [rotate_to_face_acceleration_direction] system for this entity.
#[derive(Component)]
pub struct PointInDirectionOfAcceleration;

/// This marker component enables the [accelerate_towards_target] system for this entity.
#[derive(Component)]
pub struct AccelerateToInterceptTarget;

/// Set the target entity as the entity's target.
/// [targeting_entity_system] will continuously update the entity's [Target] position
/// with the coordinates of the target entity.
#[derive(Component)]
pub struct TargetEntity(pub Entity);

/// Marks a 3D point in space as the [Target] of the entity to which it is applied.
/// The [angular_targeting_system] will attempt to align any entity with a [Target]
/// component so it points at the designated point in space, while [approach_system]
/// will attempt to accelerate towards the point in space, if the entity is directed
/// at the point.
#[derive(Component)]
pub struct Target(pub Vec3);

pub struct TrackingPlugin;

impl Plugin for TrackingPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(targeting_entity_system)
            .add_system(rotate_to_face_acceleration_direction_system)
            .add_system(accelerate_towards_target_system);
    }
}

/// Maps an entity target into a position target by translating a [TargetEntity]'s [Transform].translation.
pub fn targeting_entity_system(
    mut commands: Commands,
    mut query: Query<(Entity, Option<&mut Target>, &TargetEntity)>,
    positions: Query<(&Transform, Option<&Velocity>)>,
) {
    for (entity, target, target_entity) in query.iter_mut() {
        if let Ok((target_transform, velocity)) = positions.get(target_entity.0) {
            let updated_target =
                Target(target_transform.translation + velocity.map(|v| v.0).unwrap_or(Vec3::ZERO));

            if let Some(mut target) = target {
                *target = updated_target;
            } else {
                commands.entity(entity).insert(updated_target);
            }
        }
    }
}

/// Attempts to point in the direction of acceleration. Assuming that the rear engine
/// is more powerful (according to thrust characteristics), it makes sense to always
/// point in the intended direction of travel to maximize thrust potential
pub fn rotate_to_face_acceleration_direction_system(
    mut query: Query<
        (
            &mut AngularImpulse,
            &AngularVelocity,
            &Transform,
            &Acceleration,
        ),
        With<PointInDirectionOfAcceleration>,
    >,
) {
    for (mut angular_impulse, angular_velocity, transform, acceleration) in query.iter_mut() {
        // Convert acceleration to a quaternion so we can compare them as orientations
        let mut point_at = Transform::from_translation(Vec3::ZERO);
        point_at.look_at(acceleration.0, Vec3::Y);

        // Quaternions can flip direction apparently at some odd angles.
        let diff = if point_at.rotation.dot(transform.rotation) <= 0.0 {
            -(point_at.rotation * transform.rotation.inverse()).normalize()
        } else {
            (point_at.rotation * transform.rotation.inverse()).normalize()
        };

        // Get the difference between acceleration vector and current Orientation
        let (diff, _) = diff.to_axis_angle();

        let dir = diff - angular_velocity.0;
        angular_impulse.0 = dir.normalize() * (dir.length() * 2.0).sqrt();
    }
}

/// Perpetually to accelerate any entity with a [Target] component in such a way
/// that it will arrive at the Target location... "soon".
pub fn accelerate_towards_target_system(
    mut query: Query<
        (&mut Impulse, &Velocity, &Transform, &Target),
        With<AccelerateToInterceptTarget>,
    >,
) {
    for (mut impulse, velocity, transform, target) in query.iter_mut() {
        // Poor man's integration. Bias slightly towards current velocity to give the approach a smooth curve
        let dir = target.0 - transform.translation - velocity.0 * 5.0;
        impulse.0 = dir.normalize() * (dir.length() * 2.0).sqrt();
    }
}

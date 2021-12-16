use bevy::prelude::*;

use crate::{
    physics::{AngularVelocity, Velocity},
    ship::{AngularImpulse, Impulse},
};

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

/// Attempts to point entities with [Target] components towards their target positions by
/// applying an [AngularImpulse]
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

        // Get the difference between target Orientation and current Orientation
        let (diff, _) = diff.to_axis_angle();
        angular_impulse.0 = (diff - angular_velocity.0 * 3.0).normalize()
    }
}

/// Perpetually to accelerate any entity with a [Target] component in such a way
/// that it will arrive at the Target location... "soon".
pub fn approach_system(mut query: Query<(&mut Impulse, &Velocity, &Transform, &Target)>) {
    for (mut impulse, velocity, transform, target) in query.iter_mut() {
        // Poor man's integration. Bias slightly towards current velocity to give the approach a smooth curve
        impulse.0 = (target.0 - transform.translation - velocity.0 * 3.0) * 10.0;
    }
}

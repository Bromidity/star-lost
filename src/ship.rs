use bevy::math::Vec3;
use bevy::prelude::*;

use crate::{
    debug::DebuggableValue,
    physics::{Acceleration, AngularAcceleration, PhysicsBundle},
};

/// Specifies the Angular impulse imparted on the object via the [angular_impulse_system] into [AngularAcceleration].
/// **NOTE:** The impulse is relative to the entity's local position, not the entity's position in the world.
/// This means that applying an impulse of say [0.0, 0.0, 1.0] will always make the entity move along its local "forward"-axis relative to itself, rather than along the global Z-axis
#[derive(Debug, Default, Component)]
pub struct Impulse(pub Vec3);

/// Specifies the impulse imparted on the object via the [impulse_system] into [Acceleration].
/// **NOTE:** The impulse is relative to the entity's local rotation, not the entity's rotation in the world.
/// This means that applying an angular impulse of say [0.0, 1.0, 0.0] will always make the entity rotate along its local yaw-axis relative to itself, rather than along the global Y-axis
#[derive(Debug, Default, Component)]
pub struct AngularImpulse(pub Vec3);

/// This cobbled-together structure was/is intended to define the maximum acceleration of the ship in any direction.
/// For example, it might make sense to define an instance of this structure that defines a ship which can accelerate very
/// fast in the forward direction, but relatively slowly along the other axis to simulate a larger rear engine compared to smaller RCS-thrusters for instance.
/// The structure is used by the [impulse_system] and [angular_impulse_system]s to limit the impact of an Impulse.
#[derive(Debug, Component)]
pub struct ThrustCharacteristics {
    pub min: Vec3,
    pub max: Vec3,
    // Rotational thrust characteristics are symmetric
    pub rot: Vec3,
}

impl Default for ThrustCharacteristics {
    fn default() -> Self {
        Self {
            min: Vec3::from_slice(&[-1.0, -5.0, -1.0]),
            max: Vec3::from_slice(&[1.0, 2.0, 1.0]),
            rot: Vec3::from_slice(&[1.0, 1.0, 1.0]),
        }
    }
}

/// [Bundle](https://erasin.wang/books/bevy-cheatbook/programming/ec.html#component-bundles) containing common Ship components.
/// [PhysicsBundle](crate::physics::PhysicsBundle) + Ship control components
#[derive(Default, Bundle)]
pub struct ShipBundle {
    pub impulse: Impulse,
    pub angular_impulse: AngularImpulse,
    pub thrust_characteristics: ThrustCharacteristics,
    #[bundle]
    pub physics: PhysicsBundle,
}

pub struct ShipPlugin;

impl Plugin for ShipPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(impulse_system)
            .add_system(angular_impulse_system)
            .add_plugin(DebuggableValue::<AngularImpulse>::default())
            .add_plugin(DebuggableValue::<Impulse>::default());
    }
}

/// Takes an entity's [Impulse] component and imparts it on the entity relative to the entity's current rotation
/// while respecting the entity's [ThrustCharacteristics]
pub fn impulse_system(
    mut query: Query<(
        &mut Acceleration,
        &Transform,
        &Impulse,
        &ThrustCharacteristics,
    )>,
) {
    for (mut acceleration, transform, impulse, thrust) in query.iter_mut() {
        acceleration.0 = if impulse.0.length_squared().is_normal() {
            let l = (impulse.0 / thrust.max).abs();
            let h = (impulse.0 / thrust.min).abs();

            let smallest_factor = [l.x, l.y, l.z, h.x, h.y, h.z, impulse.0.length()]
                .iter()
                .cloned()
                .filter(|f| f.is_normal())
                .reduce(f32::min)
                .unwrap_or(0.0);

            // Finally rotate the impulse so it is relative to the ship position
            transform.rotation * impulse.0.normalize() * smallest_factor
        } else {
            Vec3::ZERO
        };
    }
}

/// Takes an entity's [AngularImpulse] component and imparts it on the entity relative to the entity's current rotation
/// while respecting the entity's [ThrustCharacteristics]
pub fn angular_impulse_system(
    mut query: Query<(
        &mut AngularAcceleration,
        &AngularImpulse,
        &ThrustCharacteristics,
    )>,
) {
    for (mut acceleration, impulse, thrust) in query.iter_mut() {
        acceleration.0 = if impulse.0.length_squared().is_normal() {
            let l = (thrust.rot / impulse.0).abs();
            let h = ((-thrust.rot) / impulse.0).abs();

            let smallest_factor = [l.x, l.y, l.z, h.x, h.y, h.z, impulse.0.length()]
                .iter()
                .cloned()
                .filter(|f| f.is_normal())
                .reduce(f32::min)
                .unwrap_or(0.0);

            impulse.0.normalize() * smallest_factor
        } else {
            Vec3::ZERO
        }
    }
}

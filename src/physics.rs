use std::ops::Deref;

use bevy::prelude::*;

use crate::debug::{debug_vector_system, DebuggableValue};

/// Translational velocity of the entity. Integrated by [velocity_system] into the entity [Transform]'s translational component.
#[derive(Debug, Default, Component)]
pub struct Velocity(pub Vec3);

impl Deref for Velocity {
    type Target = Vec3;
    fn deref(&self) -> &Vec3 {
        &self.0
    }
}

/// Angular velocity of the entity. Integrated by [angular_velocity_system] into the entity [Transform]'s rotational component.
#[derive(Debug, Default, Component)]
pub struct AngularVelocity(pub Vec3);

impl Deref for AngularVelocity {
    type Target = Vec3;
    fn deref(&self) -> &Vec3 {
        &self.0
    }
}

/// The [drag_system] uses this component to dampen the [Velocity] and [AngularVelocity] components
#[derive(Debug, Default, Component)]
pub struct Drag(pub f32);

/// Defines the *intended* acceleration of an entity. This is integrated into [Velocity] by [acceleration_system]
/// Some systems directly overwrite this component's value.
/// for example [Impulse](crate::ship::Impulse) or [AngularImpulse](crate::ship::AngularImpulse) on the ship.
/// This is the primary way in which player controls & target tracking works.
#[derive(Debug, Default, Component)]
pub struct Acceleration(pub Vec3);

impl Deref for Acceleration {
    type Target = Vec3;
    fn deref(&self) -> &Vec3 {
        &self.0
    }
}

/// Angular acceleration of the entity. Integrated by [angular_acceleration_system] into the entity's [AngularVelocity] component.
#[derive(Debug, Default, Component)]
pub struct AngularAcceleration(pub Vec3);

impl Deref for AngularAcceleration {
    type Target = Vec3;
    fn deref(&self) -> &Vec3 {
        &self.0
    }
}

/// [Bundle](https://erasin.wang/books/bevy-cheatbook/programming/ec.html#component-bundles) containing common physics components.
#[derive(Bundle, Default)]
pub struct PhysicsBundle {
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub drag: Drag,
    pub velocity: Velocity,
    pub acceleration: Acceleration,
    pub angular_velocity: AngularVelocity,
    pub angular_acceleration: AngularAcceleration,
}

/// Adds the `Physics` systems to the application, and defines
/// some of the physics components as [DebuggableValue](crate::debug::DebuggableValue)s (see: [AddDebugValue](crate::debug::AddDebugValue))
pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(drag_system)
            .add_system(acceleration_system)
            .add_system(velocity_system)
            .add_system(angular_velocity_system)
            .add_system(angular_acceleration_system)
            .add_plugin(DebuggableValue::<Transform>::default())
            .add_plugin(DebuggableValue::<Velocity>::default())
            .add_plugin(DebuggableValue::<Acceleration>::default())
            .add_plugin(DebuggableValue::<AngularAcceleration>::default())
            .add_plugin(DebuggableValue::<AngularVelocity>::default())
            .add_system(debug_vector_system::<Velocity>)
            .add_system(debug_vector_system::<Acceleration>);
    }
}

/// Integrates the [Velocity] into the entity [Transform]'s translational component
pub fn velocity_system(time: Res<Time>, mut query: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, velocity) in query.iter_mut() {
        //let translated_velocity = transform.rotation * velocity.0;
        transform.translation += velocity.0 * time.delta_seconds();
    }
}

/// Integrates the [AngularVelocity] into the entity [Transform]'s rotational component
pub fn angular_velocity_system(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &AngularVelocity)>,
) {
    for (mut transform, angular_velocity) in query.iter_mut() {
        let scaled = angular_velocity.0 * time.delta_seconds();
        if scaled.length() > 0.0 {
            let velocity_as_quat =
                Quat::from_axis_angle(scaled.normalize(), scaled.length()).normalize();
            transform.rotate(velocity_as_quat);
        }
    }
}

/// Linearly interpolates between ([Velocity] & [AngularVelocity]) and [Vec3::ZERO]
/// by a factor of [Drag] * `time.delta_seconds()`, effectively slowing the respective velocities.
/// I think timestep might affect this strangely. Assuming a 0.5s frametime, the system
/// will halve the velocities of the entity, whereas a frametime >= 1.0 will immediately stop or
/// even reverse the velocity of the object.
pub fn drag_system(
    time: Res<Time>,
    mut query: Query<(&mut Velocity, &mut AngularVelocity, &Drag)>,
) {
    for (mut velocity, mut angular_velocity, drag) in query.iter_mut() {
        velocity.0 = velocity.0.lerp(Vec3::ZERO, drag.0 * time.delta_seconds());

        angular_velocity.0 = angular_velocity
            .0
            .lerp(Vec3::ZERO, drag.0 * time.delta_seconds());
    }
}

/// Integrates the [AngularAcceleration] into [AngularVelocity]
pub fn angular_acceleration_system(
    time: Res<Time>,
    mut query: Query<(&mut AngularVelocity, &AngularAcceleration)>,
) {
    for (mut velocity, acceleration) in query.iter_mut() {
        velocity.0 += acceleration.0 * time.delta_seconds();
    }
}

/// Integrates [Acceleration] into [Velocity]
pub fn acceleration_system(time: Res<Time>, mut query: Query<(&mut Velocity, &Acceleration)>) {
    for (mut velocity, acceleration) in query.iter_mut() {
        velocity.0 += acceleration.0 * time.delta_seconds();
    }
}

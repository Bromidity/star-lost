use std::ops::Deref;

use bevy::prelude::*;

#[derive(Default, Component)]
pub struct Velocity(pub Vec3);

#[derive(Default, Component)]
pub struct AngularVelocity(pub Vec3);

#[derive(Default, Component)]
pub struct Drag(pub f32);

#[derive(Default, Component)]
pub struct Acceleration(pub Vec3);

impl Deref for Acceleration {
    type Target = Vec3;
    fn deref(&self) -> &Vec3 {
        &self.0
    }
}

#[derive(Default, Component)]
pub struct AngularAcceleration(pub Vec3);

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

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(drag_system)
            .add_system(acceleration_system)
            .add_system(velocity_system)
            .add_system(angular_velocity_system)
            .add_system(angular_acceleration_system);
    }
}

pub fn velocity_system(time: Res<Time>, mut query: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, velocity) in query.iter_mut() {
        let translated_velocity = transform.rotation * velocity.0;
        transform.translation += translated_velocity * time.delta_seconds();
    }
}

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

pub fn angular_acceleration_system(
    time: Res<Time>,
    mut query: Query<(&mut AngularVelocity, &AngularAcceleration)>,
) {
    for (mut velocity, acceleration) in query.iter_mut() {
        velocity.0 += acceleration.0 * time.delta_seconds();
    }
}

pub fn acceleration_system(time: Res<Time>, mut query: Query<(&mut Velocity, &Acceleration)>) {
    for (mut velocity, acceleration) in query.iter_mut() {
        velocity.0 += acceleration.0 * time.delta_seconds();
    }
}

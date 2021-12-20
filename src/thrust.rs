use bevy::prelude::*;

use crate::physics::Acceleration;

/// This is the exponent with which the maximum thrust is approached.
/// 0.5 means approach target thrust at the square root of the difference
const THRUST_ADJUST_SPEED: f32 = 0.5;
const THRUST_WIGGLE_MULTIPLIER: f32 = 0.01;
const THRUST_SCALE_MULTIPLIER: f32 = 20.0;

#[derive(Debug, Component)]
pub struct AnimatedThruster {
    pub vessel: Entity,
    pub initial_scale: Vec3,
    pub scale: Vec3,
    pub thrust: f32,
}

pub struct ThrustPlugin;

impl Plugin for ThrustPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(tag_thrusters_for_animation_system)
            .add_system(update_thrust_from_acceleration_system)
            .add_system(animate_thruster_system);
    }
}

/// Listens for Added<Name> events where the object is called 'thrust_rear'
/// and tags them for thrust animation.
fn tag_thrusters_for_animation_system(
    mut commands: Commands,
    potential_thrusters: Query<(Entity, &Transform, &Name, &Parent), Added<Name>>,
    parents: Query<(Option<&Parent>, Option<&Acceleration>)>,
) {
    fn find_simulated_parent(
        parents: &Query<(Option<&Parent>, Option<&Acceleration>)>,
        entity: Entity,
    ) -> Option<Entity> {
        if let Ok((parent, acceleration)) = parents.get(entity) {
            if acceleration.is_some() {
                Some(entity)
            } else {
                parent.and_then(|p| find_simulated_parent(parents, p.0))
            }
        } else {
            None
        }
    }

    for (thruster, transform, name, parent) in potential_thrusters.iter() {
        if name.as_str().contains("anim_thrust_z_neg") {
            if let Some(ship) = find_simulated_parent(&parents, parent.0) {
                debug!(
                    "inserting AnimatedThruster component for '{}' child of {:?}",
                    name.as_str(),
                    ship
                );
                commands.entity(thruster).insert(AnimatedThruster {
                    vessel: ship,
                    initial_scale: transform.scale,
                    scale: -Vec3::Z,
                    thrust: 0.0,
                });
            } else {
                warn!("found 'anim_thrust_z_neg' component with no simulated parent");
            }
        }
    }
}

/// Sets a thruster's "Thrust" based on the acceleration of its parent
fn update_thrust_from_acceleration_system(
    time: Res<Time>,
    mut thrusters: Query<&mut AnimatedThruster>,
    parent: Query<(&Transform, &Acceleration)>,
) {
    for mut thruster in thrusters.iter_mut() {
        let target_thrust = parent
            .get(thruster.vessel)
            .map(|(trans, acc)| (trans.rotation.inverse() * acc.0 * thruster.scale).length())
            .unwrap_or_default();

        thruster.thrust +=
            (target_thrust - thruster.thrust) * time.delta_seconds().powf(THRUST_ADJUST_SPEED);
    }
}

/// Animates thrusters
fn animate_thruster_system(
    time: Res<Time>,
    mut thrusters: Query<(&mut Transform, &AnimatedThruster)>,
) {
    for (mut transform, thrust) in thrusters.iter_mut() {
        // Do some funky wiggling to make it look cooler.
        transform.rotation = Quat::from_euler(
            EulerRot::XYZ,
            time.delta_seconds() * 100.0 % THRUST_WIGGLE_MULTIPLIER
                - THRUST_WIGGLE_MULTIPLIER / 2.0,
            time.delta_seconds() * 200.0 % THRUST_WIGGLE_MULTIPLIER
                - THRUST_WIGGLE_MULTIPLIER / 2.0,
            time.delta_seconds() * 300.0 % THRUST_WIGGLE_MULTIPLIER
                - THRUST_WIGGLE_MULTIPLIER / 2.0,
        );

        transform.scale = thrust.initial_scale + (thrust.initial_scale * thrust.scale)
            - thrust.initial_scale * thrust.scale * thrust.thrust * THRUST_SCALE_MULTIPLIER;
    }
}

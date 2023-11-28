use bevy::prelude::*;

use crate::GameState;

pub struct OrbitPlugin;

impl Plugin for OrbitPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Orbit>()
            .register_type::<OrbitBody>()
            .add_systems(GameState::Running, orbit_system)
            .add_systems(
                GameState::Running,
                update_orbit_position.before(orbit_system),
            );
    }
}

/// Defines the position which this entity is orbiting.
#[derive(Default, Debug, Component, Reflect)]
#[reflect(Component)]
pub struct Orbit {
    pub position: Vec3,
    pub period: f32,
}

#[derive(Debug, Component, Reflect)]
pub struct OrbitBody(pub Entity);

#[derive(Debug, Bundle)]
pub struct OrbitBundle {
    orbit: Orbit,
    parent: OrbitBody,
}

impl OrbitBundle {
    pub fn body(body: Entity, period: f32) -> OrbitBundle {
        OrbitBundle {
            orbit: Orbit {
                position: Vec3::ZERO,
                period,
            },
            parent: OrbitBody(body),
        }
    }
}

/// We use this two-step process because we can't simultaneously mutate
/// the transform of the orbiting body, whilst reading the transform
/// of the body it is orbiting.
pub fn update_orbit_position(
    mut query: Query<(&mut Orbit, &OrbitBody)>,
    parent: Query<&Transform>,
) {
    for (mut orbit, parent_body) in query.iter_mut() {
        if let Ok(transform) = parent.get(parent_body.0) {
            orbit.position = transform.translation
        }
    }
}

/// Orbits an entity around its [`Orbit`] parent entity.
pub fn orbit_system(time: Res<Time>, mut query: Query<(&mut Transform, &Orbit)>) {
    let position = (time.elapsed_seconds() as f32 + 365000.0) * 6.2832 / 365.0;

    for (mut transform, orbit) in query.iter_mut() {
        // Approximation of orbital height based on orbital period.
        // Raising `orbit.period` to the 0.665th power (almost a square root)
        // approximates the distance in AU. Multiply by 1000 to get distance
        // in giga-meters (the unit used to specify the radius of planets.)
        let orbital_distance = orbit.period.powf(0.665) * 1000.0;

        transform.translation = orbit.position
            + Quat::from_rotation_y(position / orbit.period)
                * Vec3::new(orbital_distance.log10() + 0.5, 0.0, 0.0);
    }
}

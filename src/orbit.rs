use bevy::prelude::*;

pub struct OrbitPlugin;

impl Plugin for OrbitPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(orbit_system);
    }
}

/// Defines the parent body, which this entity is orbiting.
#[derive(Debug, Component)]
pub struct Orbit {
    pub position: Vec3,
    pub offset: f32,
}

/// Orbits an entity around its [`Orbiting`] entity.
pub fn orbit_system(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Orbit)>,
) {
    for (mut transform, mut orbit) in query.iter_mut() {
        orbit.offset += time.delta_seconds() * 0.0001;

        //transform.translation = offset_transform.translation
        transform.rotate_around(orbit.position, Quat::from_rotation_y(orbit.offset));
    }
}

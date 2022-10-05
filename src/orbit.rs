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
pub fn orbit_system(time: Res<Time>, mut query: Query<(&mut Transform, &Orbit)>) {
    for (mut transform, orbit) in query.iter_mut() {
        //transform.translation = offset_transform.translation
        let rotation = Quat::from_rotation_y(time.delta_seconds() * 0.1);
        transform.rotate_around(orbit.position, rotation);

        // Apply the inverse rotation locally to cancel incidental "tidal locking"
        transform.rotate_local_y(-time.delta_seconds() * 0.1);
    }
}

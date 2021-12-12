use bevy::{pbr::AmbientLight, prelude::*};
use physics::PhysicsPlugin;
use tracking::TrackingPlugin;

mod physics;
mod ship;
mod tracking;

fn main() {
    App::new()
        .insert_resource(AmbientLight {
            color: Color::YELLOW,
            brightness: 1.0 / 2.0f32,
        })
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(PhysicsPlugin)
        .add_plugin(TrackingPlugin)
        .add_startup_system(setup)
        .add_startup_system(ship::spawn_ships)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(-30.0, 20.0, 30.0)
            .looking_at(Vec3::new(0.0, 0.3, 0.0), Vec3::Y),
        ..Default::default()
    });
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_xyz(3.0, 10.0, 3.0),
        ..Default::default()
    });
}

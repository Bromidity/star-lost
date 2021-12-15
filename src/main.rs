use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    pbr::AmbientLight,
    prelude::*,
};
use controls::ControlsPlugin;
use debug::DebugPlugin;
use physics::PhysicsPlugin;
use ship::ShipPlugin;
use tracking::TrackingPlugin;

mod controls;
mod debug;
mod physics;
mod ship;
mod station;
mod tests;
mod tracking;
mod ui;

fn main() {
    App::new()
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1.0 / 2.0f32,
        })
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .insert_resource(Msaa { samples: 4 })
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugins(DefaultPlugins)
        .add_plugin(DebugPlugin)
        .add_plugin(PhysicsPlugin)
        .add_plugin(TrackingPlugin)
        .add_plugin(ShipPlugin)
        .add_plugin(ControlsPlugin)
        .add_system(ui::follow_object_system)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    tests::station::spawn_station(
        &mut commands,
        &asset_server,
        Vec3::from_slice(&[10.0, 10.0, 10.0]),
        -0.1,
    );
    tests::station::spawn_station(
        &mut commands,
        &asset_server,
        -Vec3::from_slice(&[10.0, 10.0, 10.0]),
        0.3,
    );
    tests::tracking::spawn_tracking_ships(&mut commands, &asset_server);

    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(-40.0, 20.0, 40.0)
                .looking_at(Vec3::new(0.0, 1.0, 0.0), Vec3::Y),
            ..Default::default()
        })
        .insert(ui::WorldCamera);
    commands.spawn_bundle(UiCameraBundle::default());

    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_xyz(3.0, 10.0, 3.0),
        ..Default::default()
    });
}

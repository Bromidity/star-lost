use bevy::{pbr::AmbientLight, prelude::*};
use bevy_kira_audio::AudioPlugin;
use controls::ControlsPlugin;
use debug::DebugPlugin;
//use dust::DustPlugin;
use impulse::ImpulsePlugin;
use orbit::{Orbit, OrbitPlugin};
use physics::{AngularVelocity, PhysicsPlugin};
use route::RoutePlugin;
use thrust::ThrustPlugin;
use tracking::TrackingPlugin;

mod controls;
mod debug;
mod dust;
mod impulse;
mod orbit;
mod physics;
mod route;
mod station;
mod tests;
mod thrust;
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
        .add_plugins(DefaultPlugins)
        .add_plugin(AudioPlugin)
        .add_plugin(DebugPlugin)
        .add_plugin(PhysicsPlugin)
        .add_plugin(TrackingPlugin)
        .add_plugin(ImpulsePlugin)
        .add_plugin(ControlsPlugin)
        .add_plugin(RoutePlugin)
        .add_plugin(ThrustPlugin)
        .add_plugin(OrbitPlugin)
        //.add_plugin(DustPlugin)
        .add_system(ui::follow_object_system)
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 1.0, 1.0).looking_at(Vec3::new(0.0, 0.1, 0.0), Vec3::Y),
        ..Default::default()
    });

    let material = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        reflectance: 0.0,
        emissive: Color::WHITE,
        ..default()
    });

    let rot = Quat::from_rotation_x(-0.5);

    // commands.spawn_bundle(PointLightBundle {
    //     point_light: PointLight {
    //         color: Color::WHITE,
    //         intensity: 50000.0,
    //         range: 1000.0,
    //         shadows_enabled: true,
    //         ..Default::default()
    //     },
    //     transform: Transform::from_xyz(200.0, 4000.0, 3000.0),
    //     ..Default::default()
    // });

    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Icosphere {
                radius: 0.1,
                subdivisions: 32,
            })),
            material: material.clone(),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        })
        .insert(AngularVelocity(
            rot.mul_vec3(Vec3::from_slice(&[0.0, -0.5, 0.0])),
        ));

    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Icosphere {
                radius: 0.03,
                subdivisions: 32,
            })),
            material,
            transform: Transform::from_xyz(1.0, 0.0, 0.0),
            ..default()
        })
        .insert(Orbit {
            position: Vec3::ZERO,
            offset: 0.0,
        });
}

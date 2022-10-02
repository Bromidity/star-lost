use bevy::prelude::*;

use crate::physics::AngularVelocity;

#[allow(dead_code)]
pub fn spawn_planet(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    rotspeed: f32,
) -> Entity {
    let material = StandardMaterial {
        //base_color: Color::BLACK,
        //emissive: Color::rgb(0.1, 0.1, 0.2),
        reflectance: 0.0,
        metallic: 1.0,
        base_color_texture: Some(asset_server.load("images/earth.png")),
        normal_map_texture: Some(asset_server.load("images/2k_earth_normal_map.png")),
        ..default()
    };

    println!("{:#?}", material);

    let material = materials.add(material);

    // Just rotate the station a bit, so its rotation axis is
    // slightly wonky just because it looks cool
    let rot = Quat::from_rotation_x(-rotspeed * 1.5);

    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Icosphere {
                radius: 50.0,
                subdivisions: 32,
            })),
            material,
            transform: Transform::from_xyz(0.0, -5.0, -150.0),
            ..default()
        })
        .insert(AngularVelocity(
            rot.mul_vec3(Vec3::from_slice(&[0.0, rotspeed, 0.0])),
        ))
        .id()
}

#[allow(dead_code)]
pub fn spawn_sun(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    rotspeed: f32,
) -> Entity {
    let material = StandardMaterial {
        //base_color: Color::BLACK,
        //emissive: Color::rgb(0.1, 0.1, 0.2),
        base_color: Color::WHITE,
        reflectance: 0.0,
        emissive: Color::WHITE,
        ..default()
    };

    let material = materials.add(material);

    // Just rotate the station a bit, so its rotation axis is
    // slightly wonky just because it looks cool
    let rot = Quat::from_rotation_x(-rotspeed * 1.5);

    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            color: Color::WHITE,
            intensity: 50000.0,
            range: 1000.0,
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::from_xyz(200.0, 4000.0, 3000.0),
        ..Default::default()
    });

    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Icosphere {
                radius: 30.0,
                subdivisions: 32,
            })),
            material,
            transform: Transform::from_xyz(300.0, 4000.0, 3000.0),
            ..default()
        })
        .insert(AngularVelocity(
            rot.mul_vec3(Vec3::from_slice(&[0.0, rotspeed, 0.0])),
        ))
        .id()
}

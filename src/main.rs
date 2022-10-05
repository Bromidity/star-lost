use bevy::{app::AppExit, pbr::AmbientLight, prelude::*};
use bevy_egui::{
    egui::{self, Label},
    EguiContext, EguiPlugin,
};
use iyes_loopless::prelude::*;

use bevy_kira_audio::AudioPlugin;
use controls::ControlsPlugin;
use debug::DebugPlugin;
//use dust::DustPlugin;
use impulse::ImpulsePlugin;
use orbit::OrbitPlugin;
use physics::PhysicsPlugin;
use route::RoutePlugin;
use thrust::ThrustPlugin;
use tracking::TrackingPlugin;

mod controls;
mod debug;
mod dust;
mod impulse;
mod orbit;
mod physics;

#[allow(dead_code)]
mod route;
mod station;
mod tests;
mod thrust;
mod tracking;

#[allow(dead_code)]
mod ui;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum GameState {
    Loading,
    MainMenu,
    Running,
    Paused,
    Quit,
}

fn main() {
    App::new()
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1.0 / 5.0f32,
        })
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
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
        .add_loopless_state(GameState::Loading)
        .add_enter_system(GameState::Loading, load_assets)
        //.add_system(ui::follow_object_system)
        //.add_startup_system(setup)
        .add_system(main_menu.run_in_state(GameState::MainMenu))
        .add_system(esc_pause.run_in_state(GameState::Running))
        .add_system(esc_pause.run_in_state(GameState::Paused))
        .add_system(pause_menu.run_in_state(GameState::Paused))
        .add_system(exit_system.run_in_state(GameState::Quit))
        .run();
}

fn load_assets(mut commands: Commands) {
    println!("Hello world");

    commands.insert_resource(NextState(GameState::MainMenu));
}

fn exit_system(mut exit: EventWriter<AppExit>) {
    exit.send(AppExit);
}

fn main_menu(mut commands: Commands, mut egui_context: ResMut<EguiContext>) {
    egui::Window::new("main_menu")
        .title_bar(false)
        .resizable(false)
        .fixed_size(egui::Vec2::new(200.0, 300.0))
        .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::new(-0.0, -0.0))
        .show(egui_context.ctx_mut(), |ui| {
            ui.add_sized([200.0, 50.0], Label::new("Star Lost"));

            if ui
                .add_sized([200.0, 50.0], egui::Button::new("Start"))
                .clicked()
            {
                commands.insert_resource(NextState(GameState::Running))
            }

            if ui
                .add_sized([200.0, 50.0], egui::Button::new("Quit"))
                .clicked()
            {
                commands.insert_resource(NextState(GameState::Quit))
            }
        });
}

fn pause_menu(mut commands: Commands, mut egui_context: ResMut<EguiContext>) {
    egui::Window::new("pause_menu")
        .title_bar(false)
        .resizable(false)
        .fixed_size(egui::Vec2::new(200.0, 300.0))
        .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::new(-0.0, -0.0))
        .show(egui_context.ctx_mut(), |ui| {
            ui.add_sized([200.0, 50.0], Label::new("Paused"));

            if ui
                .add_sized([200.0, 50.0], egui::Button::new("Continue"))
                .clicked()
            {
                commands.insert_resource(NextState(GameState::Running))
            }

            if ui
                .add_sized([200.0, 50.0], egui::Button::new("Exit"))
                .clicked()
            {
                commands.insert_resource(NextState(GameState::MainMenu))
            }
        });
}

fn esc_pause(
    mut commands: Commands,
    keys: Res<Input<KeyCode>>,
    current_state: Res<CurrentState<GameState>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        if current_state.0 == GameState::Running {
            println!("Entering pause state");
            commands.insert_resource(NextState(GameState::Paused))
        } else {
            println!("Entering running state");
            commands.insert_resource(NextState(GameState::Running))
        }
    }
}

/*
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
        //base_color: Color::BLACK,
        //emissive: Color::rgb(0.1, 0.1, 0.2),
        reflectance: 0.0,
        metallic: 1.0,
        base_color_texture: Some(asset_server.load("images/earth.png")),
        normal_map_texture: Some(asset_server.load("images/2k_earth_normal_map.png")),
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
 */

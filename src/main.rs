use bevy::{app::AppExit, pbr::AmbientLight, prelude::*};
use bevy_egui::{
    egui::{self, Label},
    EguiContext, EguiPlugin,
};
use bevy_inspector_egui::WorldInspectorPlugin;
use iyes_loopless::prelude::*;
use planets::SystemPlugin;

use bevy_kira_audio::AudioPlugin;
use orbit::OrbitPlugin;

mod orbit;

mod tests;

mod controls;
mod planets;

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
        .add_startup_system(enable_hot_reloading)
        .add_loopless_state(GameState::Loading)
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1.0 / 5.0f32,
        })
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(controls::ControlPlugin)
        .add_plugin(AudioPlugin)
        .add_plugin(OrbitPlugin)
        .add_enter_system(GameState::Loading, load_assets)
        .add_system(main_menu.run_in_state(GameState::MainMenu))
        .add_system(esc_pause.run_in_state(GameState::Running))
        .add_system(esc_pause.run_in_state(GameState::Paused))
        .add_system(pause_menu.run_in_state(GameState::Paused))
        .add_system(exit_system.run_in_state(GameState::Quit))
        .add_plugin(SystemPlugin)
        .run();
}

fn load_assets(mut commands: Commands) {
    commands.insert_resource(NextState(GameState::MainMenu));
}

fn enable_hot_reloading(asset_server: Res<AssetServer>) {
    asset_server.watch_for_changes().unwrap()
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

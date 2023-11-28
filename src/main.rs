use bevy::{app::AppExit, ecs::schedule::ScheduleLabel, pbr::AmbientLight, prelude::*};
use bevy_egui::{
    egui::{self, Label},
    EguiContexts, EguiPlugin,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use bevy_kira_audio::AudioPlugin;

mod tests;

mod firstperson;
pub use firstperson::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, States, Default, ScheduleLabel)]
enum GameState {
    #[default]
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
            brightness: 0.2f32,
        })
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .insert_resource(Msaa::Sample4)
        .add_plugins(DefaultPlugins.set(AssetPlugin::default()))
        .add_state::<GameState>()
        .add_plugins(EguiPlugin)
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(AudioPlugin)
        .add_systems(OnEnter(GameState::Loading), load_assets)
        .add_systems(GameState::MainMenu, main_menu)
        .add_systems(GameState::Running, esc_pause)
        .add_systems(GameState::Paused, esc_pause)
        .add_systems(GameState::Paused, pause_menu)
        .add_systems(GameState::Quit, exit_system)
        .run();
}

fn load_assets(mut state: ResMut<NextState<GameState>>) {
    state.set(GameState::MainMenu);
}

fn exit_system(mut exit: EventWriter<AppExit>) {
    exit.send(AppExit);
}

fn main_menu(mut state: ResMut<NextState<GameState>>, mut egui_context: EguiContexts) {
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
                state.set(GameState::Running);
            }

            if ui
                .add_sized([200.0, 50.0], egui::Button::new("Quit"))
                .clicked()
            {
                state.set(GameState::Quit)
            }
        });
}

fn pause_menu(mut state: ResMut<NextState<GameState>>, mut egui_context: EguiContexts) {
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
                state.set(GameState::Running);
            }

            if ui
                .add_sized([200.0, 50.0], egui::Button::new("Exit"))
                .clicked()
            {
                state.set(GameState::MainMenu);
            }
        });
}

fn esc_pause(
    mut state: ResMut<NextState<GameState>>,
    keys: Res<Input<KeyCode>>,
    current_state: Res<State<GameState>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        if current_state.get() == &GameState::Running {
            println!("Entering pause state");
            state.set(GameState::Paused);
        } else {
            println!("Entering running state");
            state.set(GameState::Running);
        }
    }
}

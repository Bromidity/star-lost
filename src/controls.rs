use bevy::prelude::*;
use bevy_mod_picking::{DefaultPickingPlugins, PickingCameraBundle, PickingEvent, SelectionEvent};
use iyes_loopless::prelude::*;
use smooth_bevy_cameras::{
    controllers::orbit::{OrbitCameraBundle, OrbitCameraController, OrbitCameraPlugin},
    LookTransformPlugin, LookTransform,
};

use crate::GameState;

pub struct ControlPlugin;

impl Plugin for ControlPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugin(LookTransformPlugin)
            .add_plugin(OrbitCameraPlugin::default())
            .add_plugins(DefaultPickingPlugins)
            .add_enter_system(GameState::Running, init_camera)
            .add_system(select_planet.run_in_state(GameState::Running));
    }
}

fn init_camera(mut commands: Commands) {
    commands
        .spawn_bundle(Camera3dBundle::default())
        .insert_bundle(OrbitCameraBundle::new(
            OrbitCameraController::default(),
            Vec3::new(0.0, 15000.0, 15000.0),
            Vec3::new(0., 0., 0.),
        ))
        .insert_bundle(PickingCameraBundle::default());
}

fn select_planet(mut events: EventReader<PickingEvent>, mut camera: Query<&mut LookTransform>, global_transform: Query<&GlobalTransform>) {
    for event in events.iter() {
        match event {
            PickingEvent::Selection(SelectionEvent::JustSelected(entity)) => {
                if let Ok(transform) = global_transform.get(*entity) {
                    camera.single_mut().target = transform.translation();
                }
            },
            _ => ()
        }
    }
}
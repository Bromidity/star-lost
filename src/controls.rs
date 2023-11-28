use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use smooth_bevy_cameras::{
    controllers::orbit::{OrbitCameraBundle, OrbitCameraController, OrbitCameraPlugin},
    LookTransformPlugin,
};

use crate::GameState;

pub struct ControlPlugin;

impl Plugin for ControlPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(LookTransformPlugin)
            .add_plugins(OrbitCameraPlugin::default())
            .add_plugins(DefaultPickingPlugins)
            .add_systems(GameState::Running, init_camera);
        //.add_system(select_planet.run_if(in_state(GameState::Running)));
    }
}

fn init_camera(mut commands: Commands) {
    println!("Init camera");

    commands
        .spawn(Camera3dBundle::default())
        .insert((OrbitCameraBundle::new(
            OrbitCameraController::default(),
            Vec3::new(0.0, 15.0, 15.0),
            Vec3::new(0., 0., 0.),
            Vec3::Y,
        ),));
}
/*
fn select_planet(
    mut events: EventReader<PickingEvent>,
    mut camera: Query<&mut LookTransform>,
    global_transform: Query<&GlobalTransform>,
) {
    for event in events.iter() {
        match event {
            PickingEvent::Selection(SelectionEvent::JustSelected(entity)) => {
                if let Ok(transform) = global_transform.get(*entity) {
                    camera.single_mut().target = transform.translation();
                }
            }
            _ => (),
        }
    }
}
 */

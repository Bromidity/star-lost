use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::render_resource::{AsBindGroup, ShaderRef},
};
use bevy_common_assets::ron::RonAssetPlugin;
use bevy_mod_picking::PickableBundle;
use serde::Deserialize;

use crate::{orbit::OrbitBundle, GameState};

pub struct SystemPlugin;

impl Plugin for SystemPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RonAssetPlugin::<LocalSystem>::new(&["system.ron"]))
            .add_plugins(RonAssetPlugin::<PlanetPreset>::new(&["planet.ron"]))
            .add_plugins(MaterialPlugin::<PlaneMaterial>::default())
            .add_systems(GameState::Running, spawn_system)
            .add_systems(OnEnter(GameState::Loading), create_planet_presets)
            .add_systems(GameState::Running, generate_planet_meshes);
    }
}

#[derive(Debug, Deserialize)]
pub enum BodyKind {
    Star,
    Planet,
}

#[derive(Debug, Deserialize)]
pub struct Body {
    pub name: String,
    pub kind: BodyKind,
    pub size: f32,
    #[serde(default)]
    pub bodies: Vec<OrbitingBody>,
}

#[derive(Debug, Deserialize)]
pub struct OrbitingBody {
    pub body: Body,
    pub period: f32,
}

#[derive(Debug, Deserialize, TypeUuid, Resource, TypePath, Asset)]
#[uuid = "261dfb57-77a5-45ca-9a06-69b2ee640a3e"]
pub struct LocalSystem {
    pub name: String,
    pub position: Vec3,
    pub center: Body,
}

#[derive(Debug, Clone, Deserialize, TypeUuid, Resource, Asset, TypePath)]
#[uuid = "3efc2596-e948-40ff-a67d-e0786c9952e4"]
pub struct PlanetPreset {
    pub color: Color,
    pub scale: f32,
    #[serde(skip)]
    pub mesh: Handle<Mesh>,
    #[serde(skip)]
    pub material: Handle<StandardMaterial>,
    #[serde(default)]
    pub loaded: bool,
}

#[derive(Resource)]
struct PlanetPresets {
    pub example: Handle<PlanetPreset>,
}

fn generate_planet_meshes(
    mut events: EventReader<AssetEvent<PlanetPreset>>,
    mut presets: ResMut<Assets<PlanetPreset>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    // This function has two call-sites, so to prevent duplication, is split into a closure.
    let mut generate_preset = |preset: &mut PlanetPreset| {
        log::debug!("PlanetPreset updated, creating/updating associated meshes and materials");
        let surface = StandardMaterial {
            base_color: preset.color,
            ..default()
        };

        let sphere = Mesh::try_from(shape::Icosphere {
            radius: preset.scale,
            subdivisions: 32,
        })
        .unwrap();

        if let Some(mesh) = meshes.get_mut(&preset.mesh) {
            *mesh = sphere;
        } else {
            preset.mesh = meshes.add(sphere);
        }

        if let Some(material) = materials.get_mut(&preset.material) {
            *material = surface;
        } else {
            preset.material = materials.add(surface);
        }

        preset.loaded = true;
    };

    for event in events.read() {
        match event {
            AssetEvent::Added { id } => {
                let preset = presets.get_mut(*id).unwrap();
                generate_preset(preset);
            }
            AssetEvent::Modified { id } => {
                if let Some(PlanetPreset { loaded: false, .. }) = presets.get(*id) {
                    let preset = presets.get_mut(*id).unwrap();
                    generate_preset(preset);
                };
            }
            _ => (),
        }
    }
}

struct SystemBuilder<'w, 's, 'a> {
    commands: Commands<'w, 's>,
    preset: PlanetPreset,
    meshes: ResMut<'a, Assets<Mesh>>,
    orbital_material: Handle<PlaneMaterial>,
}

impl<'w, 's, 'a> SystemBuilder<'w, 's, 'a> {
    pub fn system(&mut self, system: &LocalSystem) {
        self.spawn_body(&system.center, None);
    }

    pub fn spawn_body(&mut self, body: &Body, parent: Option<(Entity, f32)>) {
        let mut planet = self.commands.spawn((
            Name::new(body.name.clone()),
            PbrBundle {
                mesh: self.meshes.add(
                    Mesh::try_from(shape::Icosphere {
                        radius: body.size,
                        subdivisions: 32,
                    })
                    .unwrap(),
                ),
                material: self.preset.material.clone(),
                transform: Transform::from_scale(Vec3::splat(1.0)),
                ..default()
            },
            PickableBundle::default(),
        ));

        // This is the ring-plane
        planet.with_children(|child| {
            child.spawn(MaterialMeshBundle {
                mesh: self.meshes.add(Mesh::from(shape::Plane {
                    size: body.size * 50.0,
                    subdivisions: 0,
                })),
                material: self.orbital_material.clone(),
                ..default()
            });
        });

        if let Some((parent, distance)) = parent {
            planet.insert(OrbitBundle::body(parent, distance));
        }

        let planet_id = planet.id();

        for child in body.bodies.iter() {
            self.spawn_body(&child.body, Some((planet_id, child.period)));
        }
    }
}

#[derive(Resource)]
struct Universe {
    pub systems: Vec<Handle<LocalSystem>>,
}

pub fn create_planet_presets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut orbital_material: ResMut<Assets<PlaneMaterial>>,
) {
    commands.insert_resource(PlanetPresets {
        example: asset_server.load("planets/example.planet.ron"),
    });

    commands.insert_resource(Universe {
        systems: vec![asset_server.load("systems/solar.system.ron")],
    });

    commands.insert_resource(PlaneMaterialHandle(orbital_material.add(PlaneMaterial {
        color: Color::rgba(0.5, 0.1, 0.1, 0.2),
    })));
}

fn spawn_system(
    commands: Commands,
    presets: Res<PlanetPresets>,
    preset_assets: Res<Assets<PlanetPreset>>,
    universe: Res<Universe>,
    systems: Res<Assets<LocalSystem>>,
    meshes: ResMut<Assets<Mesh>>,
    plane_material: Res<PlaneMaterialHandle>,
) {
    let preset = preset_assets.get(&presets.example).unwrap().to_owned();

    let mut system_builder = SystemBuilder {
        commands,
        preset,
        meshes,
        orbital_material: plane_material.0.clone(),
    };

    for system in universe.systems.iter() {
        let system = systems.get(system).unwrap();

        system_builder.system(system);
    }
}

#[derive(Resource)]
pub struct PlaneMaterialHandle(pub Handle<PlaneMaterial>);

// This is the struct that will be passed to your shader
#[derive(AsBindGroup, TypeUuid, Debug, Clone, TypePath, Asset)]
#[uuid = "2cdc9ddb-3ce2-4973-a844-5b3a9e8edc66"]
pub struct PlaneMaterial {
    #[uniform(0)]
    color: Color,
}

impl Material for PlaneMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/orbital_material.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
}

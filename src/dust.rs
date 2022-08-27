use bevy::{
    core::Name,
    math::{Vec2, Vec3, Vec4},
    prelude::*,
};
use bevy_hanabi::{
    BillboardModifier, ColorOverLifetimeModifier, EffectAsset, Gradient, HanabiPlugin,
    ParticleEffect, ParticleEffectBundle, ParticleLifetimeModifier, ParticleTextureModifier,
    PositionSphereModifier, ShapeDimension, SizeOverLifetimeModifier, Spawner, Value,
};

use crate::controls::PlayerControlled;

pub struct DustPlugin;

impl Plugin for DustPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugin(HanabiPlugin)
            .add_startup_system(create_space_dust)
            .add_system(parent_dust_emitter_to_camera);
    }
}

#[derive(Component)]
struct DustEmitter;

fn create_space_dust(
    mut commands: Commands,
    mut effects: ResMut<Assets<EffectAsset>>,
    asset_server: Res<AssetServer>,
) {
    let particle_texture: Handle<Image> = asset_server.load("images/cloud.png");

    let mut color_gradient = Gradient::new();
    color_gradient.add_key(0.0, Vec4::new(1.0, 1.0, 1.0, 0.0));
    color_gradient.add_key(0.05, Vec4::new(1.0, 1.0, 1.0, 1.0));
    color_gradient.add_key(0.95, Vec4::new(1.0, 1.0, 1.0, 1.0));
    color_gradient.add_key(1.0, Vec4::new(1.0, 1.0, 1.0, 0.0));

    let mut size_gradient = Gradient::new();
    size_gradient.add_key(0.0, Vec2::new(0.0, 0.0));
    size_gradient.add_key(0.3, Vec2::new(0.03, 0.03));
    size_gradient.add_key(1.0, Vec2::new(0.0, 0.0));

    let effect = effects.add(
        EffectAsset {
            name: "space_dust".to_string(),
            capacity: 65536,
            spawner: Spawner::rate(Value::Uniform((1500., 2000.))),
            ..Default::default()
        }
        .init(PositionSphereModifier {
            center: Vec3::ZERO,
            radius: 20.,
            dimension: ShapeDimension::Volume,
            speed: 0.0.into(),
        })
        .init(ParticleLifetimeModifier { lifetime: 10.0 })
        .render(ColorOverLifetimeModifier {
            gradient: color_gradient,
        })
        .render(SizeOverLifetimeModifier {
            gradient: size_gradient,
        })
        .render(BillboardModifier {})
        .render(ParticleTextureModifier {
            texture: particle_texture,
        }),
    );

    commands
        .spawn()
        .insert(Name::new("emit:dust"))
        .insert(DustEmitter)
        .insert_bundle(ParticleEffectBundle {
            effect: ParticleEffect::new(effect.clone()),
            ..Default::default()
        });
}

/// Parents the dust emitter to the [`PlayerControlled`] entity (generally a ship).
/// This is because the dust emitter startup system can run before or after the
/// main setup phase of the game.
fn parent_dust_emitter_to_camera(
    mut commands: Commands,
    emitter: Query<Entity, (With<DustEmitter>, Without<Parent>)>,
    camera_query: Query<Entity, With<PlayerControlled>>,
) {
    for camera_entity in camera_query.iter() {
        for emitter_entity in emitter.iter() {
            commands.entity(camera_entity).add_child(emitter_entity);
        }
    }
}

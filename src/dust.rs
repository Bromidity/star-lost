use bevy::{
    math::{Vec2, Vec3, Vec4},
    prelude::*,
};
use bevy_hanabi::{
    Attribute, ColorOverLifetimeModifier, EffectAsset, ExprWriter, Gradient, HanabiPlugin,
    ImageSampleMapping, OrientMode, OrientModifier, ParticleEffect, ParticleEffectBundle,
    ParticleTextureModifier, SetAttributeModifier, SetPositionSphereModifier, ShapeDimension,
    SizeOverLifetimeModifier, Spawner,
};

use crate::{controls::PlayerControlled, GameState};

pub struct DustPlugin;

impl Plugin for DustPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(HanabiPlugin)
            .add_systems(OnEnter(GameState::Running), create_space_dust)
            .add_systems(Update, parent_dust_emitter_to_camera);
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
    size_gradient.add_key(0.1, Vec2::new(0.01, 0.01));
    size_gradient.add_key(1.0, Vec2::new(0.0, 0.0));

    let writer = ExprWriter::new();

    let init_position = SetPositionSphereModifier {
        center: writer.lit(Vec3::ZERO).expr(),
        radius: writer.lit(10.0).expr(),
        dimension: ShapeDimension::Volume,
    };

    let size_lifetime = SizeOverLifetimeModifier {
        gradient: size_gradient,
        screen_space_size: false,
    };

    let color_lifetime = ColorOverLifetimeModifier {
        gradient: color_gradient,
    };

    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, writer.lit(10.0).expr());

    let init_velocity =
        SetAttributeModifier::new(Attribute::VELOCITY, writer.lit(Vec3::ZERO).expr());

    let effect = effects.add(
        EffectAsset::new(
            160000,
            Spawner::burst(16000.0.into(), 1.0.into()),
            writer.finish(),
        )
        .with_name("emit:dust")
        .init(init_position)
        .init(init_velocity)
        .init(init_lifetime)
        .render(color_lifetime)
        .render(size_lifetime)
        .render(OrientModifier {
            mode: OrientMode::ParallelCameraDepthPlane,
        })
        .render(ParticleTextureModifier {
            texture: particle_texture,
            sample_mapping: ImageSampleMapping::ModulateOpacityFromR,
        }),
    );

    commands.spawn((
        DustEmitter,
        ParticleEffectBundle {
            transform: Transform::from_xyz(0.0, -10.0, 0.0),
            effect: ParticleEffect::new(effect.clone()),
            ..Default::default()
        },
    ));
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

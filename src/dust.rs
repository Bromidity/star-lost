use bevy::{
    core::Name,
    math::{Vec2, Vec3, Vec4},
    prelude::{AssetServer, Assets, Commands, Handle, Image, Plugin, Res, ResMut, Transform},
};
use bevy_hanabi::{
    BillboardModifier, ColorOverLifetimeModifier, EffectAsset, Gradient, HanabiPlugin,
    ParticleEffect, ParticleEffectBundle, ParticleLifetimeModifier, ParticleTextureModifier,
    PositionSphereModifier, ShapeDimension, SizeOverLifetimeModifier, Spawner, Value,
};

pub struct DustPlugin;

impl Plugin for DustPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugin(HanabiPlugin)
            .add_startup_system(create_space_dust);
    }
}

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
    size_gradient.add_key(0.05, Vec2::new(0.01, 0.01));
    size_gradient.add_key(0.5, Vec2::new(0.1, 0.1));
    size_gradient.add_key(1.0, Vec2::new(0.0, 0.0));

    let effect = effects.add(
        EffectAsset {
            name: "space_dust".to_string(),
            capacity: 65536,
            spawner: Spawner::rate(Value::Uniform((500., 1000.))),
            ..Default::default()
        }
        .init(PositionSphereModifier {
            center: Vec3::ZERO,
            radius: 50.,
            dimension: ShapeDimension::Volume,
            speed: 0.0.into(),
        })
        .init(ParticleLifetimeModifier { lifetime: 60.0 })
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
        .insert(Name::new("emit:random1"))
        .insert_bundle(ParticleEffectBundle {
            effect: ParticleEffect::new(effect.clone()),
            transform: Transform::from_translation(Vec3::new(35.0, 11.0, 35.0))
                .looking_at(Vec3::new(35.0, 8.0, 35.0), Vec3::Y),
            ..Default::default()
        });
}

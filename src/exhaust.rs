use bevy::{
    app::{Plugin, Startup, Update},
    asset::{AssetServer, Assets, Handle},
    core::Name,
    ecs::{
        entity::Entity,
        query::{Added, With},
        schedule::{common_conditions::in_state, IntoSystemConfigs},
        system::{Commands, Query, Res, ResMut, Resource},
    },
    hierarchy::{BuildChildren, Parent},
    log::info,
    math::{Vec2, Vec3, Vec4},
    render::texture::Image,
    transform::components::Transform,
};
use bevy_hanabi::{
    Attribute, ColorOverLifetimeModifier, CompiledParticleEffect, EffectAsset, ExprWriter,
    Gradient, ImageSampleMapping, OrientMode, OrientModifier, ParticleEffect, ParticleEffectBundle,
    ParticleTextureModifier, SetAttributeModifier, SetPositionSphereModifier,
    SetVelocityTangentModifier, ShapeDimension, SizeOverLifetimeModifier, Spawner,
};
use log::debug;

use crate::{physics::Acceleration, thrust::AnimatedThruster, GameState};

#[derive(Default, Clone, Resource)]
struct ExhaustEffect {
    handle: Handle<EffectAsset>,
}

pub struct ExhaustPlugin;

impl Plugin for ExhaustPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(ExhaustEffect::default())
            .add_systems(Startup, create_exhaust_effect)
            .add_systems(
                Update,
                associate_exhaust_effect_with_thruster.run_if(in_state(GameState::Running)),
            );
    }
}

fn create_exhaust_effect(
    mut effects: ResMut<Assets<EffectAsset>>,
    mut exhaust_handle: ResMut<ExhaustEffect>,
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

    let init_position =
        SetAttributeModifier::new(Attribute::POSITION, writer.lit(Vec3::ZERO).expr());

    let size_lifetime = SizeOverLifetimeModifier {
        gradient: size_gradient,
        screen_space_size: false,
    };

    let color_lifetime = ColorOverLifetimeModifier {
        gradient: color_gradient,
    };

    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, writer.lit(10.0).expr());

    let init_velocity = SetAttributeModifier::new(Attribute::VELOCITY, writer.lit(-Vec3::Z).expr());

    let effect = EffectAsset::new(100, Spawner::rate(10.0.into()), writer.finish())
        .with_name("emit:exhaust")
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
        })
        .with_simulation_space(bevy_hanabi::SimulationSpace::Global);

    info!("Exhaust effect created: {:#?}", effect.properties());
    exhaust_handle.handle = effects.add(effect);
}

/// Listens for Added<Name> events where the object is called 'thrust_rear'
/// and tags them for thrust animation.
fn associate_exhaust_effect_with_thruster(
    mut commands: Commands,
    exhaust: Res<ExhaustEffect>,
    parents: Query<(Option<&Parent>, Option<&Acceleration>)>,
    potential_thrusters: Query<(Entity, &Transform, &Name, &Parent), Added<Name>>,
) {
    fn find_simulated_parent(
        parents: &Query<(Option<&Parent>, Option<&Acceleration>)>,
        entity: Entity,
    ) -> Option<Entity> {
        if let Ok((parent, acceleration)) = parents.get(entity) {
            if acceleration.is_some() {
                Some(entity)
            } else {
                parent.and_then(|p| find_simulated_parent(parents, p.get()))
            }
        } else {
            None
        }
    }

    for (thruster, transform, name, parent) in potential_thrusters.iter() {
        if name.as_str().contains("anim_thrust_z_neg") {
            if let Some(ship) = find_simulated_parent(&parents, parent.get()) {
                debug!(
                    "inserting ParticleEmitter component for '{}'",
                    name.as_str()
                );

                let effect = commands
                    .spawn(ParticleEffectBundle {
                        effect: ParticleEffect::new(exhaust.handle.clone()),
                        ..Default::default()
                    })
                    .id();

                commands.entity(ship).add_child(effect);
            }
        }
    }
}

/*
/// Animates thrusters
fn animate_thruster_system(
    time: Res<Time>,
    mut thrusters: Query<(&mut Transform, &AnimatedThruster)>,
) {
    for (mut transform, thrust) in thrusters.iter_mut() {
        // Do some funky wiggling to make it look cooler.
        transform.rotation = Quat::from_euler(
            EulerRot::XYZ,
            time.delta_seconds() * 100.0 % THRUST_WIGGLE_MULTIPLIER
                - THRUST_WIGGLE_MULTIPLIER / 2.0,
            time.delta_seconds() * 200.0 % THRUST_WIGGLE_MULTIPLIER
                - THRUST_WIGGLE_MULTIPLIER / 2.0,
            time.delta_seconds() * 300.0 % THRUST_WIGGLE_MULTIPLIER
                - THRUST_WIGGLE_MULTIPLIER / 2.0,
        );

        transform.scale = thrust.initial_scale + (thrust.initial_scale * thrust.scale)
            - thrust.initial_scale * thrust.scale * thrust.thrust * THRUST_SCALE_MULTIPLIER;
    }
}

*/

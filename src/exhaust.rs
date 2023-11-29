use bevy::{
    app::{Plugin, Startup, Update, FixedUpdate},
    asset::{AssetServer, Assets, Handle},
    core::Name,
    ecs::{
        component::Component,
        entity::Entity,
        query::Added,
        schedule::{common_conditions::in_state, IntoSystemConfigs},
        system::{Commands, Query, Res, ResMut, Resource},
    },
    hierarchy::{BuildChildren, Parent},
    log::info,
    math::{Vec2, Vec3, Vec4},
    render::texture::Image,
    transform::components::{GlobalTransform, Transform}, time::Time,
};
use bevy_hanabi::{
    Attribute, ColorOverLifetimeModifier, CompiledParticleEffect, EffectAsset, ExprWriter,
    Gradient, ImageSampleMapping, OrientMode, OrientModifier, ParticleEffect, ParticleEffectBundle,
    ParticleTextureModifier, SetAttributeModifier, SizeOverLifetimeModifier, Spawner, accel, EffectSpawner,
};
use log::debug;

use crate::{
    physics::{Acceleration, Velocity},
    GameState,
};

#[derive(Clone, Component)]
pub struct ExhaustVelocity {
    parent_entity: Entity,
}

#[derive(Default, Clone, Resource)]
struct ExhaustEffect {
    handle: Handle<EffectAsset>,
}

pub struct ExhaustPlugin;

impl Plugin for ExhaustPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(ExhaustEffect::default())
            .add_systems(Startup, create_exhaust_effect)
            .add_systems(Update, update_exhaust_velocity)
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
    color_gradient.add_key(0.0, Vec4::new(0.2, 0.2, 1.0, 0.0));
    color_gradient.add_key(0.1, Vec4::new(0.2, 0.2, 1.0, 0.8));
    color_gradient.add_key(0.3, Vec4::new(1.0, 0.4, 0.4, 0.5));
    color_gradient.add_key(1.0, Vec4::new(1.0, 1.0, 1.0, 0.0));

    let mut size_gradient = Gradient::new();
    size_gradient.add_key(0.0, Vec2::new(0.03, 0.03));
    size_gradient.add_key(0.8, Vec2::new(0.1, 0.1));
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

    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, writer.lit(1.0).expr());

    let exhaust_velocity = writer.prop("exhaust_velocity").expr();
    let init_velocity = SetAttributeModifier::new(Attribute::VELOCITY, exhaust_velocity);

    let effect = EffectAsset::new(100000, Spawner::rate(2500.0.into()).with_starts_active(false), writer.finish())
        .with_name("emit:exhaust")
        .with_property("exhaust_velocity", Vec3::ZERO.into())
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
        });

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
                    .spawn((
                        ParticleEffectBundle {
                            effect: ParticleEffect::new(exhaust.handle.clone()),
                            transform: Transform::from_translation(transform.translation),
                            ..Default::default()
                        },
                        ExhaustVelocity {
                            parent_entity: ship,
                        },
                    ))
                    .id();

                commands.entity(ship).add_child(effect);
            }
        }
    }
}

fn update_exhaust_velocity(
    time: Res<Time>,
    simulated_entities: Query<(&GlobalTransform, &Velocity, &Acceleration)>,
    mut query: Query<(&mut CompiledParticleEffect, &mut EffectSpawner, &ExhaustVelocity)>,
) {
    for (mut compiled, mut spawner, exhaust) in query.iter_mut() {
        if let Ok((transform, velocity, acceleration)) = simulated_entities.get(exhaust.parent_entity) {
            if acceleration.length_squared() > 0.1 {
                let velocity = -transform.forward() * 2.0 + transform.right() * (time.elapsed_seconds_wrapped() % 0.01 - 0.005) * 20.0;
                compiled.set_property("exhaust_velocity", velocity.into());
                spawner.set_active(true);
            } else {
                spawner.set_active(false);
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

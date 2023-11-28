use bevy::{ecs::system::EntityCommands, prelude::*};
use std::{collections::HashMap, fmt::Write, marker::PhantomData, ops::Deref};

use crate::ui::FollowObject;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DebugValueAddedEvent>()
            .add_stage_after(PostUpdate, "debug")
            .add_system_to_stage(
                "debug",
                handle_debug_value_event.label("debug_value_event_handler"),
            )
            .add_system_to_stage(
                "debug",
                refresh_rendered_debug_window.label("update_debug_window"),
            )
            .add_system_to_stage(
                "debug",
                remove_unused_debug_windows.after("update_debug_window"),
            );
    }
}

/// Indicates that an a Debug vector should be drawn based on this entity's T-component
/// Can for instance be attached to an instance as a `DebugVector<Acceleration>` to
/// have an arrow drawn in 3D space to indicate a the value of a ship's [Acceleration](crate::physics::Acceleration) component.
#[derive(Component)]
pub struct DebugVector<T: Component> {
    _data: PhantomData<T>,
}

impl<T: Component> Default for DebugVector<T> {
    fn default() -> Self {
        Self {
            _data: PhantomData::default(),
        }
    }
}

/// Utility trait for attaching a [DebugVector] to an entity
pub trait AddDebugArrow {
    fn debug_vector<T: Component + Deref<Target = Vec3>>(
        &mut self,
        _asset_server: &Res<AssetServer>,
    ) -> &mut Self;
}

impl AddDebugArrow for EntityCommands<'_, '_, '_> {
    fn debug_vector<T: Component + Deref<Target = Vec3>>(
        &mut self,
        asset_server: &Res<AssetServer>,
    ) -> &mut Self {
        self.with_children(|parent| {
            parent
                .spawn_bundle((
                    Transform::default(),
                    GlobalTransform::default(),
                    DebugVector::<T>::default(),
                ))
                .with_children(|arrow| {
                    arrow.spawn_bundle(SceneBundle {
                        scene: asset_server.load("models/arrow.glb#Scene0"),
                        ..Default::default()
                    });
                });
        })
    }
}

/// Responsible for updating the [Transform] of [DebugVector]s to reflect their reflected values
pub fn debug_vector_system<T: Component + Deref<Target = Vec3>>(
    mut query: Query<(&mut Transform, &Parent), With<DebugVector<T>>>,
    value: Query<(&GlobalTransform, &T), Without<DebugVector<T>>>,
) {
    for (mut transform, parent) in query.iter_mut() {
        if let Ok((global, debugged_value)) = value.get(parent.get()) {
            transform.look_at(**debugged_value, Vec3::Y);
            transform.rotation *= global.to_scale_rotation_translation().1.inverse();
            transform.scale = Vec3::from_slice(&[1.0, debugged_value.length() * 4.0, 1.0]);
        }
    }
}

/// Plugin for initializing the systems required to populate [DebugWindow]s with information
/// from a specific T-component. This must be called for every [Component] which might need to
/// be debugged in the future, otherwise the systems responsible for recording the value into
/// the [DebugWindow] won't be instantiated ([update_debug_window_with_value_system], [emit_debug_value_added_event_system]),
/// and therefore won't ever be displayed. See [PhysicsPlugin](crate::physics::PhysicsPlugin) for an example of how to make components debuggable.
#[derive(Default)]
pub struct DebuggableValue<T: Component + std::fmt::Debug>(PhantomData<T>);

impl<T: Component + std::fmt::Debug> Plugin for DebuggableValue<T> {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(
            "debug",
            emit_debug_value_added_event_system::<T>.before("debug_value_event_handler"),
        )
        .add_system_to_stage(
            "debug",
            update_debug_window_with_value_system::<T>.before("update_debug_window"),
        );
    }
}

/// Component for storing information meant to be rendered on top of an [Entity] in 3D space
#[derive(Debug, Component)]
pub struct DebugWindow {
    /// The reason I'm doing this instead of [Parent]ing directly to the targeted entity
    /// is because the [DebugWindow] and UI-component belong to the same entity and
    /// mixing 3D elements (the parent entity) and 2D elements (the UI component)
    /// within the same hierarchy is a generally a bad idea dn concretely very confusing
    /// for Bevy, causing it to emit a WARNing
    pub parent: Entity,
    pub values: HashMap<&'static str, String>,
}

impl DebugWindow {
    pub fn showing_values_for(entity: Entity) -> DebugWindow {
        DebugWindow {
            parent: entity,
            values: HashMap::new(),
        }
    }
}

/// When attached to a 3D entity, ensures the value of the entity's T-component
/// is captured in a [DebugWindow] attached to the entity's world-to-screen-space
/// coordinate.
#[derive(Component)]
pub struct DebugValue<T: Component> {
    label: &'static str,
    _data: PhantomData<T>,
}

impl<T: Component> DebugValue<T> {
    pub fn with_label(label: &'static str) -> Self {
        DebugValue {
            label,
            _data: PhantomData::default(),
        }
    }
}

/// Updates the UI-element displaying the [DebugWindow] information with information
/// from the [DebugWindow] component.
pub fn refresh_rendered_debug_window(mut query: Query<(&mut Text, &DebugWindow)>) {
    for (mut text, window) in query.iter_mut() {
        let mut block = String::new();
        for (k, v) in window.values.iter() {
            writeln!(block, "{}: {}", k, v.as_str()).ok();
        }
        text.sections[0].value = block;
    }
}

/// Reads the T-component from the entity and writes its value into the entity's [DebugWindow]
pub fn update_debug_window_with_value_system<T: Component + std::fmt::Debug>(
    mut window_query: Query<&mut DebugWindow>,
    values_query: Query<(&DebugValue<T>, &T)>,
) {
    for mut window in window_query.iter_mut() {
        if let Ok((label, value)) = values_query.get(window.parent) {
            let value = format!("{:#?}", value);
            window.values.insert(label.label, value);
        }
    }
}

/// Emitted whenever a [DebugValue] is added to an entity.
/// Captured by the [handle_debug_value_event] system in order to make sure a [DebugWindow] is attached to the entity in question.
struct DebugValueAddedEvent(Entity);

/// Utility structure added to a 3D entity to link it to its 2D UI-element/[DebugWindow] entity
#[derive(Component)]
pub struct HasDebugWindow(pub Entity);

/// Checks if a [DebugWindow] already exists for the 3D entity which just had a [DebugValue] added, otherwise creates one for it.
fn handle_debug_value_event(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut events: EventReader<DebugValueAddedEvent>,
    query: Query<Entity, With<HasDebugWindow>>,
) {
    let font = asset_server.load("fonts/DejaVuSansMono.ttf");
    let mut just_added = Vec::new();
    for DebugValueAddedEvent(entity) in events.iter() {
        if !just_added.contains(entity) && query.get(*entity).is_err() {
            info!("creating DebugWindow for {:?}", entity);

            let debug_window = commands
                .spawn_bundle(TextBundle {
                    style: Style {
                        align_self: AlignSelf::FlexEnd,
                        position_type: PositionType::Absolute,
                        position: UiRect {
                            bottom: Val::Px(5.0),
                            left: Val::Px(15.0),
                            ..Default::default()
                        },
                        size: Size {
                            width: Val::Px(400.0),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    text: Text::from_section(
                        "EMTPY DEBUG WINDOW".to_string(),
                        TextStyle {
                            font: font.clone(),
                            font_size: 12.0,
                            color: Color::WHITE,
                        },
                    )
                    .with_alignment(TextAlignment {
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .insert(DebugWindow::showing_values_for(*entity))
                .insert(FollowObject(*entity))
                .id();

            commands
                .entity(*entity)
                .insert(HasDebugWindow(debug_window));
            just_added.push(*entity);
        }
    }
}

/// Emits a [DebugValueAddedEvent] whenever a [DebugValue] is added to an entity.
fn emit_debug_value_added_event_system<T: Component>(
    query: Query<Entity, (Without<HasDebugWindow>, Added<DebugValue<T>>)>,
    mut events: EventWriter<DebugValueAddedEvent>,
) {
    for entity in query.iter() {
        events.send(DebugValueAddedEvent(entity));
    }
}

/// Run as one of the last systems in the `debug` stage, deleting any [DebugWindow]s which do not contain
/// any [DebugValue]s. Acts as automatic clean-up of unused [DebugWindow]s.
fn remove_unused_debug_windows(
    mut commands: Commands,
    window_query: Query<(Entity, &DebugWindow)>,
) {
    for (entity, window) in window_query.iter() {
        if window.values.is_empty() {
            info!("removing unused DebugWindow for {:?}", window.parent);
            commands.entity(window.parent).remove::<HasDebugWindow>();
            commands.entity(entity).despawn_recursive();
        }
    }
}

/// Utility extension for [EntityCommands] for easily adding a [DebugValue] to an entity.
pub trait AddDebugValue {
    fn debug_value<T: Component + std::fmt::Debug>(&mut self, label: &'static str) -> &mut Self;
}

impl AddDebugValue for EntityCommands<'_, '_, '_> {
    fn debug_value<T: Component + std::fmt::Debug>(&mut self, label: &'static str) -> &mut Self {
        self.insert(DebugValue::<T>::with_label(label))
    }
}

use bevy::{ecs::system::EntityCommands, prelude::*};
use std::{collections::HashMap, marker::PhantomData, ops::Deref};

use crate::ui::FollowObject;

#[derive(Component)]
pub struct DebugArrow<T: Component> {
    _data: PhantomData<T>,
}

impl<T: Component> Default for DebugArrow<T> {
    fn default() -> Self {
        Self {
            _data: PhantomData::default(),
        }
    }
}

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
                    DebugArrow::<T>::default(),
                ))
                .with_children(|arrow| {
                    arrow.spawn_scene(asset_server.load("models/arrow.glb#Scene0"));
                });
        })
    }
}

pub fn debug_arrow_system<T: Component + Deref<Target = Vec3>>(
    mut query: Query<(&mut Transform, &Parent), With<DebugArrow<T>>>,
    value: Query<&T>,
) {
    for (mut transform, parent) in query.iter_mut() {
        if let Ok(debugged_value) = value.get_component::<T>(parent.0) {
            transform.rotation = Quat::from_rotation_arc(Vec3::X, debugged_value.normalize());
            transform.scale = Vec3::from_slice(&[1.0, debugged_value.length() * 2.0, 1.0]);
        }
    }
}

#[derive(Default)]
pub struct DebuggableValue<T: Component + std::fmt::Debug>(PhantomData<T>);

impl<T: Component + std::fmt::Debug> Plugin for DebuggableValue<T> {
    fn build(&self, app: &mut App) {
        app.add_system(add_debug_window_if_necessary_system::<T>)
            .add_system(update_debug_window_with_value_system::<T>);
    }
}

#[derive(Debug, Component)]
pub struct DebugWindow {
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

#[derive(Component)]
pub struct DebugValue<T: Component> {
    label: &'static str,
    _data: PhantomData<T>,
}

impl<T: Component> DebugValue<T> {
    pub fn with_label(label: &'static str) -> Self {
        DebugValue {
            label: label,
            _data: PhantomData::default(),
        }
    }
}

pub fn refresh_rendered_debug_window(mut query: Query<(&mut Text, &DebugWindow)>) {
    for (mut text, window) in query.iter_mut() {
        let mut block = String::new();
        for (k, v) in window.values.iter() {
            block += &format!("{}: {}\n", k, v.as_str());
        }
        text.sections[0].value = block;
    }
}

pub fn update_debug_window_with_value_system<T: Component + std::fmt::Debug>(
    mut window_query: Query<&mut DebugWindow>,
    values_query: Query<(&DebugValue<T>, &T)>,
) {
    for mut window in window_query.iter_mut() {
        for (label, value) in values_query.get(window.parent) {
            let value = format!("{:#?}", value);
            window.values.insert(label.label, value);
        }
    }
}

pub fn add_debug_window_if_necessary_system<T: Component>(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<Entity, (Without<DebugWindow>, Added<DebugValue<T>>)>,
) {
    let font = asset_server.load("fonts/DejaVuSansMono.ttf");
    for entity in query.iter() {
        commands
            .spawn_bundle(TextBundle {
                style: Style {
                    align_self: AlignSelf::FlexEnd,
                    position_type: PositionType::Absolute,
                    position: Rect {
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
                text: Text::with_section(
                    "EMTPY DEBUG WINDOW".to_string(),
                    TextStyle {
                        font: font.clone(),
                        font_size: 12.0,
                        color: Color::WHITE,
                    },
                    TextAlignment {
                        ..Default::default()
                    },
                ),
                ..Default::default()
            })
            .insert(DebugWindow::showing_values_for(entity))
            .insert(FollowObject(entity));
    }
}

pub trait AddDebugValue {
    fn debug_value<T: Component + std::fmt::Debug>(&mut self, label: &'static str) -> &mut Self;
}

impl AddDebugValue for EntityCommands<'_, '_, '_> {
    fn debug_value<T: Component + std::fmt::Debug>(&mut self, label: &'static str) -> &mut Self {
        self.insert(DebugValue::<T>::with_label(label))
    }
}

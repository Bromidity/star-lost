use bevy::prelude::*;

use crate::tracking::{Target, TargetEntity};

pub struct RoutePlugin;

impl Plugin for RoutePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(point_travelling_system);
    }
}

#[derive(Component)]
pub struct Route {
    pub current_waypoint: usize,
    pub waypoints: Vec<Waypoint>,
}

pub enum Waypoint {
    TargetEntity(Entity),
    Target(Vec3),
}

impl From<Vec3> for Waypoint {
    fn from(vec: Vec3) -> Self {
        Waypoint::Target(vec)
    }
}

impl From<Entity> for Waypoint {
    fn from(ent: Entity) -> Self {
        Waypoint::TargetEntity(ent)
    }
}

pub fn point_travelling_system(
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &Route,
        Option<&mut Target>,
        Option<&mut TargetEntity>,
    )>,
) {
    for (entity, route, current_target, current_target_entity) in query.iter_mut() {
        match route.waypoints[route.current_waypoint] {
            Waypoint::TargetEntity(waypoint_target) => {
                if let Some(mut target_entity) = current_target_entity {
                    *target_entity = TargetEntity(waypoint_target);
                } else {
                    commands
                        .entity(entity)
                        .insert(TargetEntity(waypoint_target));
                }
            }
            Waypoint::Target(waypoint_position) => {
                if let Some(mut target) = current_target {
                    *target = Target(waypoint_position);
                } else {
                    commands.entity(entity).insert(Target(waypoint_position));
                }

                // Remove any TargetEntity if it exists, since it will screw
                // up everything.
                if current_target_entity.is_some() {
                    commands.entity(entity).remove::<TargetEntity>();
                }
            }
        }
    }
}

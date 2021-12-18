use bevy::prelude::*;

use crate::{
    debug::DebuggableValue,
    tracking::{Target, TargetEntity},
};

pub struct RoutePlugin;

impl Plugin for RoutePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(point_travelling_system)
            .add_system(point_reached_system)
            .add_plugin(DebuggableValue::<Route>::default());
    }
}

#[derive(Debug, Default, Component)]
pub struct Route {
    current_waypoint: usize,
    pub waypoints: Vec<Waypoint>,
}

impl Route {
    pub fn next(&mut self) {
        self.current_waypoint = (self.current_waypoint + 1) % self.waypoints.len();
    }

    pub fn set_waypoint(&mut self, id: usize) {
        self.current_waypoint = id % self.waypoints.len();
    }
}

impl From<Vec<Waypoint>> for Route {
    fn from(waypoints: Vec<Waypoint>) -> Self {
        Self {
            current_waypoint: 0,
            waypoints,
        }
    }
}

#[derive(Debug, Clone)]
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

pub fn point_reached_system(mut query: Query<(&mut Route, &Transform, &Target)>) {
    for (mut route, transform, target) in query.iter_mut() {
        if (transform.translation - target.0).length_squared() < 20.0 {
            route.next();
        }
    }
}

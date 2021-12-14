use bevy::{prelude::*, render::camera::Camera};

use crate::WorldCamera;

#[derive(Component)]
pub struct FollowObject(pub Entity);

pub fn follow_object_system(
    windows: Res<Windows>,
    mut query: Query<(&mut Style, &CalculatedSize, &FollowObject)>,
    object_query: Query<&Transform>,
    camera_query: Query<(&Camera, &GlobalTransform), With<WorldCamera>>,
) {
    for (camera, cam_trans) in camera_query.iter() {
        for (mut style, size, follow) in query.iter_mut() {
            if let Ok(world_pos) = object_query.get(follow.0) {
                if let Some(screen_pos) =
                    camera.world_to_screen(&windows, cam_trans, world_pos.translation)
                {
                    style.position.left = Val::Px(screen_pos.x);
                    style.position.bottom = Val::Px(screen_pos.y - size.size.height);
                }
            }
        }
    }
}

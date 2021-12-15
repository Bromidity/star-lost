use bevy::{prelude::*, render::camera::Camera};

/// Marks an entity as *the* 3D World camera. This is used by the
/// [follow_object_system](ui::follow_object_system) to translate
/// an entity's 3D world position into screen space.
#[derive(Component)]
pub struct WorldCamera;

#[derive(Component)]
pub struct FollowObject(pub Entity);

/// UI entities with a FollowObject component will have their [Style] component updated
/// to track the world-to-screen-position of the followed object.
/// The world 3D camera is required to have the [WorldCamera] tag, in order for the system
/// to deduce how to translate between world and screen coordinates.
/// Having multiple camera entities with the [WorldCamera] tag is not well defined.
pub fn follow_object_system(
    windows: Res<Windows>,
    mut query: Query<(&mut Style, &CalculatedSize, &FollowObject)>,
    object_query: Query<&Transform>,
    camera_query: Query<(&Camera, &GlobalTransform), With<WorldCamera>>,
) {
    if let Some((camera, cam_trans)) = camera_query.iter().next() {
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

use bevy::prelude::*;
use pi_engine_shell::prelude::ActionList;
use pi_scene_math::Vector3;

use crate::{
    prelude::{
        CameraFov, CameraNearFar, CameraParam, CameraTarget, EFixedMode, EFreeCameraMode,
        SceneBoundingPool, WorldMatrix,
    },
    viewer::prelude::ViewerTransformMatrix,
};

pub struct RayTest(Entity, Entity, f32, f32);
pub type ActionListRayTest = ActionList<RayTest>;

pub struct PluginRayTest;
impl Plugin for PluginRayTest {
    fn build(&self, app: &mut App) {
        app.insert_resource(ActionListRayTest::default());
    }
}

pub fn sys_ray_test(
    mut rays: ResMut<ActionListRayTest>,
    transforms: Query<&WorldMatrix>,
    // mut viewers: Query<&ViewerTransformMatrix>,
    scenes: Query<(&SceneBoundingPool, &ViewerTransformMatrix)>,
) {
    rays.drain()
        .drain(..)
        .for_each(|RayTest(scene_id, entity, x, y)| {
            if let Ok(world_matrix) = transforms.get(entity) {
                let x = (x - 0.5) * 2.;
                let y = (y - 0.5) * 2.;
                let org = Vector3::from(world_matrix.0.fixed_view::<3, 1>(0, 3));

                if let Ok((pool, viewer)) = scenes.get(scene_id) {
                    let pos = if let Some(inverse) = viewer.0.try_inverse() {
                        inverse.transform_vector(&Vector3::new(x, y, 0.))
                    } else {
                        viewer.0.transform_vector(&Vector3::new(x, y, 0.))
                    };
                    let dir = pos - org;
                    let mut result = None;
                    pool.ray_test(org, dir, &mut result);
                }
            }
        });
}

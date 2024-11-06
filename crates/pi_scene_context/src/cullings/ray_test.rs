use pi_scene_shell::prelude::*;

use crate::{flags::GlobalEnable, prelude::RenderQueueSortParam};

use super::base::{PiRay, PickResult, SceneColliderPool, SceneBoundingPool};

pub fn ray_cast(
    scenes: &Query<(&SceneColliderPool, &SceneBoundingPool)>,
    ray: &PiRay,
    idscene: Entity,
    check_bounding_while_no_collider: bool,
    sortparams: &Query<(&RenderQueueSortParam, &GlobalEnable)>,
) -> Option<PickResult> {
    let mut result = None;
    if let Ok((colliderpool, boundingpool)) = scenes.get(idscene) {
        colliderpool.ray_test(ray, &mut result, sortparams);
        if result.is_none() && check_bounding_while_no_collider {
            boundingpool.ray_test(ray, &mut result, sortparams);
            if let Some(result) = &mut result {
                result.bybounding = true;
            }
        }
    }
    result
}

pub fn ray_cast_scene(
    scene: (&SceneColliderPool, &SceneBoundingPool),
    ray: &PiRay,
    check_bounding_while_no_collider: bool,
    sortparams: &Query<(&RenderQueueSortParam, &GlobalEnable)>,
) -> Option<PickResult> {
    let mut result = None;
    let (colliderpool, boundingpool) = scene;
    colliderpool.ray_test(ray, &mut result, sortparams);
    if result.is_none() && check_bounding_while_no_collider {
        boundingpool.ray_test(ray, &mut result, sortparams);
        if let Some(result) = &mut result {
            result.bybounding = true;
        }
    }
    result
}

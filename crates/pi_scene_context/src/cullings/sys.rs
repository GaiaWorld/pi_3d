use pi_ecs::prelude::{Query};
use pi_ecs_macros::setup;
use pi_engine_shell::run_stage::TSystemStageInfo;
use pi_scene_math::{frustum::FrustumPlanes};

use crate::{cullings::{BoundingInfo}, object::GameObject, flags::SceneID, viewer::ViewerTransformMatrix};


pub struct SysCameraCulling;
impl TSystemStageInfo for SysCameraCulling {

}
#[setup]
impl SysCameraCulling {
    #[system]
    pub fn tick(
        cameras: Query<GameObject, (&ViewerTransformMatrix, &SceneID)>,
        mut objects: Query<GameObject, (&BoundingInfo, &SceneID)>,
    ) {
        //  log::debug!("Scene Camera Culling:");
        cameras.iter().for_each(|camera| {
            let mut frustum_planes = FrustumPlanes::default();
            frustum_planes.from_transform_matrix(&camera.0.0);
            objects.iter().for_each(|object| {
                object.0.is_in_frustum(&frustum_planes);
            });
        });
    }
}

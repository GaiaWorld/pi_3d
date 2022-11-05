use pi_ecs::prelude::{Query};
use pi_ecs_macros::setup;
use pi_scene_math::{frustum::FrustumPlanes};

use crate::{cullings::{BoundingInfo}, object::GameObject, cameras::camera::{CameraRenderData, CameraTransformMatrix}, flags::SceneID};


pub struct SysCameraCulling;
#[setup]
impl SysCameraCulling {
    #[system]
    pub fn tick(
        cameras: Query<GameObject, (&CameraTransformMatrix, &SceneID)>,
        mut objects: Query<GameObject, (&BoundingInfo, &SceneID)>,
    ) {
        println!("Scene Camera Culling:");
        cameras.iter().for_each(|camera| {
            let mut frustum_planes = FrustumPlanes::default();
            frustum_planes.from_transform_matrix(&camera.0.0);
            objects.iter().for_each(|object| {
                object.0.is_in_frustum(&frustum_planes);
            });
        });
    }
}
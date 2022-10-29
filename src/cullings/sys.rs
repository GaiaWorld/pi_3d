use pi_ecs::prelude::{Query, ResMut};
use pi_ecs_macros::setup;
use pi_render::rhi::{device::RenderDevice, dyn_uniform_buffer::DynUniformBuffer};
use pi_scene_math::{frustum::FrustumPlanes};
use render_data_container::GeometryBufferPool;

use crate::{meshes::Mesh, cullings::{BoundingInfo, bounding::check_boundings}, object::GameObject, shaders::*, environment::{fog::SceneFog, ambient_light::AmbientLight}, cameras::camera::CameraRenderData, flags::SceneID};


pub struct SysCameraCulling;
#[setup]
impl SysCameraCulling {
    #[system]
    pub fn tick(
        cameras: Query<GameObject, (&CameraRenderData, &SceneID)>,
        mut objects: Query<GameObject, (&BoundingInfo, &SceneID)>,
    ) {
        println!("Scene Camera Culling:");
        cameras.iter().for_each(|camera| {
            let mut frustum_planes = FrustumPlanes::default();
            frustum_planes.from_transform_matrix(&camera.0.transform_matrix);
            objects.iter().for_each(|object| {
                object.0.is_in_frustum(&frustum_planes);
            });
        });
    }
}
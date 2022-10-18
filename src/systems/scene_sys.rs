use pi_ecs::prelude::{Query, ResMut};
use pi_ecs_macros::setup;
use pi_render::rhi::{device::RenderDevice, dyn_uniform_buffer::DynUniformBuffer};
use pi_scene_math::{frustum::FrustumPlanes};
use render_data_container::GeometryBufferPool;

use crate::{scene::{SceneParam, SceneTime}, cameras::camera::Camera, meshes::Mesh, cullings::{BoundingInfo, check_boundings}, object::GameObject, shaders::*, environment::{fog::SceneFog, ambient_light::AmbientLight}};


pub fn scene_camera_culling_tick(
    camera_param: &Camera,
    boundings: &[BoundingInfo],
) -> Vec<bool> {
    println!("Scene Camera Culling:");
    let mut frustum_planes = FrustumPlanes::default();
    frustum_planes.from_transform_matrix(&camera_param.transform_matrix);
    check_boundings(boundings, &frustum_planes)
}

pub fn scene_camera_render_tick(
    device: &RenderDevice,
    scene_param: &SceneParam,
    camera_param: &Camera,
    view_rect: (u32, u32, u32, u32),
    meshes: &[Mesh],
    // geometry_pool: &GeometryBufferPool<usize>,

) {
    // let mut renderpass = device.
}

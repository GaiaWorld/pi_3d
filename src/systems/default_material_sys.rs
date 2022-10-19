use pi_ecs::{prelude::{Query, ResMut, Res}, query::With};
use pi_ecs_macros::setup;
use pi_render::rhi::{bind_group::BindGroup, dyn_uniform_buffer::DynUniformBuffer, device::RenderDevice};
use render_data_container::GeometryBufferPool;

use crate::{object::GameObject, transforms::transform_node::GlobalTransform, cameras::camera::{MainCameraBindGroup}, flags::{SceneID01, SceneCameraID01}, scene::SceneTime, environment::fog::SceneFog, materials::default_material::{DefaultMaterialMeta, DefaultMaterialPropertype}, shaders::{buildin_attributes::{BuildinAttributePosition, BuildinAttributeColor4, BuildinAttributeIndices}, VertexAttributeMeta}, geometry::SingleGeometryBufferPool};

pub struct DefaultMaterialUniformTickUpdate;
#[setup]
impl DefaultMaterialUniformTickUpdate {
    #[system]
    pub fn tick(
        mut query_materials: Query<GameObject, (&GlobalTransform, &DefaultMaterialPropertype, &mut DefaultMaterialMeta)>,
        mut dynbuffer: ResMut<DynUniformBuffer>,
    ) {
        println!("DefaultMaterial Uniform TickUpdate");
        query_materials.iter_mut().for_each(|(transform, value_bind, mut material)| {
            dynbuffer.set_uniform::<GlobalTransform>(&material.model_bind_offset, transform);
            dynbuffer.set_uniform::<DefaultMaterialPropertype>(&value_bind.bind_offset, value_bind);
        });
    }
}

pub struct DefaultMaterialTickRender;
#[setup]
impl DefaultMaterialTickRender {
    #[system]
    pub fn tick(
        query_camera: Query<GameObject, (&MainCameraBindGroup, &SceneID01, &SceneCameraID01)>,
        query_materials: Query<GameObject, (&GlobalTransform, &SceneID01, &SceneCameraID01)>,

    ) {
        for camera in query_camera.iter() {

        }
    }
}

fn render_scene_camera<'a>(
    renderpass: &mut wgpu::RenderPass<'a>,
    camera: &'a MainCameraBindGroup,
) {
    match camera.bind_group.as_ref() {
        Some(bind_group) => {
            renderpass.set_bind_group(camera.set, &bind_group, &[]);
        },
        None => {},
    }
}

fn render_scene_camera_model<'a>(
    renderpass: &mut wgpu::RenderPass<'a>,
    material: &'a DefaultMaterialMeta,
) {
    match material.bind_group.as_ref() {
        Some(bind_group) => {
            renderpass.set_bind_group(material.set, &bind_group, &[]);
        },
        None => {},
    }
}

fn render_vertex_position<'a>(
    renderpass: &mut wgpu::RenderPass<'a>,
    position: & BuildinAttributePosition,
    gbp: &'a SingleGeometryBufferPool,
) {
    match gbp.get_buffer(&position.buffer_id) {
        Some(buffer) => {
            renderpass.set_vertex_buffer(BuildinAttributePosition::SLOT, buffer.slice(..));
        },
        None => {},
    }
}

fn render_vertex_color<'a>(
    renderpass: &mut wgpu::RenderPass<'a>,
    color: & BuildinAttributeColor4,
    gbp: &'a SingleGeometryBufferPool,
) {
    match gbp.get_buffer(&color.buffer_id) {
        Some(buffer) => {
            renderpass.set_vertex_buffer(BuildinAttributeColor4::SLOT, buffer.slice(..));
        },
        None => {},
    }
}

fn render_vertex_indices<'a>(
    renderpass: &mut wgpu::RenderPass<'a>,
    indices: & BuildinAttributeIndices,
    gbp: &'a SingleGeometryBufferPool,
) {
    match gbp.get_buffer(&indices.buffer_id) {
        Some(buffer) => {
            renderpass.set_index_buffer(buffer.slice(..), indices.format);
        },
        None => {},
    }
}

fn render_draw<'a>(
    renderpass: &mut wgpu::RenderPass<'a>,
    position: & BuildinAttributePosition,
    gbp: &'a SingleGeometryBufferPool,
) {
    let vertices = gbp.get_size(&position.buffer_id) as u32;
    renderpass.draw(0..vertices, 0..1);
}

fn render_draw_indexed<'a>(
    renderpass: &mut wgpu::RenderPass<'a>,
    indices: & BuildinAttributeIndices,
    position: & BuildinAttributePosition,
    gbp: &'a SingleGeometryBufferPool,
) {
    let indices = gbp.get_size(&indices.buffer_id) as u32;
    let base_vertex = gbp.get_size(&position.buffer_id) as i32;
    renderpass.draw_indexed(0..indices, base_vertex, 0..1);
}
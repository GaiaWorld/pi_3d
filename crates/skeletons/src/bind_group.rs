use pi_ecs::{
    prelude::{Query, Res, ResMut},
    sys::system,
};
use pi_ecs_macros::setup;
use pi_render::rhi::{
    bind_group::BindGroup, bind_group_layout::BindGroupLayout, device::RenderDevice,
};

use pi_scene_context::{
    materials::{
        bind_group::{RenderBindGroup, RenderBindGroupKey, RenderBindGroupPool},
        uniform_buffer::SingleDynUnifromBufferReBindFlag,
    },
    meshes::model::BuildinModelBind,
    object::{GameObject, ObjectID},
    resources::RenderDynUniformBuffer,
    shaders::FragmentUniformBind,
};

use super::material::SkeletonsPropertype;

pub struct IDSkeletonsBindGroup(pub RenderBindGroupKey);
impl IDSkeletonsBindGroup {
    const LABEL: &'static str = "BoneTextureWidthBindGroup";
    pub const SET: u32 = 1;

    pub fn layout(device: &RenderDevice) -> BindGroupLayout {
        BindGroupLayout::from(
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some(Self::LABEL),
                entries: &[BuildinModelBind::ENTRY, SkeletonsPropertype::ENTRY],
            }),
        )
    }

    pub fn bind_group(
        device: &RenderDevice,
        group: &mut RenderBindGroup,
        dynbuffer: &RenderDynUniformBuffer,
    ) {
        group.bind_group = Some(BindGroup::from(device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                label: Some(IDSkeletonsBindGroup::LABEL),
                layout: &group.layout,
                entries: &[
                    BuildinModelBind::dyn_entry(dynbuffer),
                    SkeletonsPropertype::dyn_entry(dynbuffer),
                ],
            },
        )));
    }
}

pub struct SkeletonsBindGroupUpdate;
#[setup]
impl SkeletonsBindGroupUpdate {
    #[system]
    pub fn tick(
        device: Res<RenderDevice>,
        dynbuffer: Res<RenderDynUniformBuffer>,
        dynbuffer_flag: Res<SingleDynUnifromBufferReBindFlag>,
        mut bindgroups: ResMut<RenderBindGroupPool>,
        id: ResMut<IDSkeletonsBindGroup>,
    ) {
        println!("Sys SkyboxMaterial BindGroup Update");
        if dynbuffer_flag.0 {
            match bindgroups.get_mut(id.0) {
                Some(mut group) => {
                    IDSkeletonsBindGroup::bind_group(&device, &mut group, &dynbuffer);
                }
                None => todo!(),
            }
        }
    }
}

use pi_ecs::{prelude::{Res, Query, ResMut}, sys::system};
use pi_ecs_macros::setup;
use pi_render::rhi::{device::RenderDevice, bind_group_layout::BindGroupLayout, bind_group::BindGroup};

use pi_scene_context::{object::{ObjectID, GameObject}, meshes::model::BuildinModelBind, materials::{bind_group::{RenderBindGroup, RenderBindGroupKey, RenderBindGroupPool}, uniform_buffer::SingleDynUnifromBufferReBindFlag}, resources::RenderDynUniformBuffer, shaders::FragmentUniformBind};

use super::material::WaterMaterialPropertype;

pub struct IDWaterMaterialBindGroup(pub RenderBindGroupKey);
impl IDWaterMaterialBindGroup {
    const LABEL: &'static str = "WaterMaterialBindGroup";
    pub const SET: u32 = 1;

    pub fn layout(device: &RenderDevice) -> BindGroupLayout {
        BindGroupLayout::from(
            device.create_bind_group_layout(
                &wgpu::BindGroupLayoutDescriptor {
                    label: Some(Self::LABEL),
                    entries: &[
                        BuildinModelBind::ENTRY,
                        WaterMaterialPropertype::ENTRY,
                    ],
                }
            )
        )
    }

    pub fn bind_group(
        device: &RenderDevice,
        group: &mut RenderBindGroup,
        dynbuffer: &RenderDynUniformBuffer,
    ) {
        group.bind_group = Some(
            BindGroup::from(
                device.create_bind_group(
                    &wgpu::BindGroupDescriptor {
                        label: Some(IDWaterMaterialBindGroup::LABEL),
                        layout: &group.layout,
                        entries: &[
                            BuildinModelBind::dyn_entry(dynbuffer),
                            WaterMaterialPropertype::dyn_entry(dynbuffer),
                        ],
                    }
                )
            )
        ); 
    }
}


pub struct SysWaterMaterialBindGroupUpdate;
#[setup]
impl SysWaterMaterialBindGroupUpdate {
    #[system]
    pub fn tick(
        device: Res<RenderDevice>,
        dynbuffer: Res<RenderDynUniformBuffer>,
        dynbuffer_flag: Res<SingleDynUnifromBufferReBindFlag>,
        mut bindgroups: ResMut<RenderBindGroupPool>,
        id: ResMut<IDWaterMaterialBindGroup>,
    ) {
        println!("Sys WaterMaterial BindGroup Update");
        if dynbuffer_flag.0 {
            match bindgroups.get_mut(id.0) {
                Some(mut group) => {
                    IDWaterMaterialBindGroup::bind_group(&device, &mut group, &dynbuffer);
                },
                None => todo!(),
            }
        }
    }
}

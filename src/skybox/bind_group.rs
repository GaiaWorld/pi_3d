use pi_ecs::{prelude::{Res, Query, ResMut}, sys::system};
use pi_ecs_macros::setup;
use pi_render::rhi::{device::RenderDevice, bind_group_layout::BindGroupLayout, bind_group::BindGroup};

use crate::{object::{ObjectID, GameObject}, meshes::model::BuildinModelBind, materials::{bind_group::RenderBindGroup, uniform_buffer::SingleDynUnifromBufferReBindFlag}, resources::RenderDynUniformBuffer, shaders::FragmentUniformBind};

use super::material::SkyboxMaterialPropertype;

pub struct IDSkyboxMaterialBindGroup(pub ObjectID);
impl IDSkyboxMaterialBindGroup {
    const LABEL: &'static str = "SkyboxMaterialBindGroup";
    pub const SET: u32 = 1;

    pub fn layout(device: &RenderDevice) -> BindGroupLayout {
        BindGroupLayout::from(
            device.create_bind_group_layout(
                &wgpu::BindGroupLayoutDescriptor {
                    label: Some(Self::LABEL),
                    entries: &[
                        BuildinModelBind::ENTRY,
                        SkyboxMaterialPropertype::ENTRY,
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
                        label: Some(IDSkyboxMaterialBindGroup::LABEL),
                        layout: &group.layout,
                        entries: &[
                            BuildinModelBind::dyn_entry(dynbuffer),
                            SkyboxMaterialPropertype::dyn_entry(dynbuffer),
                        ],
                    }
                )
            )
        ); 
    }
}


pub struct SysSkyboxMaterialBindGroupUpdate;
#[setup]
impl SysSkyboxMaterialBindGroupUpdate {
    #[system]
    pub fn tick(
        device: Res<RenderDevice>,
        dynbuffer: Res<RenderDynUniformBuffer>,
        dynbuffer_flag: Res<SingleDynUnifromBufferReBindFlag>,
        mut bindgroups: Query<GameObject, &mut RenderBindGroup>,
        id: ResMut<IDSkyboxMaterialBindGroup>,
    ) {
        println!("Sys SkyboxMaterial BindGroup Update");
        if dynbuffer_flag.0 {
            match bindgroups.get_mut(id.0) {
                Some(mut group) => {
                    IDSkyboxMaterialBindGroup::bind_group(&device, &mut group, &dynbuffer);
                },
                None => todo!(),
            }
        }
    }
}
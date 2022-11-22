use pi_ecs::{prelude::{Res, Query, ResMut}, sys::system};
use pi_ecs_macros::setup;
use pi_render::rhi::{device::RenderDevice, bind_group_layout::BindGroupLayout, bind_group::BindGroup};

use pi_scene_context::{object::{ObjectID, GameObject}, meshes::model::BuildinModelBind, materials::{bind_group::RenderBindGroup, uniform_buffer::SingleDynUnifromBufferReBindFlag}, resources::RenderDynUniformBuffer, shaders::FragmentUniformBind};

use super::material::CloudMaterialPropertype;

pub struct IDCloudMaterialBindGroup(pub ObjectID);
impl IDCloudMaterialBindGroup {
    const LABEL: &'static str = "CloudMaterialBindGroup";
    pub const SET: u32 = 1;

    pub fn layout(device: &RenderDevice) -> BindGroupLayout {
        BindGroupLayout::from(
            device.create_bind_group_layout(
                &wgpu::BindGroupLayoutDescriptor {
                    label: Some(Self::LABEL),
                    entries: &[
                        BuildinModelBind::ENTRY,
                        CloudMaterialPropertype::ENTRY,
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
                        label: Some(IDCloudMaterialBindGroup::LABEL),
                        layout: &group.layout,
                        entries: &[
                            BuildinModelBind::dyn_entry(dynbuffer),
                            CloudMaterialPropertype::dyn_entry(dynbuffer),
                        ],
                    }
                )
            )
        ); 
    }
}


pub struct SysCloudMaterialBindGroupUpdate;
#[setup]
impl SysCloudMaterialBindGroupUpdate {
    #[system]
    pub fn tick(
        device: Res<RenderDevice>,
        dynbuffer: Res<RenderDynUniformBuffer>,
        dynbuffer_flag: Res<SingleDynUnifromBufferReBindFlag>,
        mut bindgroups: Query<GameObject, &mut RenderBindGroup>,
        id: ResMut<IDCloudMaterialBindGroup>,
    ) {
        println!("Sys CloudMaterial BindGroup Update");
        if dynbuffer_flag.0 {
            match bindgroups.get_mut(id.0) {
                Some(mut group) => {
                    IDCloudMaterialBindGroup::bind_group(&device, &mut group, &dynbuffer);
                },
                None => todo!(),
            }
        }
    }
}

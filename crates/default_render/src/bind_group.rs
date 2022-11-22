use pi_ecs::prelude::{Res, Query, ResMut};
use pi_ecs_macros::setup;
use pi_render::rhi::{bind_group_layout::BindGroupLayout, bind_group::BindGroup, device::RenderDevice};

use pi_scene_context::{materials::{bind_group::{RenderBindGroup, RenderBindGroupKey, RenderBindGroupPool}, uniform_buffer::SingleDynUnifromBufferReBindFlag}, object::{GameObject, ObjectID}, meshes::model::BuildinModelBind, shaders::FragmentUniformBind, resources::RenderDynUniformBuffer};

use super::default_material::DefaultMaterialPropertype;

pub struct IDDefaultMaterialBindGroup(pub RenderBindGroupKey);
impl IDDefaultMaterialBindGroup {
    const LABEL: &'static str = "DefaultMaterialBindGroup";
    pub const SET: u32 = 1;

    pub fn layout(device: &RenderDevice) -> BindGroupLayout {
        BindGroupLayout::from(
            device.create_bind_group_layout(
                &wgpu::BindGroupLayoutDescriptor {
                    label: Some(Self::LABEL),
                    entries: &[
                        BuildinModelBind::ENTRY,
                        DefaultMaterialPropertype::ENTRY,
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
                        label: Some(IDDefaultMaterialBindGroup::LABEL),
                        layout: &group.layout,
                        entries: &[
                            BuildinModelBind::dyn_entry(dynbuffer),
                            DefaultMaterialPropertype::dyn_entry(dynbuffer),
                        ],
                    }
                )
            )
        ); 
    }
}

pub struct SysDefaultMaterialBindGroupUpdate;
#[setup]
impl SysDefaultMaterialBindGroupUpdate {
    #[system]
    pub fn tick(
        device: Res<RenderDevice>,
        dynbuffer: Res<RenderDynUniformBuffer>,
        dynbuffer_flag: Res<SingleDynUnifromBufferReBindFlag>,
        mut bindgroups: ResMut<RenderBindGroupPool>,
        id: ResMut<IDDefaultMaterialBindGroup>,
    ) {
        // println!("Sys DefaultMaterial BindGroup Update");
        if dynbuffer_flag.0 {
            match bindgroups.get_mut(id.0) {
                Some(mut group) => {
                    // println!("IDDefaultMaterialBindGroup bind_group");
                    IDDefaultMaterialBindGroup::bind_group(&device, &mut group, &dynbuffer);
                },
                None => todo!(),
            }
        }
    }
}
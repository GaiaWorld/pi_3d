use pi_ecs::{prelude::ResMut, world::World};
use pi_render::{rhi::{bind_group_layout::BindGroupLayout, device::RenderDevice, bind_group::BindGroup, dyn_uniform_buffer::DynUniformBuffer}, graph::{NodeId, graph::RenderGraph}, components::view::target_alloc::ShareTargetView};
use wgpu::RenderPass;

use crate::{cameras::camera::CameraRenderData, scene::SceneTime, environment::{fog::SceneFog, ambient_light::AmbientLight}, shaders::FragmentUniformBind};

use super::RenderNode;


pub struct MainCameraBindGroup {
    pub bind_group: Option<BindGroup>,
    pub set: u32,
}
impl MainCameraBindGroup {
    pub fn new(
        device: &RenderDevice,
        dynbuffer: &mut DynUniformBuffer,
    ) -> Self {
        let camera_bind = dynbuffer.alloc_binding::<CameraRenderData>();
        let fog_bind = dynbuffer.alloc_binding::<SceneFog>();
        let time_bind = dynbuffer.alloc_binding::<SceneTime>();
        let ambient_light_bind = dynbuffer.alloc_binding::<AmbientLight>();

        let bind_group_0_layout = BindGroupLayout::from(
            device.create_bind_group_layout(
                &wgpu::BindGroupLayoutDescriptor {
                    label: Some("BuildinBindGroup"),
                    entries: &[
                        CameraRenderData::ENTRY,
                        SceneTime::ENTRY,
                        SceneFog::ENTRY,
                        AmbientLight::ENTRY,
                    ],
                }
            )
        );
        
        let bind_group_0 = BindGroup::from(
            device.create_bind_group(
                &wgpu::BindGroupDescriptor {
                    label: Some("BuildinBindGroup"),
                    layout: &bind_group_0_layout,
                    entries: &[
                        CameraRenderData::entry(&camera_bind, dynbuffer),
                        SceneTime::entry(&time_bind, dynbuffer),
                        SceneFog::entry(&fog_bind, dynbuffer),
                        AmbientLight::entry(&ambient_light_bind, dynbuffer),
                    ],
                }
            )
        );

        Self {
            bind_group: Some(bind_group_0),
            set: 0,
        }
    }
}

pub struct MainCameraOpaqueRenderer {
    pub graphic: Option<NodeId>,
    // pub render_calls: Vec<dyn Fn(wgpu::RenderPass)>,
}
impl MainCameraOpaqueRenderer {
    pub fn new(
        mut rg: ResMut<RenderGraph>,
        world: &World,
    ) -> Self {
        
        let node = rg.add_node("MainCameraOpaque", RenderNode::new(world));
        match node {
            Ok(node) => {
                Self {
                    graphic: Some(node)
                }
            },
            Err(_) => {
                Self {
                    graphic: None
                }
            },
        }
    }
}

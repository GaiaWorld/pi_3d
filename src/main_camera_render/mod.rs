use futures::FutureExt;
use pi_ecs::{query::QueryState, prelude::Setup};
use pi_futures::BoxFuture;
use pi_render::{graph::{NodeId, graph::RenderGraph, node::Node, RenderContext}, rhi::{texture::ScreenTexture, device::RenderDevice}};
use render_data_container::GeometryBufferPool;

use crate::{cameras::camera::CameraViewport, renderers::{render_object::{RenderObjectOpaqueList, RenderObjectTransparentList, RenderObjectID}}, object::{ObjectID, GameObject}, resources::{SingleRenderObjectPipelinePool, SingleGeometryBufferPool}, plugin::Plugin, materials::command::{SingleRenderBindGroupCommandList, RenderBindGroupCommand}};

use self::{command::{SingleMainCameraRenderCommandList, SysMainCameraRenderCommand, MainCameraRenderCommand}, graph::SingleMainCameraOpaqueRenderNode, draw_sort_sys::DrawSortTick, bind_group::{SysMainCameraRenderBindGroupUpdate, SysMainCameraRenderUniformUpdate, IDMainCameraRenderBindGroup}};

pub mod command;
pub mod bind_group;
pub mod graph;
pub mod draw_sort_sys;
pub mod interface;

pub struct MainCameraRenderer {
    pub viewport: CameraViewport,
    pub opaque_graphic: Option<NodeId>,
    pub skybox_graphic: Option<NodeId>,
    pub transparent_graphic: Option<NodeId>,
    pub opaque_draws: RenderObjectOpaqueList,
    pub skybox_draws: RenderObjectOpaqueList,
    pub transparent_draws: RenderObjectTransparentList,
}
impl MainCameraRenderer {
    pub fn new(
        object_id: ObjectID,
        rg: &mut RenderGraph,
    ) -> Self {
        let opaque_graphic = rg.add_node("MainCameraOpaque", SingleMainCameraOpaqueRenderNode::new(object_id)).unwrap();
        Self {
            viewport: CameraViewport::default(),
            opaque_graphic: Some(opaque_graphic),
            skybox_graphic: None,
            transparent_graphic: None,
            opaque_draws: RenderObjectOpaqueList::default(),
            skybox_draws: RenderObjectOpaqueList::default(),
            transparent_draws: RenderObjectTransparentList::default(),
        }
    }
}

pub struct PluginMainCameraRender;
impl Plugin for PluginMainCameraRender {
    fn init(
        engine: &mut crate::engine::Engine,
        stages: &mut crate::run_stage::RunStage,
    ) -> Result<(), crate::plugin::ErrorPlugin> {
        let mut world = engine.world_mut().clone();
        let device = world.get_resource::<RenderDevice>().unwrap().clone();

        SysMainCameraRenderCommand::setup(&mut world, stages.command_stage());
        DrawSortTick::setup(&mut world, stages.render_sort());
        SysMainCameraRenderUniformUpdate::setup(&mut world, stages.uniform_update());
        SysMainCameraRenderBindGroupUpdate::setup(&mut world, stages.between_uniform_update_and_filter_culling());
        
        world.insert_resource(SingleMainCameraRenderCommandList::default());

        let main_camera_bind_group_id = engine.new_object();
        let layout = IDMainCameraRenderBindGroup::layout(&device);
        let commands = world.get_resource_mut::<SingleRenderBindGroupCommandList>().unwrap();
        commands.list.push(RenderBindGroupCommand::Create(main_camera_bind_group_id, layout, IDMainCameraRenderBindGroup::SET));
        world.insert_resource(IDMainCameraRenderBindGroup(main_camera_bind_group_id));

        Ok(())
    }
}

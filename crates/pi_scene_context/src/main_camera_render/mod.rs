

use pi_ecs::{prelude::{Setup}};
use pi_engine_shell::{assets::sync_load::{PluginAssetSyncNotNeedLoad}, run_stage::{ERunStageChap}};
use pi_render::{graph::{NodeId, graph::RenderGraph}, rhi::{device::RenderDevice}};

use crate::{
    cameras::{camera::{CameraViewport}},
    renderers::{render_object::{RenderObjectOpaqueList, RenderObjectTransparentList, RenderObjectBindGroup}, pipeline::{ResRenderPipeline, KeyRenderPipeline}},
    object::{ObjectID},
    plugin::Plugin,
};

use self::{
    command::{SingleMainCameraRenderCommandList, SysMainCameraRenderCommand},
    graph::SingleMainCameraOpaqueRenderNode, draw_sort_sys::{DrawSortTick, SysModelListUpdateByCamera, SysModelListAfterCullinUpdateByCamera, SysModelListUpdateByGeometry, SysModelListAfferCullingUpdateByModelWorldMatrix, SysModelListAfterCullinUpdateByGeometry}, renderer_binds_sys::{SysRendererInitForCamera, SysRendererInitBindForCamera, SysCameraBindUpdate}, bind_group::SysMainCameraRendererBindGroup,
};

pub mod command;
pub mod bind_group;
pub mod graph;
pub mod draw_sort_sys;
pub mod interface;
pub mod pipeline;
pub mod renderer_binds_sys;

pub struct MainCameraRenderer {
    pub ready: bool,
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
            ready: false,
        }
    }
    pub fn clear(&mut self) {
        self.opaque_draws.bind_groups.clear();
        self.opaque_draws.draws.clear();;

        self.skybox_draws.bind_groups.clear();
        self.skybox_draws.draws.clear();

        self.transparent_draws.bind_groups.clear();
        self.transparent_draws.draws.clear();
        self.ready = false;
    }
    pub fn reset(&mut self, camera_bind_group: RenderObjectBindGroup) {
        self.opaque_draws.bind_groups.push(camera_bind_group.clone());

        self.skybox_draws.bind_groups.push(camera_bind_group.clone());

        self.transparent_draws.bind_groups.push(camera_bind_group);
        self.ready = true;
    }
}


pub struct PluginMainCameraRender;
impl Plugin for PluginMainCameraRender {
    fn init(
        &mut self,
        engine: &mut crate::engine::Engine,
        stages: &mut crate::run_stage::RunStage,
    ) -> Result<(), crate::plugin::ErrorPlugin> {
        PluginAssetSyncNotNeedLoad::<KeyRenderPipeline, ResRenderPipeline>::new(false, 10 * 1024 * 1024, 60 * 1000).init(engine, stages);
    
        let world = engine.world_mut();
        let device = world.get_resource::<RenderDevice>().unwrap().clone();

        // let main_camera_bind_group_id = Atom::from(IDMainCameraRenderBindGroup::LABEL);
        // world.get_resource_mut::<RenderBindGroupPool>().unwrap().creat(&device, main_camera_bind_group_id.clone(), IDMainCameraRenderBindGroup::layout_entries().as_slice(), IDMainCameraRenderBindGroup::SET);

        let world = engine.world_mut();

        SysMainCameraRenderCommand::setup(world, stages.query_stage::<SysMainCameraRenderCommand>(ERunStageChap::Command));
        SysMainCameraRendererBindGroup::setup(world, stages.query_stage::<SysMainCameraRendererBindGroup>(ERunStageChap::Uniform));
        DrawSortTick::setup(world, stages.query_stage::<DrawSortTick>(ERunStageChap::Uniform));

        SysModelListUpdateByCamera::setup(world, stages.query_stage::<SysModelListUpdateByCamera>(ERunStageChap::Command));
        SysModelListUpdateByGeometry::setup(world, stages.query_stage::<SysModelListUpdateByGeometry>(ERunStageChap::Command));

        SysRendererInitForCamera::setup(world, stages.query_stage::<SysRendererInitForCamera>(ERunStageChap::Command));

        SysRendererInitBindForCamera::setup(world, stages.query_stage::<SysRendererInitBindForCamera>(ERunStageChap::Command));
        SysCameraBindUpdate::setup(world, stages.query_stage::<SysCameraBindUpdate>(ERunStageChap::Command));
        
        SysModelListAfterCullinUpdateByCamera::setup(world, stages.query_stage::<SysModelListAfterCullinUpdateByCamera>(ERunStageChap::Command));
        SysModelListAfterCullinUpdateByGeometry::setup(world, stages.query_stage::<SysModelListAfterCullinUpdateByGeometry>(ERunStageChap::Command));
        SysModelListAfferCullingUpdateByModelWorldMatrix::setup(world, stages.query_stage::<SysModelListAfferCullingUpdateByModelWorldMatrix>(ERunStageChap::Command));

        world.insert_resource(SingleMainCameraRenderCommandList::default());

        // world.insert_resource(IDMainCameraRenderBindGroup(main_camera_bind_group_id));

        Ok(())
    }
}

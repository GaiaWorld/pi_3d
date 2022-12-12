
use pi_atom::Atom;
use pi_ecs::{prelude::{Setup, Query, Res}, query::With};
use pi_ecs_macros::setup;
use pi_engine_shell::{object::{InterfaceObject, GameObject}, assets::sync_load::{PluginAssetSyncLoad, PluginAssetSyncNotNeedLoad}};
use pi_render::{graph::{NodeId, graph::RenderGraph}, rhi::{device::RenderDevice}};

use crate::{
    cameras::camera::{CameraViewport, CameraRenderData},
    renderers::{render_object::{RenderObjectOpaqueList, RenderObjectTransparentList, RenderObjectBindGroup, RenderObjectID}, pipeline::{ResRenderPipeline, KeyRenderPipeline}},
    object::{ObjectID},
    plugin::Plugin,
    materials::{bind_group::{RenderBindGroup, RenderBindGroupPool}}
};

use self::{
    command::{SingleMainCameraRenderCommandList, SysMainCameraRenderCommand},
    graph::SingleMainCameraOpaqueRenderNode, draw_sort_sys::{DrawSortTick, SysMainCameraFilter},
    bind_group::{SysMainCameraRenderBindGroupUpdate, SysMainCameraRenderUniformUpdate, IDMainCameraRenderBindGroup}, 
    pipeline::{SysMaterialMainCameraChangeByMesh, SysMaterialMainCameraChangeByMat, SysMainCameraPipeline}
};

pub mod command;
pub mod bind_group;
pub mod graph;
pub mod draw_sort_sys;
pub mod interface;
pub mod pipeline;

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


struct SysMainCameraRenderReset;
#[setup]
impl SysMainCameraRenderReset {
    #[system]
    pub fn reset(
        maincameras: Query<GameObject, (&CameraRenderData, &RenderObjectID)>,
        mut renders: Query<GameObject, &mut MainCameraRenderer>,
        id_bind_group_main_camera: Res<IDMainCameraRenderBindGroup>,
        bind_groups: Res<RenderBindGroupPool>,
    ) {
        let id_bind_group_main_camera = id_bind_group_main_camera.0.clone();
        maincameras.iter().for_each(|(cameradata, renderid)| {
            if bind_groups.get(&id_bind_group_main_camera).unwrap().bind_group.is_some() {
                if let Some(mut render) = renders.get_mut(renderid.0) {
                    let camera_bind_group = RenderObjectBindGroup {
                        bind_group: id_bind_group_main_camera.clone(),
                        offsets: vec![
                            *cameradata.bind_offset,
                            0, 0, 0,
                        ],
                    };
                    render.reset(camera_bind_group);
                }
            }
        });
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

        let main_camera_bind_group_id = Atom::from(IDMainCameraRenderBindGroup::LABEL);
        world.get_resource_mut::<RenderBindGroupPool>().unwrap().creat(&device, main_camera_bind_group_id.clone(), IDMainCameraRenderBindGroup::layout_entries().as_slice(), IDMainCameraRenderBindGroup::SET);

        let world = engine.world_mut();

        SysMaterialMainCameraChangeByMesh::setup(world, stages.command_stage());
        SysMaterialMainCameraChangeByMat::setup(world, stages.command_stage());
        SysMainCameraPipeline::setup(world, stages.command_stage());

        SysMainCameraRenderCommand::setup(world, stages.command_stage());
        DrawSortTick::setup(world, stages.render_sort());
        SysMainCameraRenderUniformUpdate::setup(world, stages.uniform_update());
        SysMainCameraRenderBindGroupUpdate::setup(world, stages.between_uniform_update_and_filter_culling());
        SysMainCameraRenderReset::setup(world, stages.between_uniform_update_and_filter_culling());
        SysMainCameraFilter::setup(world, stages.filter_culling());
        
        world.insert_resource(SingleMainCameraRenderCommandList::default());

        world.insert_resource(IDMainCameraRenderBindGroup(main_camera_bind_group_id));

        Ok(())
    }
}



use instance::instanced_buffer::*;
use pi_atom::Atom;

use pi_scene_shell::{prelude::*, run_stage::EngineCustomPlugins};
///
/// 网格信息单独与 GameObject 绑定

use crate::{object::sys_dispose_ready, prelude::*};

use self::{
    sys_vertex_buffer_use::*,
    load::*,
    command::*,
    command_sys::*,
    base::*, instance::instanced_buffer::InstanceBufferAllocator,
};

pub mod base;
pub mod command;
pub mod command_sys;
pub mod vertex_buffer_useinfo;
pub mod sys_vertex_buffer_use;
pub mod geometry;
pub mod indices;
pub mod instance;
pub mod load;
pub mod prelude;

pub type VDK = usize;
pub type GBID = Atom;

pub struct PluginGeometry;
impl Plugin for PluginGeometry {

    fn build(&self, app: &mut App) {
        let option = app.world.get_resource::<EngineCustomPlugins>().unwrap().clone();
        app.insert_resource(ActionListGeometryCreate::default());
        app.insert_resource(VertexBufferDataMap3D(SingleVertexBufferDataMap::default()));
        
        let cfg = if let Some(cfg) = app.world.get_resource::<AssetCfgVertexBuffer3D>() {
            cfg
        } else {
            app.insert_resource(AssetCfgVertexBuffer3D::default());
            app.world.get_resource::<AssetCfgVertexBuffer3D>().unwrap()
        };
        let vbsize = if let Some(arg) = app.world.get_resource::<ArgVertexBufferAllocator3DSize>() {
            arg.0
        } else { 64 * 1024 };
        let insbsize = if let Some(arg) = app.world.get_resource::<ArgInstanceBufferAllocatorSize>() {
            arg.0
        } else { 64 * 1024 };

        let device = app.world.get_resource::<PiRenderDevice>().unwrap();
        let queue = app.world.get_resource::<PiRenderQueue>().unwrap();
        let mut allocator = VertexBufferAllocator3D(VertexBufferAllocator::create(vbsize as usize, cfg.0.timeout, true));
        let instanceallocator = InstanceBufferAllocator::new(insbsize, &mut allocator, device, queue);
        
        app.insert_resource(CombineBuffer::new(option.combinebuffersize, &mut allocator, device, queue));
        app.insert_resource(CombineDataCommon::new(option.combinebuffersize));

        app.insert_resource(allocator);
        app.insert_resource(instanceallocator);
        let cfg = app.world.get_resource_mut::<AssetMgrConfigs>().unwrap().query::<EVertexBufferRange>();
        app.insert_resource(ShareAssetMgr::<EVertexBufferRange>::create(GarbageEmpty(), cfg.flag, &cfg));
        app.insert_resource(GeometryVBLoader::default());

#[cfg(feature = "use_bevy")]
        app.configure_sets(
            Update, 
            (
                StageGeometry::GeoCreate.after(StageModel::_InitMesh),
                StageGeometry::_GeoCreate.after(StageGeometry::GeoCreate),
                StageGeometry::VertexBufferLoaded.in_set(FrameDataPrepare).after(StageGeometry::_GeoCreate),
                StageGeometry::_VertexBufferLoadedApply.in_set(FrameDataPrepare).after(StageGeometry::VertexBufferLoaded),
                StageGeometry::GeometryLoaded.in_set(FrameDataPrepare).after(StageGeometry::_VertexBufferLoadedApply).before(ERunStageChap::Collect),
                StageGeometry::GeoUpload.in_set(FrameDataPrepare).after(StageGeometry::GeometryLoaded).after(StageRenderer::DrawList),
            )
        );

#[cfg(feature = "use_bevy")]
        app.add_systems(
            Update, 
            (
                apply_deferred.in_set(StageGeometry::_GeoCreate),
                apply_deferred.in_set(StageGeometry::_VertexBufferLoadedApply),
                sys_create_geometry.in_set(StageGeometry::GeoCreate),
                sys_vertex_buffer_loaded.in_set(StageGeometry::VertexBufferLoaded),
                (
                    sys_vertex_buffer_slots_loaded,
                    sys_geometry_enable
                ).chain().in_set(StageGeometry::GeometryLoaded),
                sys_instanced_buffer_upload.in_set(StageGeometry::GeoUpload),
                (
                    sys_dispose_about_geometry  // .run_if(should_run)
                    .after(sys_dispose_ready)
                ).in_set(ERunStageChap::Dispose)
            )
        );

#[cfg(not(feature = "use_bevy"))]
        app
        .configure_set(Update, StageGeometry::GeoCreate                 .in_set(ERunStageChap::Create).after(StageModel::_InitMesh))
        .configure_set(Update, StageGeometry::_GeoCreate                .in_set(ERunStageChap::Create).after(StageGeometry::GeoCreate).before(ERunStageChap::Dispose))
        .configure_set(Update, StageGeometry::VertexBufferLoaded        .in_set(ERunStageChap::Modify).in_set(FrameDataPrepare).after(StageGeometry::_GeoCreate))
        .configure_set(Update, StageGeometry::_VertexBufferLoadedApply  .in_set(ERunStageChap::Modify).in_set(FrameDataPrepare).after(StageGeometry::VertexBufferLoaded))
        .configure_set(Update, StageGeometry::GeometryLoaded            .in_set(ERunStageChap::Culled).in_set(FrameDataPrepare).after(StageGeometry::_VertexBufferLoadedApply))
        .configure_set(Update, StageGeometry::GeoUpload                 .in_set(ERunStageChap::Culled).in_set(FrameDataPrepare).after(StageGeometry::GeometryLoaded))
        .configure_set(Update, StageGeometry::GeoDispose                .in_set(ERunStageChap::Dispose).before(StageModel::MeshDispose))
        ;

#[cfg(not(feature = "use_bevy"))]
        app
        .add_systems(Update, sys_create_geometry
            // .run_if(runif_acts::<OpsGeomeryCreate>)     
            .in_set(StageGeometry::GeoCreate))
        .add_systems(Update, sys_vertex_buffer_loaded    .in_set(StageGeometry::VertexBufferLoaded))
        .add_systems(Update, sys_vertex_buffer_slots_loaded
            // .run_if(runif_comp::<FlagGeometryDirty>)      
            .in_set(StageGeometry::GeometryLoaded))
        .add_systems(Update, sys_geometry_enable
            // .run_if(runif_comp::<RenderGeometryComp>)
            .after(sys_vertex_buffer_slots_loaded).in_set(StageGeometry::GeometryLoaded))
        .add_systems(Update, sys_instanced_buffer_upload     .in_set(StageGeometry::GeoUpload))
        .add_systems(Update, sys_dispose_about_geometry      .after(sys_dispose_ready).in_set(StageGeometry::GeoDispose))
        ;
    }
}

pub fn sys_dispose_about_geometry(
) {
}
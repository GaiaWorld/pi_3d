

use pi_atom::Atom;

use pi_scene_shell::prelude::*;
///
/// 网格信息单独与 GameObject 绑定

use crate::{object::sys_dispose_ready, prelude::*};

use self::{
    sys_vertex_buffer_use::*,
    load::sys_vertex_buffer_loaded,
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
        app.insert_resource(ActionListGeometryCreate::default());
        app.insert_resource(VertexBufferDataMap3D(SingleVertexBufferDataMap::default()));
        
        let cfg = if let Some(cfg) = app.world.get_resource::<AssetCfgVertexBuffer3D>() {
            cfg
        } else {
            app.insert_resource(AssetCfgVertexBuffer3D::default());
            app.world.get_resource::<AssetCfgVertexBuffer3D>().unwrap()
        };
        let device = app.world.get_resource::<PiRenderDevice>().unwrap();
        let queue = app.world.get_resource::<PiRenderQueue>().unwrap();
        let mut allocator = VertexBufferAllocator3D(VertexBufferAllocator::new(cfg.0.min, cfg.0.timeout));
        let instanceallocator = InstanceBufferAllocator::new(4 * 1024 * 1024, &mut allocator, device, queue);
        app.insert_resource(allocator);
        app.insert_resource(instanceallocator);
        let cfg = app.world.get_resource_mut::<AssetMgrConfigs>().unwrap().query::<EVertexBufferRange>();
        app.insert_resource(ShareAssetMgr::<EVertexBufferRange>::create(GarbageEmpty(), cfg.flag, &cfg));
        app.insert_resource(GeometryVBLoader::default());

#[cfg(feature = "use_bevy")]
        app.configure_sets(
            Update, 
            (
                StageGeometry::Create.after(StageModel::_InitMesh),
                StageGeometry::_GeoCreate.after(StageGeometry::Create),
                StageGeometry::VertexBufferLoaded.in_set(FrameDataPrepare).after(StageGeometry::_GeoCreate),
                StageGeometry::_VertexBufferLoadedApply.in_set(FrameDataPrepare).after(StageGeometry::VertexBufferLoaded),
                StageGeometry::GeometryLoaded.in_set(FrameDataPrepare).after(StageGeometry::_VertexBufferLoadedApply).before(ERunStageChap::Uniform),
                StageGeometry::Upload.in_set(FrameDataPrepare).after(StageGeometry::GeometryLoaded).after(StageRenderer::DrawList),
            )
        );

#[cfg(feature = "use_bevy")]
        app.add_systems(
            Update, 
            (
                apply_deferred.in_set(StageGeometry::_GeoCreate),
                apply_deferred.in_set(StageGeometry::_VertexBufferLoadedApply),
                sys_create_geometry.in_set(StageGeometry::Create),
                sys_vertex_buffer_loaded.in_set(StageGeometry::VertexBufferLoaded),
                (
                    sys_vertex_buffer_slots_loaded,
                    sys_geometry_enable
                ).chain().in_set(StageGeometry::GeometryLoaded),
                sys_instanced_buffer_upload.in_set(StageGeometry::Upload),
                (
                    sys_dispose_about_geometry  // .run_if(should_run)
                    .after(sys_dispose_ready)
                ).in_set(ERunStageChap::Dispose)
            )
        );

#[cfg(not(feature = "use_bevy"))]
        app
        .configure_set(Update, StageGeometry::Create.after(StageModel::_InitMesh))
        .configure_set(Update, StageGeometry::_GeoCreate.after(StageGeometry::Create))
        .configure_set(Update, StageGeometry::VertexBufferLoaded.in_set(FrameDataPrepare).after(StageGeometry::_GeoCreate))
        .configure_set(Update, StageGeometry::_VertexBufferLoadedApply.in_set(FrameDataPrepare).after(StageGeometry::VertexBufferLoaded))
        .configure_set(Update, StageGeometry::GeometryLoaded.in_set(FrameDataPrepare).after(StageGeometry::_VertexBufferLoadedApply).before(ERunStageChap::Uniform))
        .configure_set(Update, StageGeometry::Upload.in_set(FrameDataPrepare).after(StageGeometry::GeometryLoaded).after(StageRenderer::DrawList))
        ;

#[cfg(not(feature = "use_bevy"))]
        app
        .add_systems(Update, sys_create_geometry         .in_set(StageGeometry::Create))
        .add_systems(Update, sys_vertex_buffer_loaded    .in_set(StageGeometry::VertexBufferLoaded))
        .add_systems(Update, sys_vertex_buffer_slots_loaded      .in_set(StageGeometry::GeometryLoaded))
        .add_systems(Update, sys_geometry_enable                 .after(sys_vertex_buffer_slots_loaded).in_set(StageGeometry::GeometryLoaded))
        .add_systems(Update, sys_instanced_buffer_upload     .in_set(StageGeometry::Upload))
        .add_systems(Update, sys_dispose_about_geometry      .after(sys_dispose_ready).in_set(ERunStageChap::Dispose))
        ;
    }
}

pub fn sys_dispose_about_geometry(
    items: Query<(Entity, &DisposeReady, &GeometryDesc), Changed<DisposeReady>>,
    mut disposecanlist: ResMut<ActionListDisposeCan>,
) {
    items.iter().for_each(|(entity, state, _)| {
        if state.0 == false { return; }

        disposecanlist.push(OpsDisposeCan::ops(entity));
    });
}
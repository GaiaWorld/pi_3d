

use pi_atom::Atom;

use pi_engine_shell::prelude::*;
///
/// 网格信息单独与 GameObject 绑定

use crate::{object::sys_dispose_ready, plugin::Plugin, prelude::*};

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

    fn build(&self, app: &mut bevy::prelude::App) {
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
        let instanceallocator = InstanceBufferAllocator::new(1024 * 1024, &mut allocator, device, queue);
        app.insert_resource(allocator);
        app.insert_resource(instanceallocator);
        app.insert_resource(ShareAssetMgr::<EVertexBufferRange>::new(GarbageEmpty(), false, 10 * 1024, 10 * 1000));
        app.insert_resource(GeometryVBLoader::default());

        app.configure_set(Update, StageGeometry::VertexBufferLoaded.after(ERunStageChap::_InitialApply));
        app.configure_set(Update, StageGeometry::VertexBufferLoadedApply.after(StageGeometry::VertexBufferLoaded));
        app.configure_set(Update, StageGeometry::GeometryLoaded.after(StageGeometry::VertexBufferLoadedApply).before(ERunStageChap::Uniform));
        app.add_systems(Update, apply_deferred.in_set(StageGeometry::VertexBufferLoadedApply) );

        app.add_systems(
			Update,
            (
                sys_create_geometry.in_set(ERunStageChap::Initial),
                sys_vertex_buffer_loaded.in_set(StageGeometry::VertexBufferLoaded),
            )
        );
        app.add_systems(
			Update,
            (
                sys_vertex_buffer_loaded_01,
                sys_vertex_buffer_loaded_02,
                sys_vertex_buffer_loaded_03,
                sys_vertex_buffer_loaded_04,
                sys_vertex_buffer_loaded_05,
                sys_vertex_buffer_loaded_06,
            ).chain().in_set(StageGeometry::GeometryLoaded)
        );
        app.add_systems(Update, 
            sys_geometry_enable.after(sys_vertex_buffer_loaded_06).in_set(StageGeometry::GeometryLoaded)
        );

        app.add_systems(Update, 
            sys_instanced_buffer_upload.in_set(ERunStageChap::Uniform)
        );

        app.add_systems(Update, 
            sys_dispose_about_geometry.run_if(should_run).after(sys_dispose_ready).in_set(ERunStageChap::Dispose)
        );
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
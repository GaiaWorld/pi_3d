use std::mem::replace;

use pi_assets::{mgr::AssetMgr, asset::GarbageEmpty};
use pi_atom::Atom;
use pi_ecs::{prelude::{ResMut, Setup, Commands, Query, EntityDelete, EntityCommands, Res, Component}};
use pi_ecs_macros::setup;
use pi_engine_shell::{engine_shell::EnginShell, object::GameObject, assets::sync_load::{AssetSyncWait, PluginAssetSyncLoad, InterfaceAssetSyncCreate}, run_stage::{TSystemStageInfo, ERunStageChap}};
use pi_render::{
    renderer::{
        vertex_buffer_loader::{VertexBufferLoader, SingleVertexBufferDataMap},
        vertex_buffer_desc::VertexBufferDesc,
        vertex_buffer::{VertexBufferLayouts, KeyVertexBuffer, VertexBufferAllocator, EVertexBufferRange},
        indices::{AssetKeyBufferIndices, AssetResBufferIndices, IndicesBufferDesc}, vertices::EVerticesBufferUsage, instance::EInstanceKind
    },
    render_3d::shader::instance_code::EInstanceCode,
    rhi::{RenderQueue, device::RenderDevice}
};
use pi_share::Share;
///
/// 网格信息单独与 GameObject 绑定

use crate::{object::ObjectID, plugin::Plugin, meshes::command::SysMeshCreateCommand};

use self::{
    vertex_buffer_useinfo::*,
    sys_vertex_buffer_use::{PluginVertexBuffers},
    geometry::RenderGeometryEable, load::sys_vertex_buffer_loaded, instance::{InstanceSourceRecord, instance_world_matrix::InstancedBufferWorldMatrix, instance_color::InstancedBufferColor, instance_tilloff::InstancedBufferTillOff}, base::{VBLoaderSlot, VBAllocator}
};

pub mod base;
pub mod command;
pub mod vertex_buffer_useinfo;
pub mod sys_vertex_buffer_use;
pub mod geometry;
pub mod indices;
pub mod instance;
pub mod load;

pub type VDK = usize;
pub type GBID = Atom;

pub struct GeometryDesc {
    pub list: Vec<VertexBufferDesc>,
}
impl GeometryDesc {
    pub fn slot_count(&self) -> usize {
        self.list.len()
    }
    pub fn get_desc(&self, slot: usize) -> VertexBufferDesc {
        self.list.get(slot).unwrap().clone()
    }
}

pub trait TInterfaceGeomtery {
    fn create_vertex_buffer(
        &self,
        key: KeyVertexBuffer,
        buffer: Vec<u8>,
    ) -> &Self;
    fn use_geometry(
        &self,
        entity: ObjectID,
        vertices: Vec<VertexBufferDesc>,
        indices: Option<IndicesBufferDesc>,
    ) -> &Self;
}

// impl TInterfaceGeomtery for EnginShell {
//     fn create_vertex_buffer(
//         &self,
//         key: KeyVertexBuffer,
//         buffer: Vec<u8>,
//     ) -> &Self {
//         let world = self.world();
//         let assert_mgr = world.get_resource::<Share<AssetMgr<EVertexBufferRange>>>().unwrap();
//         if !assert_mgr.check_asset(&key) {
//             let data_map = world.get_resource_mut::<SingleVertexBufferDataMap>().unwrap();
//             data_map.add(&key, buffer);
//         }

//         self
//     }
//     fn use_geometry(
//         &self,
//         entity: ObjectID,
//         descs: Vec<VertexBufferDesc>,
//         indices: Option<IndicesBufferDesc>,
//     ) -> &Self {
//         let commands = self.world().get_resource_mut::<SingleGeometryVBCommands>().unwrap();
//         commands.0.push(ECommand::Desc(entity, descs, indices));

//         self
//     }
// }

pub struct PluginGeometry;
impl Plugin for PluginGeometry {
    // fn init(
    //     &mut self,
    //     engine: &mut crate::engine::Engine,
    //     stages: &mut crate::run_stage::RunStage,
    // ) -> Result<(), crate::plugin::ErrorPlugin> {
    //     let world = engine.world_mut();
    //     world.insert_resource(SingleGeometryVBCommands::default());
    //     world.insert_resource(VBAllocator::new());
    //     world.insert_resource(AssetMgr::<EVertexBufferRange>::new(GarbageEmpty(), false, 1 * 1024 * 1024, 10 * 1000));

    //     world.insert_resource(VBLoaderSlot::<ObjectID, AssetResVBSlot01>::default());
    //     world.insert_resource(VBLoaderSlot::<ObjectID, AssetResVBSlot02>::default());
    //     world.insert_resource(VBLoaderSlot::<ObjectID, AssetResVBSlot03>::default());
    //     world.insert_resource(VBLoaderSlot::<ObjectID, AssetResVBSlot04>::default());
    //     world.insert_resource(VBLoaderSlot::<ObjectID, AssetResVBSlot05>::default());
    //     world.insert_resource(VBLoaderSlot::<ObjectID, AssetResVBSlot06>::default());
    //     world.insert_resource(VBLoaderSlot::<ObjectID, AssetResVBSlot07>::default());
    //     world.insert_resource(VBLoaderSlot::<ObjectID, AssetResVBSlot08>::default());
    //     world.insert_resource(VBLoaderSlot::<ObjectID, AssetResVBSlot09>::default());
    //     world.insert_resource(VBLoaderSlot::<ObjectID, AssetResVBSlot10>::default());
    //     world.insert_resource(VBLoaderSlot::<ObjectID, AssetResVBSlot11>::default());
    //     world.insert_resource(VBLoaderSlot::<ObjectID, AssetResVBSlot12>::default());
    //     world.insert_resource(VBLoaderSlot::<ObjectID, AssetResVBSlot13>::default());
    //     world.insert_resource(VBLoaderSlot::<ObjectID, AssetResVBSlot14>::default());
    //     world.insert_resource(VBLoaderSlot::<ObjectID, AssetResVBSlot15>::default());
    //     world.insert_resource(VBLoaderSlot::<ObjectID, AssetResVBSlot16>::default());
    //     world.insert_resource(VBLoaderSlot::<ObjectID, AssetResBufferIndices>::default());

    //     SysGeometryVBCommand::setup(world, stages.query_stage::<SysGeometryVBCommand>(ERunStageChap::Initial));
    //     SysVertexBufferLoad::setup(world, stages.query_stage::<SysVertexBufferLoad>(ERunStageChap::Draw));
    //     PluginVertexBuffers.init(engine, stages);

    //     Ok(())
    // }

    fn build(&self, app: &mut bevy::prelude::App) {
        // app.world.insert_resource(SingleGeometryVBCommands::default());
        app.world.insert_resource(VBAllocator::new());
        app.world.insert_resource(AssetMgr::<EVertexBufferRange>::new(GarbageEmpty(), false, 1 * 1024 * 1024, 10 * 1000));
        
        app.world.insert_resource(VBLoaderSlot::<ObjectID, AssetResVBSlot01>::default());
        app.world.insert_resource(VBLoaderSlot::<ObjectID, AssetResVBSlot02>::default());
        app.world.insert_resource(VBLoaderSlot::<ObjectID, AssetResVBSlot03>::default());
        app.world.insert_resource(VBLoaderSlot::<ObjectID, AssetResVBSlot04>::default());
        app.world.insert_resource(VBLoaderSlot::<ObjectID, AssetResVBSlot05>::default());
        app.world.insert_resource(VBLoaderSlot::<ObjectID, AssetResVBSlot06>::default());
        app.world.insert_resource(VBLoaderSlot::<ObjectID, AssetResVBSlot07>::default());
        app.world.insert_resource(VBLoaderSlot::<ObjectID, AssetResVBSlot08>::default());
        app.world.insert_resource(VBLoaderSlot::<ObjectID, AssetResVBSlot09>::default());
        app.world.insert_resource(VBLoaderSlot::<ObjectID, AssetResVBSlot10>::default());
        app.world.insert_resource(VBLoaderSlot::<ObjectID, AssetResVBSlot11>::default());
        app.world.insert_resource(VBLoaderSlot::<ObjectID, AssetResVBSlot12>::default());
        app.world.insert_resource(VBLoaderSlot::<ObjectID, AssetResVBSlot13>::default());
        app.world.insert_resource(VBLoaderSlot::<ObjectID, AssetResVBSlot14>::default());
        app.world.insert_resource(VBLoaderSlot::<ObjectID, AssetResVBSlot15>::default());
        app.world.insert_resource(VBLoaderSlot::<ObjectID, AssetResVBSlot16>::default());
        app.world.insert_resource(VBLoaderSlot::<ObjectID, AssetResBufferIndices>::default());

        app.add_system(
            sys_vertex_buffer_loaded
        );

        app.add_plugin(PluginVertexBuffers);
    }
}

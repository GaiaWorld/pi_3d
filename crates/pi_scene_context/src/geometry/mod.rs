use std::mem::replace;

use pi_assets::{mgr::AssetMgr, asset::GarbageEmpty};
use pi_atom::Atom;
use pi_ecs::{prelude::{ResMut, Setup, Commands}};
use pi_ecs_macros::setup;
use pi_engine_shell::{engine_shell::EnginShell, object::GameObject, assets::sync_load::{AssetSyncWait, PluginAssetSyncLoad, InterfaceAssetSyncCreate}, run_stage::{TSystemStageInfo, ERunStageChap}};
use pi_render::{
    renderer::{
        vertex_buffer_loader::{VertexBufferLoader, SingleVertexBufferDataMap},
        vertex_buffer_desc::VertexBufferDesc,
        vertex_buffer::{VertexBufferLayouts, KeyVertexBuffer, VertexBufferAllocator, EVertexBufferRange},
        indices::{AssetKeyBufferIndices, AssetResBufferIndices}
    },
    render_3d::shader::instance_code::EInstanceCode,
    rhi::{RenderQueue, device::RenderDevice}
};
use pi_share::Share;
///
/// 网格信息单独与 GameObject 绑定

use crate::{object::ObjectID, plugin::Plugin};

use self::{
    vertex_buffer_useinfo::*,
    sys_vertex_buffer_use::{PluginVertexBuffers, SysGeometryStatesInit},
    geometry::RenderGeometryEable, load::SysVertexBufferLoad
};

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

#[derive(Debug)]
pub enum ECommand {
    Desc(ObjectID, Vec<VertexBufferDesc>)
}
#[derive(Debug, Default)]
pub struct SingleGeometryVBCommands(pub Vec<ECommand>);

pub struct SysGeometryVBCommand;
impl TSystemStageInfo for SysGeometryVBCommand {

}
#[setup]
impl SysGeometryVBCommand {
    #[system]
    pub fn cmd(
        mut cmds: ResMut<SingleGeometryVBCommands>,
        mut geo_desc_cmd: Commands<GameObject, GeometryDesc>,
        mut geo_vb_cmd: Commands<GameObject, VertexBufferLayouts>,
        mut geo_enable_cmd: Commands<GameObject, RenderGeometryEable>,
        mut inscode_cmd: Commands<GameObject, EInstanceCode>,
    ) {
        let mut list = replace(&mut cmds.0, vec![]);

        list.drain(..).for_each(|cmd| {
            match cmd {
                ECommand::Desc(entity, descs) => {
                    // log::debug!(">>>>  GeometryDesc ");
                    geo_vb_cmd.insert(entity.clone(), VertexBufferLayouts::from(&descs));
                    geo_desc_cmd.insert(entity.clone(), GeometryDesc { list: descs });
                    geo_enable_cmd.insert(entity.clone(), RenderGeometryEable(true));
                    inscode_cmd.insert(entity.clone(), EInstanceCode(EInstanceCode::NONE));
                },
            }
        });
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
        descs: Vec<VertexBufferDesc>,
    ) -> &Self;
}

impl TInterfaceGeomtery for EnginShell {
    fn create_vertex_buffer(
        &self,
        key: KeyVertexBuffer,
        buffer: Vec<u8>,
    ) -> &Self {
        let world = self.world();
        let assert_mgr = world.get_resource::<Share<AssetMgr<EVertexBufferRange>>>().unwrap();
        if !assert_mgr.check_asset(&key) {
            let data_map = world.get_resource_mut::<SingleVertexBufferDataMap>().unwrap();
            data_map.add(&key, buffer);
        }

        self
    }
    fn use_geometry(
        &self,
        entity: ObjectID,
        descs: Vec<VertexBufferDesc>,
    ) -> &Self {
        let commands = self.world().get_resource_mut::<SingleGeometryVBCommands>().unwrap();
        commands.0.push(ECommand::Desc(entity, descs));

        self
    }
}

pub struct PluginGeometry;
impl Plugin for PluginGeometry {
    fn init(
        &mut self,
        engine: &mut crate::engine::Engine,
        stages: &mut crate::run_stage::RunStage,
    ) -> Result<(), crate::plugin::ErrorPlugin> {
        let world = engine.world_mut();
        world.insert_resource(SingleGeometryVBCommands::default());
        world.insert_resource(VertexBufferAllocator::new());
        world.insert_resource(AssetMgr::<EVertexBufferRange>::new(GarbageEmpty(), false, 1 * 1024 * 1024, 10 * 1000));

        world.insert_resource(VertexBufferLoader::<ObjectID, AssetResVBSlot01>::default());
        world.insert_resource(VertexBufferLoader::<ObjectID, AssetResVBSlot02>::default());
        world.insert_resource(VertexBufferLoader::<ObjectID, AssetResVBSlot03>::default());
        world.insert_resource(VertexBufferLoader::<ObjectID, AssetResVBSlot04>::default());
        world.insert_resource(VertexBufferLoader::<ObjectID, AssetResVBSlot05>::default());
        world.insert_resource(VertexBufferLoader::<ObjectID, AssetResVBSlot06>::default());
        world.insert_resource(VertexBufferLoader::<ObjectID, AssetResVBSlot07>::default());
        world.insert_resource(VertexBufferLoader::<ObjectID, AssetResVBSlot08>::default());
        world.insert_resource(VertexBufferLoader::<ObjectID, AssetResVBSlot09>::default());
        world.insert_resource(VertexBufferLoader::<ObjectID, AssetResVBSlot10>::default());
        world.insert_resource(VertexBufferLoader::<ObjectID, AssetResVBSlot11>::default());
        world.insert_resource(VertexBufferLoader::<ObjectID, AssetResVBSlot12>::default());
        world.insert_resource(VertexBufferLoader::<ObjectID, AssetResVBSlot13>::default());
        world.insert_resource(VertexBufferLoader::<ObjectID, AssetResVBSlot14>::default());
        world.insert_resource(VertexBufferLoader::<ObjectID, AssetResVBSlot15>::default());
        world.insert_resource(VertexBufferLoader::<ObjectID, AssetResVBSlot16>::default());
        world.insert_resource(VertexBufferLoader::<ObjectID, AssetResBufferIndices>::default());

        SysGeometryVBCommand::setup(world, stages.query_stage::<SysGeometryVBCommand>(ERunStageChap::Initial));
        SysVertexBufferLoad::setup(world, stages.query_stage::<SysVertexBufferLoad>(ERunStageChap::Uniform));
        stages.query_stage::<SysGeometryStatesInit>(ERunStageChap::Initial);
        PluginVertexBuffers.init(engine, stages);

        Ok(())
    }
}

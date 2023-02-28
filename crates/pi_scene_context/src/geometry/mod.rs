use std::mem::replace;

use pi_assets::{mgr::AssetMgr};
use pi_atom::Atom;
use pi_ecs::{prelude::{ResMut, Setup, Commands}};
use pi_ecs_macros::setup;
use pi_engine_shell::{engine_shell::EnginShell, object::GameObject, assets::sync_load::{AssetSyncWait, PluginAssetSyncLoad, InterfaceAssetSyncCreate}, run_stage::{TSystemStageInfo, ERunStageChap}};
use pi_render::{
    renderer::{
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
    geometry::RenderGeometryEable
};

pub mod vertex_buffer_useinfo;
pub mod sys_vertex_buffer_use;
pub mod geometry;
pub mod indices;
pub mod instance;
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
        buffer: &[u8],
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
        buffer: &[u8],
    ) -> &Self {
        let world = self.world();
        let assert_mgr = world.get_resource::<Share<AssetMgr<EVertexBufferRange>>>().unwrap();
        let queue = world.get_resource::<RenderQueue>().unwrap();
        let device = world.get_resource::<RenderDevice>().unwrap();
        if !assert_mgr.check_asset(&key) {
            let allocator = world.get_resource_mut::<VertexBufferAllocator>().unwrap();
            if let Some(buffer) = allocator.create_not_updatable_buffer(&device, &queue, buffer) {
                if let Some(buffer) = assert_mgr.insert(key.clone(), buffer) {
                    let list_wait = world.get_resource_mut::<AssetSyncWait<KeyVertexBuffer, AssetKeyBufferIndices, EVertexBufferRange, AssetResBufferIndices>>().unwrap();
                    list_wait.loaded(key.clone(), buffer.clone());
                    let list_wait = world.get_resource_mut::<AssetSyncWait<KeyVertexBuffer, AssetKeyVBSlot01, EVertexBufferRange, AssetResVBSlot01>>().unwrap();
                    list_wait.loaded(key.clone(), buffer.clone());
                    let list_wait = world.get_resource_mut::<AssetSyncWait<KeyVertexBuffer, AssetKeyVBSlot02, EVertexBufferRange, AssetResVBSlot02>>().unwrap();
                    list_wait.loaded(key.clone(), buffer.clone());
                    let list_wait = world.get_resource_mut::<AssetSyncWait<KeyVertexBuffer, AssetKeyVBSlot03, EVertexBufferRange, AssetResVBSlot03>>().unwrap();
                    list_wait.loaded(key.clone(), buffer.clone());
                    let list_wait = world.get_resource_mut::<AssetSyncWait<KeyVertexBuffer, AssetKeyVBSlot04, EVertexBufferRange, AssetResVBSlot04>>().unwrap();
                    list_wait.loaded(key.clone(), buffer.clone());
                    let list_wait = world.get_resource_mut::<AssetSyncWait<KeyVertexBuffer, AssetKeyVBSlot05, EVertexBufferRange, AssetResVBSlot05>>().unwrap();
                    list_wait.loaded(key.clone(), buffer.clone());
                    let list_wait = world.get_resource_mut::<AssetSyncWait<KeyVertexBuffer, AssetKeyVBSlot06, EVertexBufferRange, AssetResVBSlot06>>().unwrap();
                    list_wait.loaded(key.clone(), buffer.clone());
                    let list_wait = world.get_resource_mut::<AssetSyncWait<KeyVertexBuffer, AssetKeyVBSlot07, EVertexBufferRange, AssetResVBSlot07>>().unwrap();
                    list_wait.loaded(key.clone(), buffer.clone());
                    let list_wait = world.get_resource_mut::<AssetSyncWait<KeyVertexBuffer, AssetKeyVBSlot08, EVertexBufferRange, AssetResVBSlot08>>().unwrap();
                    list_wait.loaded(key.clone(), buffer.clone());
                    let list_wait = world.get_resource_mut::<AssetSyncWait<KeyVertexBuffer, AssetKeyVBSlot09, EVertexBufferRange, AssetResVBSlot09>>().unwrap();
                    list_wait.loaded(key.clone(), buffer.clone());
                }
            }
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

pub struct SysVertexBufferLoad;
impl TSystemStageInfo for SysVertexBufferLoad {
    fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
        vec![
            SysGeometryStatesInit::key(),
        ]
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
        SysGeometryVBCommand::setup(world, stages.query_stage::<SysGeometryVBCommand>(ERunStageChap::Initial));

        stages.query_stage::<SysGeometryStatesInit>(ERunStageChap::Initial);
        stages.query_stage::<SysVertexBufferLoad>(ERunStageChap::Initial);

        PluginAssetSyncLoad::<KeyVertexBuffer, AssetKeyBufferIndices, EVertexBufferRange, AssetResBufferIndices, SysGeometryStatesInit>::new(false, 60 * 1024 * 1024, 60 * 1000).init(engine, stages);
        PluginAssetSyncLoad::<KeyVertexBuffer, AssetKeyVBSlot01, EVertexBufferRange, AssetResVBSlot01, SysGeometryStatesInit>::new(false, 60 * 1024 * 1024, 60 * 1000).init(engine, stages);
        PluginAssetSyncLoad::<KeyVertexBuffer, AssetKeyVBSlot02, EVertexBufferRange, AssetResVBSlot02, SysGeometryStatesInit>::new(false, 60 * 1024 * 1024, 60 * 1000).init(engine, stages);
        PluginAssetSyncLoad::<KeyVertexBuffer, AssetKeyVBSlot03, EVertexBufferRange, AssetResVBSlot03, SysGeometryStatesInit>::new(false, 60 * 1024 * 1024, 60 * 1000).init(engine, stages);
        PluginAssetSyncLoad::<KeyVertexBuffer, AssetKeyVBSlot04, EVertexBufferRange, AssetResVBSlot04, SysGeometryStatesInit>::new(false, 60 * 1024 * 1024, 60 * 1000).init(engine, stages);
        PluginAssetSyncLoad::<KeyVertexBuffer, AssetKeyVBSlot05, EVertexBufferRange, AssetResVBSlot05, SysGeometryStatesInit>::new(false, 60 * 1024 * 1024, 60 * 1000).init(engine, stages);
        PluginAssetSyncLoad::<KeyVertexBuffer, AssetKeyVBSlot06, EVertexBufferRange, AssetResVBSlot06, SysGeometryStatesInit>::new(false, 60 * 1024 * 1024, 60 * 1000).init(engine, stages);
        PluginAssetSyncLoad::<KeyVertexBuffer, AssetKeyVBSlot07, EVertexBufferRange, AssetResVBSlot07, SysGeometryStatesInit>::new(false, 60 * 1024 * 1024, 60 * 1000).init(engine, stages);
        PluginAssetSyncLoad::<KeyVertexBuffer, AssetKeyVBSlot08, EVertexBufferRange, AssetResVBSlot08, SysGeometryStatesInit>::new(false, 60 * 1024 * 1024, 60 * 1000).init(engine, stages);
        PluginAssetSyncLoad::<KeyVertexBuffer, AssetKeyVBSlot09, EVertexBufferRange, AssetResVBSlot09, SysGeometryStatesInit>::new(false, 60 * 1024 * 1024, 60 * 1000).init(engine, stages);
        PluginAssetSyncLoad::<KeyVertexBuffer, AssetKeyVBSlot10, EVertexBufferRange, AssetResVBSlot10, SysGeometryStatesInit>::new(false, 60 * 1024 * 1024, 60 * 1000).init(engine, stages);
        PluginAssetSyncLoad::<KeyVertexBuffer, AssetKeyVBSlot11, EVertexBufferRange, AssetResVBSlot11, SysGeometryStatesInit>::new(false, 60 * 1024 * 1024, 60 * 1000).init(engine, stages);
        PluginAssetSyncLoad::<KeyVertexBuffer, AssetKeyVBSlot12, EVertexBufferRange, AssetResVBSlot12, SysGeometryStatesInit>::new(false, 60 * 1024 * 1024, 60 * 1000).init(engine, stages);
        PluginAssetSyncLoad::<KeyVertexBuffer, AssetKeyVBSlot13, EVertexBufferRange, AssetResVBSlot13, SysGeometryStatesInit>::new(false, 60 * 1024 * 1024, 60 * 1000).init(engine, stages);
        PluginAssetSyncLoad::<KeyVertexBuffer, AssetKeyVBSlot14, EVertexBufferRange, AssetResVBSlot14, SysGeometryStatesInit>::new(false, 60 * 1024 * 1024, 60 * 1000).init(engine, stages);
        PluginAssetSyncLoad::<KeyVertexBuffer, AssetKeyVBSlot15, EVertexBufferRange, AssetResVBSlot15, SysGeometryStatesInit>::new(false, 60 * 1024 * 1024, 60 * 1000).init(engine, stages);
        PluginAssetSyncLoad::<KeyVertexBuffer, AssetKeyVBSlot16, EVertexBufferRange, AssetResVBSlot16, SysGeometryStatesInit>::new(false, 60 * 1024 * 1024, 60 * 1000).init(engine, stages);

        PluginVertexBuffers.init(engine, stages);

        Ok(())
    }
}

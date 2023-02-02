use std::mem::replace;

use derive_deref::{Deref, DerefMut};
use pi_assets::{mgr::AssetMgr, asset::Handle};
use pi_atom::Atom;
use pi_ecs::{prelude::{ResMut, Setup, Commands}};
use pi_ecs_macros::setup;
use pi_engine_shell::{engine_shell::EnginShell, object::GameObject, assets::sync_load::{AssetSyncWait, PluginAssetSyncLoad, InterfaceAssetSyncCreate, AssetSyncLoad}, run_stage::{TSystemStageInfo, ERunStageChap}};
use pi_share::Share;
use render_data_container::{vertex_layout_key::KeyVertexLayouts, KeyVertexBuffer, VertexBuffer, VertexBufferPool};
use render_geometry::{vertex_data::{VertexBufferLayouts, VertexBufferDesc}, indices::{AssetKeyBufferIndices, AssetResBufferIndices}};
use render_shader::instance_code::EInstanceCode;
///
/// 网格信息单独与 GameObject 绑定

use crate::{object::ObjectID, plugin::Plugin};

use self::{vertex_buffer_useinfo::{AssetKeyVBSlot01, AssetKeyVBSlot02, AssetKeyVBSlot03, AssetKeyVBSlot04, AssetKeyVBSlot05, AssetKeyVBSlot06, AssetKeyVBSlot07, AssetKeyVBSlot08, AssetKeyVBSlot09, AssetResVBSlot02, AssetResVBSlot03, AssetResVBSlot04, AssetResVBSlot05, AssetResVBSlot06, AssetResVBSlot07, AssetResVBSlot08, AssetResVBSlot09, AssetResVBSlot01}, sys_vertex_buffer_use::{PluginVertexBuffers, SysGeometryStatesInit}, geometry::RenderGeometryEable, instance::{InstanceList, InstanceSourceRecord}};

pub mod vertex_buffer_useinfo;
pub mod sys_vertex_buffer_use;
pub mod geometry;
pub mod indices;
pub mod instance;
pub mod vertex_buffer;

pub type VDK = usize;
pub type GBID = Atom;

#[derive(Debug, Deref, DerefMut, Clone, Hash)]
pub struct AssetKeyVBLayouts(pub KeyVertexLayouts);

#[derive(Debug, Deref, DerefMut)]
pub struct AssetResVBLayouts(pub Handle<VertexBufferLayouts>);
impl From<Handle<VertexBufferLayouts>> for AssetResVBLayouts {
    fn from(value: Handle<VertexBufferLayouts>) -> Self {
        log::debug!("AssetResVBLayouts OK");
        Self(value)
    }
}

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
    Desc(ObjectID, Vec<VertexBufferDesc>, KeyVertexLayouts)
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
        mut geo_vb_cmd: Commands<GameObject, AssetKeyVBLayouts>,
        mut geo_enable_cmd: Commands<GameObject, RenderGeometryEable>,
        mut inscode_cmd: Commands<GameObject, EInstanceCode>,
    ) {
        let mut list = replace(&mut cmds.0, vec![]);

        list.drain(..).for_each(|cmd| {
            match cmd {
                ECommand::Desc(entity, descs, key) => {
                    log::debug!(">>>>  GeometryDesc ");
                    geo_desc_cmd.insert(entity.clone(), GeometryDesc { list: descs });
                    geo_vb_cmd.insert(entity.clone(), AssetKeyVBLayouts(key));
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
        buffer: VertexBuffer,
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
        buffer: VertexBuffer,
    ) -> &Self {
        let world = self.world();
        let assert_mgr = world.get_resource::<Share<AssetMgr<VertexBuffer>>>().unwrap();
        if !assert_mgr.check_asset(&key) {
            let buffer = assert_mgr.create_asset(key.clone(), buffer);

            let list_wait = world.get_resource_mut::<AssetSyncWait<KeyVertexBuffer, AssetKeyBufferIndices, VertexBuffer, AssetResBufferIndices>>().unwrap();
            list_wait.loaded(key.clone(), buffer.clone());

            let list_wait = world.get_resource_mut::<AssetSyncWait<KeyVertexBuffer, AssetKeyVBSlot01, VertexBuffer, AssetResVBSlot01>>().unwrap();
            list_wait.loaded(key.clone(), buffer.clone());
            let list_wait = world.get_resource_mut::<AssetSyncWait<KeyVertexBuffer, AssetKeyVBSlot02, VertexBuffer, AssetResVBSlot02>>().unwrap();
            list_wait.loaded(key.clone(), buffer.clone());
            let list_wait = world.get_resource_mut::<AssetSyncWait<KeyVertexBuffer, AssetKeyVBSlot03, VertexBuffer, AssetResVBSlot03>>().unwrap();
            list_wait.loaded(key.clone(), buffer.clone());
            let list_wait = world.get_resource_mut::<AssetSyncWait<KeyVertexBuffer, AssetKeyVBSlot04, VertexBuffer, AssetResVBSlot04>>().unwrap();
            list_wait.loaded(key.clone(), buffer.clone());
            let list_wait = world.get_resource_mut::<AssetSyncWait<KeyVertexBuffer, AssetKeyVBSlot05, VertexBuffer, AssetResVBSlot05>>().unwrap();
            list_wait.loaded(key.clone(), buffer.clone());
            let list_wait = world.get_resource_mut::<AssetSyncWait<KeyVertexBuffer, AssetKeyVBSlot06, VertexBuffer, AssetResVBSlot06>>().unwrap();
            list_wait.loaded(key.clone(), buffer.clone());
            let list_wait = world.get_resource_mut::<AssetSyncWait<KeyVertexBuffer, AssetKeyVBSlot07, VertexBuffer, AssetResVBSlot07>>().unwrap();
            list_wait.loaded(key.clone(), buffer.clone());
            let list_wait = world.get_resource_mut::<AssetSyncWait<KeyVertexBuffer, AssetKeyVBSlot08, VertexBuffer, AssetResVBSlot08>>().unwrap();
            list_wait.loaded(key.clone(), buffer.clone());
            let list_wait = world.get_resource_mut::<AssetSyncWait<KeyVertexBuffer, AssetKeyVBSlot09, VertexBuffer, AssetResVBSlot09>>().unwrap();
            list_wait.loaded(key.clone(), buffer.clone());
        }

        self
    }
    fn use_geometry(
        &self,
        entity: ObjectID,
        descs: Vec<VertexBufferDesc>,
    ) -> &Self {
        if let Ok(key) = VertexBufferLayouts::calc_key::<VertexBufferDesc>(&descs) {

            let asset_mgr = self.world().get_resource_mut::<Share<AssetMgr<VertexBufferLayouts>>>().unwrap();
            if !asset_mgr.check_asset(&key) {
                let data = VertexBufferLayouts::from(&descs);
                let data = asset_mgr.create_asset(key.clone(), data);

                let list_wait = self.world().get_resource_mut::<AssetSyncWait<KeyVertexLayouts, AssetKeyVBLayouts, VertexBufferLayouts, AssetResVBLayouts>>().unwrap();
                list_wait.loaded(key.clone(), data);
            }
            
            let commands = self.world().get_resource_mut::<SingleGeometryVBCommands>().unwrap();
            commands.0.push(ECommand::Desc(entity, descs, key.clone()));
        }

        self
    }
}

pub struct SysVertexBufferLoad;
impl TSystemStageInfo for SysVertexBufferLoad {
    fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
        vec![
            SysGeometryStatesInit::key(),
            AssetSyncLoad::<KeyVertexLayouts, AssetKeyVBLayouts, VertexBufferLayouts, AssetResVBLayouts, SysGeometryStatesInit>::key()
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
        world.insert_resource(VertexBufferPool::default());
        SysGeometryVBCommand::setup(world, stages.query_stage::<SysGeometryVBCommand>(ERunStageChap::Initial));

        stages.query_stage::<SysGeometryStatesInit>(ERunStageChap::Initial);

        PluginAssetSyncLoad::<KeyVertexLayouts, AssetKeyVBLayouts, VertexBufferLayouts, AssetResVBLayouts, SysGeometryStatesInit>::new(false, 2 * 1024 * 1024, 60 * 1000).init(engine, stages);
        stages.query_stage::<SysVertexBufferLoad>(ERunStageChap::Initial);

        PluginAssetSyncLoad::<KeyVertexBuffer, AssetKeyBufferIndices, VertexBuffer, AssetResBufferIndices, SysGeometryStatesInit>::new(false, 60 * 1024 * 1024, 60 * 1000).init(engine, stages);
        PluginAssetSyncLoad::<KeyVertexBuffer, AssetKeyVBSlot01, VertexBuffer, AssetResVBSlot01, SysGeometryStatesInit>::new(false, 60 * 1024 * 1024, 60 * 1000).init(engine, stages);
        PluginAssetSyncLoad::<KeyVertexBuffer, AssetKeyVBSlot02, VertexBuffer, AssetResVBSlot02, SysGeometryStatesInit>::new(false, 60 * 1024 * 1024, 60 * 1000).init(engine, stages);
        PluginAssetSyncLoad::<KeyVertexBuffer, AssetKeyVBSlot03, VertexBuffer, AssetResVBSlot03, SysGeometryStatesInit>::new(false, 60 * 1024 * 1024, 60 * 1000).init(engine, stages);
        PluginAssetSyncLoad::<KeyVertexBuffer, AssetKeyVBSlot04, VertexBuffer, AssetResVBSlot04, SysGeometryStatesInit>::new(false, 60 * 1024 * 1024, 60 * 1000).init(engine, stages);
        PluginAssetSyncLoad::<KeyVertexBuffer, AssetKeyVBSlot05, VertexBuffer, AssetResVBSlot05, SysGeometryStatesInit>::new(false, 60 * 1024 * 1024, 60 * 1000).init(engine, stages);
        PluginAssetSyncLoad::<KeyVertexBuffer, AssetKeyVBSlot06, VertexBuffer, AssetResVBSlot06, SysGeometryStatesInit>::new(false, 60 * 1024 * 1024, 60 * 1000).init(engine, stages);
        PluginAssetSyncLoad::<KeyVertexBuffer, AssetKeyVBSlot07, VertexBuffer, AssetResVBSlot07, SysGeometryStatesInit>::new(false, 60 * 1024 * 1024, 60 * 1000).init(engine, stages);
        PluginAssetSyncLoad::<KeyVertexBuffer, AssetKeyVBSlot08, VertexBuffer, AssetResVBSlot08, SysGeometryStatesInit>::new(false, 60 * 1024 * 1024, 60 * 1000).init(engine, stages);
        PluginAssetSyncLoad::<KeyVertexBuffer, AssetKeyVBSlot09, VertexBuffer, AssetResVBSlot09, SysGeometryStatesInit>::new(false, 60 * 1024 * 1024, 60 * 1000).init(engine, stages);

        // PluginVertexBufferLoad::new(false, 60 * 1024 * 1024, 60 * 1000).init(engine, stages);
        PluginVertexBuffers.init(engine, stages);

        Ok(())
    }
}

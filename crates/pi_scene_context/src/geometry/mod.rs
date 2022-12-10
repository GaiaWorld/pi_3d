use std::mem::replace;

use derive_deref::{Deref, DerefMut};
use pi_assets::{mgr::AssetMgr, asset::Handle};
use pi_atom::Atom;
use pi_ecs::{prelude::{ResMut, Query, Setup}, query::Write};
use pi_ecs_macros::setup;
use pi_engine_shell::{engine_shell::EnginShell, object::GameObject, assets::sync_load::{AssetSyncWait, PluginAssetSyncLoad, InterfaceAssetSyncCreate}};
use pi_share::Share;
use render_data_container::{vertex_layout_key::KeyVertexLayouts, KeyVertexBuffer, VertexBuffer};
use render_geometry::{vertex_data::{VertexBufferLayouts, VertexBufferDesc}, indices::{AssetKeyBufferIndices, AssetResBufferIndices}};
///
/// 网格信息单独与 GameObject 绑定

use crate::{object::ObjectID, plugin::Plugin};

use self::{vertex_buffer_useinfo::{AssetKeyVBSlot1, AssetKeyVBSlot2, AssetKeyVBSlot3, AssetKeyVBSlot4, AssetKeyVBSlot5, AssetKeyVBSlot6, AssetKeyVBSlot7, AssetKeyVBSlot8, AssetKeyVBSlot9, AssetResVBSlot2, AssetResVBSlot3, AssetResVBSlot4, AssetResVBSlot5, AssetResVBSlot6, AssetResVBSlot7, AssetResVBSlot8, AssetResVBSlot9, AssetResVBSlot1}, sys_vertex_buffer_use::PluginVertexBuffers};

pub mod vertex_buffer_useinfo;
pub mod sys_vertex_buffer_use;
pub mod geometry;

pub type VDK = usize;
pub type GBID = Atom;

#[derive(Debug, Deref, DerefMut, Clone, Hash)]
pub struct AssetKeyVBLayouts(pub KeyVertexLayouts);

#[derive(Debug, Deref, DerefMut)]
pub struct AssetResVBLayouts(pub Handle<VertexBufferLayouts>);
impl From<Handle<VertexBufferLayouts>> for AssetResVBLayouts {
    fn from(value: Handle<VertexBufferLayouts>) -> Self {
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
enum ECommand {
    Desc(ObjectID, Vec<VertexBufferDesc>, KeyVertexLayouts)
}
#[derive(Debug, Default)]
struct SingleGeometryCommands(pub Vec<ECommand>);
struct SysGeometryCommand;
#[setup]
impl SysGeometryCommand {
    #[system]
    pub fn cmd(
        mut cmds: ResMut<SingleGeometryCommands>,
        mut items: Query<GameObject, (Write<GeometryDesc>, Write<AssetKeyVBLayouts>)>,
    ) {
        let mut list = replace(&mut cmds.0, vec![]);

        list.drain(..).for_each(|cmd| {
            match cmd {
                ECommand::Desc(entity, descs, key) => {
                    if let Some((mut descwrite, mut keywrite)) = items.get_mut(entity) {
                        descwrite.write(GeometryDesc{ list: descs });
                        keywrite.write(AssetKeyVBLayouts(key));
                    }
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

            let list_wait = world.get_resource_mut::<AssetSyncWait<KeyVertexBuffer, AssetKeyVBSlot1, VertexBuffer, AssetResVBSlot1>>().unwrap();
            list_wait.loaded(key.clone(), buffer.clone());
            let list_wait = world.get_resource_mut::<AssetSyncWait<KeyVertexBuffer, AssetKeyVBSlot2, VertexBuffer, AssetResVBSlot2>>().unwrap();
            list_wait.loaded(key.clone(), buffer.clone());
            let list_wait = world.get_resource_mut::<AssetSyncWait<KeyVertexBuffer, AssetKeyVBSlot3, VertexBuffer, AssetResVBSlot3>>().unwrap();
            list_wait.loaded(key.clone(), buffer.clone());
            let list_wait = world.get_resource_mut::<AssetSyncWait<KeyVertexBuffer, AssetKeyVBSlot4, VertexBuffer, AssetResVBSlot4>>().unwrap();
            list_wait.loaded(key.clone(), buffer.clone());
            let list_wait = world.get_resource_mut::<AssetSyncWait<KeyVertexBuffer, AssetKeyVBSlot5, VertexBuffer, AssetResVBSlot5>>().unwrap();
            list_wait.loaded(key.clone(), buffer.clone());
            let list_wait = world.get_resource_mut::<AssetSyncWait<KeyVertexBuffer, AssetKeyVBSlot6, VertexBuffer, AssetResVBSlot6>>().unwrap();
            list_wait.loaded(key.clone(), buffer.clone());
            let list_wait = world.get_resource_mut::<AssetSyncWait<KeyVertexBuffer, AssetKeyVBSlot7, VertexBuffer, AssetResVBSlot7>>().unwrap();
            list_wait.loaded(key.clone(), buffer.clone());
            let list_wait = world.get_resource_mut::<AssetSyncWait<KeyVertexBuffer, AssetKeyVBSlot8, VertexBuffer, AssetResVBSlot8>>().unwrap();
            list_wait.loaded(key.clone(), buffer.clone());
            let list_wait = world.get_resource_mut::<AssetSyncWait<KeyVertexBuffer, AssetKeyVBSlot9, VertexBuffer, AssetResVBSlot9>>().unwrap();
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
            
            let commands = self.world().get_resource_mut::<SingleGeometryCommands>().unwrap();
            commands.0.push(ECommand::Desc(entity, descs, key.clone()));
        }

        self
    }
}

pub struct PluginBuildinGeometry;
impl Plugin for PluginBuildinGeometry {
    fn init(
        &mut self,
        engine: &mut crate::engine::Engine,
        stages: &mut crate::run_stage::RunStage,
    ) -> Result<(), crate::plugin::ErrorPlugin> {
        
        PluginAssetSyncLoad::<KeyVertexLayouts, AssetKeyVBLayouts, VertexBufferLayouts, AssetResVBLayouts>::new(false, 2 * 1024 * 1024, 60 * 1000).init(engine, stages);

        PluginAssetSyncLoad::<KeyVertexBuffer, AssetKeyBufferIndices, VertexBuffer, AssetResBufferIndices>::new(false, 60 * 1024 * 1024, 60 * 1000).init(engine, stages);
        PluginAssetSyncLoad::<KeyVertexBuffer, AssetKeyVBSlot1, VertexBuffer, AssetResVBSlot1>::new(false, 60 * 1024 * 1024, 60 * 1000).init(engine, stages);
        PluginAssetSyncLoad::<KeyVertexBuffer, AssetKeyVBSlot2, VertexBuffer, AssetResVBSlot2>::new(false, 60 * 1024 * 1024, 60 * 1000).init(engine, stages);
        PluginAssetSyncLoad::<KeyVertexBuffer, AssetKeyVBSlot3, VertexBuffer, AssetResVBSlot3>::new(false, 60 * 1024 * 1024, 60 * 1000).init(engine, stages);
        PluginAssetSyncLoad::<KeyVertexBuffer, AssetKeyVBSlot4, VertexBuffer, AssetResVBSlot4>::new(false, 60 * 1024 * 1024, 60 * 1000).init(engine, stages);
        PluginAssetSyncLoad::<KeyVertexBuffer, AssetKeyVBSlot5, VertexBuffer, AssetResVBSlot5>::new(false, 60 * 1024 * 1024, 60 * 1000).init(engine, stages);
        PluginAssetSyncLoad::<KeyVertexBuffer, AssetKeyVBSlot6, VertexBuffer, AssetResVBSlot6>::new(false, 60 * 1024 * 1024, 60 * 1000).init(engine, stages);
        PluginAssetSyncLoad::<KeyVertexBuffer, AssetKeyVBSlot7, VertexBuffer, AssetResVBSlot7>::new(false, 60 * 1024 * 1024, 60 * 1000).init(engine, stages);
        PluginAssetSyncLoad::<KeyVertexBuffer, AssetKeyVBSlot8, VertexBuffer, AssetResVBSlot8>::new(false, 60 * 1024 * 1024, 60 * 1000).init(engine, stages);
        PluginAssetSyncLoad::<KeyVertexBuffer, AssetKeyVBSlot9, VertexBuffer, AssetResVBSlot9>::new(false, 60 * 1024 * 1024, 60 * 1000).init(engine, stages);

        PluginVertexBuffers.init(engine, stages);

        let world = engine.world_mut();
        world.insert_resource(SingleGeometryCommands::default());
        SysGeometryCommand::setup(world, stages.command_stage());

        Ok(())
    }
}

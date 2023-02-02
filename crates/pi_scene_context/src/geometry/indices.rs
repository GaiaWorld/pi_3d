
use pi_ecs::{prelude::{ResMut, Setup, Commands}};
use pi_ecs_macros::setup;
use pi_engine_shell::{assets::sync_load::{PluginAssetSyncLoad, AssetSyncLoad}, run_stage::{TSystemStageInfo, ERunStageChap}};
use render_data_container::{VertexBuffer, KeyVertexBuffer};
use render_geometry::indices::{IndicesBufferDesc, AssetKeyBufferIndices, AssetResBufferIndices};

use crate::{object::{ObjectID, GameObject}, plugin::Plugin, engine::Engine};

#[derive(Debug)]
pub enum ECommand {
    Use(ObjectID, IndicesBufferDesc),
}
#[derive(Debug, Default)]
pub struct CommandListBufferIndices {
    pub list: Vec<ECommand>,
}
pub struct SysGeometryIndicesCommand;
impl TSystemStageInfo for SysGeometryIndicesCommand {

}
#[setup]
impl SysGeometryIndicesCommand {
    #[system]
    pub fn cmd(
        mut cmds: ResMut<CommandListBufferIndices>,
        mut desc_cmd: Commands<GameObject, IndicesBufferDesc>,
        mut key_cmd: Commands<GameObject, AssetKeyBufferIndices>,
    ) {
        let mut list = std::mem::replace(&mut cmds.list, vec![]);
        list.drain(..).for_each(|cmd| {
            match cmd {
                ECommand::Use(entity, desc) => {
                    key_cmd.insert(entity.clone(), AssetKeyBufferIndices(desc.buffer.clone()));
                    desc_cmd.insert(entity.clone(), desc);
                },
            }
        });
    }
}

pub trait InterfaceBufferIndices {
    fn use_indices(
        & self,
        entity: ObjectID,
        info: IndicesBufferDesc,
    ) -> &Self;
}
impl InterfaceBufferIndices for Engine {
    fn use_indices(
        & self,
        entity: ObjectID,
        info: IndicesBufferDesc,
    ) -> &Self {
        let commands = self.world().get_resource_mut::<CommandListBufferIndices>().unwrap();
        commands.list.push(ECommand::Use(entity, info));

        self
    }
}

pub type SysInstanceBufferLoad = AssetSyncLoad<KeyVertexBuffer, AssetKeyBufferIndices, VertexBuffer, AssetResBufferIndices, SysGeometryIndicesCommand>;

pub struct PluginBufferIndices;
impl Plugin for PluginBufferIndices {
    fn init(
        &mut self,
        engine: &mut crate::engine::Engine,
        stages: &mut crate::run_stage::RunStage,
    ) -> Result<(), crate::plugin::ErrorPlugin> {

        let world = engine.world_mut();
        world.insert_resource(CommandListBufferIndices::default());
        SysGeometryIndicesCommand::setup(world, stages.query_stage::<SysGeometryIndicesCommand>(ERunStageChap::Initial));

        PluginAssetSyncLoad::<KeyVertexBuffer, AssetKeyBufferIndices, VertexBuffer, AssetResBufferIndices, SysGeometryIndicesCommand>::new(false, 60 * 1024 * 1024, 60 * 1000).init(engine, stages);

        Ok(())
    }
}

use std::ops::Range;

use derive_deref::{Deref, DerefMut};
use pi_assets::asset::Handle;
use pi_ecs::{prelude::{ResMut, Query, Setup}, query::Write};
use pi_ecs_macros::setup;
use pi_engine_shell::{assets::sync_load::PluginAssetSyncLoad};
use render_data_container::{EVertexDataFormat, VertexBuffer, KeyVertexBuffer, TVertexBufferMeta, TIndicesMeta};
use render_geometry::indices::{IndicesBufferDesc, AssetKeyBufferIndices, AssetResBufferIndices};

use crate::{object::{ObjectID, GameObject}, plugin::Plugin, engine::Engine};

#[derive(Debug)]
enum ECommand {
    Use(ObjectID, IndicesBufferDesc),
}
#[derive(Debug, Default)]
struct CommandListBufferIndices {
    pub list: Vec<ECommand>,
}
struct SysCommand;
#[setup]
impl SysCommand {
    #[system]
    pub fn cmd(
        mut cmds: ResMut<CommandListBufferIndices>,
        mut items: Query<GameObject, (Write<IndicesBufferDesc>, Write<AssetKeyBufferIndices>)>,
    ) {
        let mut list = std::mem::replace(&mut cmds.list, vec![]);
        list.drain(..).for_each(|cmd| {
            match cmd {
                ECommand::Use(entity, desc) => {
                    if let Some((mut itemdesc, mut itemkey)) = items.get_mut(entity) {
                        itemkey.write(AssetKeyBufferIndices(desc.buffer.clone()));
                        itemdesc.write(desc);
                    }
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

pub struct PluginBufferIndices;
impl Plugin for PluginBufferIndices {
    fn init(
        &mut self,
        engine: &mut crate::engine::Engine,
        stages: &mut crate::run_stage::RunStage,
    ) -> Result<(), crate::plugin::ErrorPlugin> {

        PluginAssetSyncLoad::<KeyVertexBuffer, AssetKeyBufferIndices, VertexBuffer, AssetResBufferIndices>::new(false, 60 * 1024 * 1024, 60 * 1000).init(engine, stages);

        let world = engine.world_mut();
        world.insert_resource(CommandListBufferIndices::default());
        SysCommand::setup(world, stages.command_stage());

        Ok(())
    }
}

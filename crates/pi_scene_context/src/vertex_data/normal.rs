use derive_deref::{Deref, DerefMut};
use pi_assets::asset::Handle;
use pi_ecs::{prelude::{ResMut, Query, Setup}, query::Write};
use pi_ecs_macros::setup;
use pi_engine_shell::{assets::sync_load::PluginAssetSyncLoad};
use render_data_container::{EVertexDataFormat, VertexBuffer, KeyVertexBuffer, TVertexBufferMeta, TAttributeMeta};

use crate::{object::{ObjectID, GameObject}, plugin::Plugin, engine::Engine};


#[derive(Debug)]
enum ECommand {
    Use(ObjectID, KeyVertexBuffer),
}
#[derive(Debug, Default)]
struct CommandListBufferNormal {
    pub list: Vec<ECommand>,
}
struct SysCommand;
#[setup]
impl SysCommand {
    #[system]
    pub fn cmd(
        mut cmds: ResMut<CommandListBufferNormal>,
        mut items: Query<GameObject, Write<AssetKeyBufferNormal>>,
    ) {
        let mut list = std::mem::replace(&mut cmds.list, vec![]);
        list.drain(..).for_each(|cmd| {
            match cmd {
                ECommand::Use(entity, key) => {
                    if let Some(mut item) = items.get_mut(entity) {
                        item.write(AssetKeyBufferNormal(key.clone()));
                    }
                },
            }
        });
    }
}

#[derive(Debug, Deref, DerefMut, Clone, Hash)]
pub struct AssetKeyBufferNormal(pub KeyVertexBuffer);

#[derive(Deref, DerefMut)]
pub struct AssetResBufferNormal(pub Handle<VertexBuffer>);
impl From<Handle<VertexBuffer>> for AssetResBufferNormal {
    fn from(value: Handle<VertexBuffer>) -> Self {
        Self(value)
    }
}
impl AssetResBufferNormal {
    pub const NUMBER_BYTES: wgpu::BufferAddress = 4;
    pub const NUMBER_COUNT: wgpu::BufferAddress = 3;
    pub const OFFSET: u32 = 0 * 4;
}
impl TVertexBufferMeta for AssetResBufferNormal {
    const DATA_FORMAT: EVertexDataFormat = EVertexDataFormat::F32;
    const STEP_MODE: wgpu::VertexStepMode = wgpu::VertexStepMode::Vertex;
    fn size_per_vertex(&self) -> wgpu::BufferAddress {
        Self::NUMBER_COUNT * Self::NUMBER_BYTES
    }

    fn number_per_vertex(&self) -> wgpu::BufferAddress {
        Self::NUMBER_COUNT
    }
}

pub struct AttributeNormal {
    pub format: wgpu::VertexFormat,
    pub offset: wgpu::BufferAddress,
    pub shader_location: u32,
}
impl TAttributeMeta for AttributeNormal {
    fn format(&self) -> wgpu::VertexFormat {
        self.format
    }

    fn offset(&self) -> wgpu::BufferAddress {
        self.offset
    }

    fn shader_location(&self) -> u32 {
        self.shader_location
    }
}

pub trait InterfaceBufferNormal {
    fn use_vertex_data_normal(
        & self,
        entity: ObjectID,
        key: KeyVertexBuffer,
    ) -> &Self;
}
impl InterfaceBufferNormal for Engine {
    fn use_vertex_data_normal(
        & self,
        entity: ObjectID,
        key: KeyVertexBuffer,
    ) -> &Self {
        let commands = self.world().get_resource_mut::<CommandListBufferNormal>().unwrap();
        commands.list.push(ECommand::Use(entity, key));

        self
    }
}

pub struct PluginBufferNormal;
impl Plugin for PluginBufferNormal {
    fn init(
        &mut self,
        engine: &mut crate::engine::Engine,
        stages: &mut crate::run_stage::RunStage,
    ) -> Result<(), crate::plugin::ErrorPlugin> {

        PluginAssetSyncLoad::<KeyVertexBuffer, AssetKeyBufferNormal, VertexBuffer, AssetResBufferNormal>::new(false, 60 * 1024 * 1024, 60 * 1000).init(engine, stages);

        let world = engine.world_mut();
        world.insert_resource(CommandListBufferNormal::default());
        SysCommand::setup(world, stages.command_stage());

        Ok(())
    }
}

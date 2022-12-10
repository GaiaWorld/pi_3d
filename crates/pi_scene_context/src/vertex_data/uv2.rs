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
struct CommandListBufferUV2 {
    pub list: Vec<ECommand>,
}
struct SysCommand;
#[setup]
impl SysCommand {
    #[system]
    pub fn cmd(
        mut cmds: ResMut<CommandListBufferUV2>,
        mut items: Query<GameObject, Write<AssetKeyBufferUV2>>,
    ) {
        let mut list = std::mem::replace(&mut cmds.list, vec![]);
        list.drain(..).for_each(|cmd| {
            match cmd {
                ECommand::Use(entity, key) => {
                    if let Some(mut item) = items.get_mut(entity) {
                        item.write(AssetKeyBufferUV2(key.clone()));
                    }
                },
            }
        });
    }
}

#[derive(Debug, Deref, DerefMut, Clone, Hash)]
pub struct AssetKeyBufferUV2(pub KeyVertexBuffer);

#[derive(Deref, DerefMut)]
pub struct AssetResBufferUV2(pub Handle<VertexBuffer>);
impl From<Handle<VertexBuffer>> for AssetResBufferUV2 {
    fn from(value: Handle<VertexBuffer>) -> Self {
        Self(value)
    }
}
impl AssetResBufferUV2 {
    pub const NUMBER_BYTES: wgpu::BufferAddress = 4;
    pub const NUMBER_COUNT: wgpu::BufferAddress = 3;
    pub const OFFSET: u32 = 0 * 4;
}
impl TVertexBufferMeta for AssetResBufferUV2 {
    const DATA_FORMAT: EVertexDataFormat = EVertexDataFormat::F32;
    const STEP_MODE: wgpu::VertexStepMode = wgpu::VertexStepMode::Vertex;
    fn size_per_vertex(&self) -> wgpu::BufferAddress {
        Self::NUMBER_COUNT * Self::NUMBER_BYTES
    }

    fn number_per_vertex(&self) -> wgpu::BufferAddress {
        Self::NUMBER_COUNT
    }
}

pub struct AttributeUV2 {
    pub format: wgpu::VertexFormat,
    pub offset: wgpu::BufferAddress,
    pub shader_location: u32,
}
impl TAttributeMeta for AttributeUV2 {
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

pub trait InterfaceBufferUV2 {
    fn use_vertex_data_uv(
        & self,
        entity: ObjectID,
        key: KeyVertexBuffer,
    ) -> &Self;
}
impl InterfaceBufferUV2 for Engine {
    fn use_vertex_data_uv(
        & self,
        entity: ObjectID,
        key: KeyVertexBuffer,
    ) -> &Self {
        let commands = self.world().get_resource_mut::<CommandListBufferUV2>().unwrap();
        commands.list.push(ECommand::Use(entity, key));

        self
    }
}
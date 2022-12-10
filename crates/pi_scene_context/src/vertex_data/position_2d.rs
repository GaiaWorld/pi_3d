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
struct CommandListBufferPosition2D {
    pub list: Vec<ECommand>,
}
struct SysCommand;
#[setup]
impl SysCommand {
    #[system]
    pub fn cmd(
        mut cmds: ResMut<CommandListBufferPosition2D>,
        mut items: Query<GameObject, Write<AssetKeyBufferPosition2D>>,
    ) {
        let mut list = std::mem::replace(&mut cmds.list, vec![]);
        list.drain(..).for_each(|cmd| {
            match cmd {
                ECommand::Use(entity, key) => {
                    if let Some(mut item) = items.get_mut(entity) {
                        item.write(AssetKeyBufferPosition2D(key.clone()));
                    }
                },
            }
        });
    }
}

#[derive(Debug, Deref, DerefMut, Clone, Hash)]
pub struct AssetKeyBufferPosition2D(pub KeyVertexBuffer);

#[derive(Deref, DerefMut)]
pub struct AssetResBufferPosition2D(pub Handle<VertexBuffer>);
impl From<Handle<VertexBuffer>> for AssetResBufferPosition2D {
    fn from(value: Handle<VertexBuffer>) -> Self {
        Self(value)
    }
}
impl AssetResBufferPosition2D {
    pub const NUMBER_BYTES: wgpu::BufferAddress = 4;
    pub const NUMBER_COUNT: wgpu::BufferAddress = 3;
    pub const OFFSET: u32 = 0 * 4;
}
impl TVertexBufferMeta for AssetResBufferPosition2D {
    const DATA_FORMAT: EVertexDataFormat = EVertexDataFormat::F32;
    const STEP_MODE: wgpu::VertexStepMode = wgpu::VertexStepMode::Vertex;
    fn size_per_vertex(&self) -> wgpu::BufferAddress {
        Self::NUMBER_COUNT * Self::NUMBER_BYTES
    }

    fn number_per_vertex(&self) -> wgpu::BufferAddress {
        Self::NUMBER_COUNT
    }
}

pub struct AttributePosition2D {
    pub format: wgpu::VertexFormat,
    pub offset: wgpu::BufferAddress,
    pub shader_location: u32,
}
impl TAttributeMeta for AttributePosition2D {
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

pub trait InterfaceBufferPosition2D {
    fn use_vertex_data_position(
        & self,
        entity: ObjectID,
        key: KeyVertexBuffer,
    ) -> &Self;
}
impl InterfaceBufferPosition2D for Engine {
    fn use_vertex_data_position(
        & self,
        entity: ObjectID,
        key: KeyVertexBuffer,
    ) -> &Self {
        let commands = self.world().get_resource_mut::<CommandListBufferPosition2D>().unwrap();
        commands.list.push(ECommand::Use(entity, key));

        self
    }
}
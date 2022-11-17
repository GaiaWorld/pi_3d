use pi_ecs::{prelude::{ResMut, Query, Setup}, query::Write};
use pi_ecs_macros::setup;
use pi_engine_shell::object::InterfaceObject;
use render_data_container::{EVertexDataFormat, GeometryBuffer, GeometryBufferPool};
use render_geometry::geometry::{VertexAttributeBufferMeta, VertexAttributeMeta};

use crate::{object::{ObjectID, GameObject}, geometry::GBID, plugin::Plugin, resources::SingleGeometryBufferPool};

#[derive(Debug, Clone, Copy)]
pub struct IDAttributeUV(pub ObjectID);

#[derive(Debug)]
pub struct AttributeUV {
    pub meta: VertexAttributeBufferMeta<GBID>,
}
impl AttributeUV {
    pub const UV: u32 = 2;
    pub const UV_OFFSET: u32 = 0 * 4;
    pub const UV_FORMAT: wgpu::VertexFormat = wgpu::VertexFormat::Float32x2;
    pub const UV_LOCATION: u32 = 2;

    pub const ATTRIBUTES: [wgpu::VertexAttribute; 1] = [
        wgpu::VertexAttribute {
            format: Self::UV_FORMAT,
            offset: Self::UV_OFFSET as wgpu::BufferAddress,
            shader_location: Self::UV_LOCATION,
        }
    ];
}
impl VertexAttributeMeta for AttributeUV {
    const SLOT: u32 = 2;

    const SIZE_PER_VERTEX: u32 = Self::UV_OFFSET + Self::UV * 4;

    const STEP_MODE: wgpu::VertexStepMode = wgpu::VertexStepMode::Vertex;
    const FORMAT: EVertexDataFormat = EVertexDataFormat::F32;
}

#[derive(Debug)]
pub enum AttributeUVCommand {
    Create(ObjectID, AttributeUV)
}

#[derive(Debug, Default)]
pub struct SingleAttributeUVCommandList {
    pub list: Vec<AttributeUVCommand>,
}

pub struct SysAttributeUVCommand;
#[setup]
impl SysAttributeUVCommand {
    #[system]
    fn sys(
        mut commands: ResMut<SingleAttributeUVCommandList>,
        mut colors: Query<GameObject, Write<AttributeUV>>,
    ) {
        commands.list.drain(..).for_each(|cmd| {
            match cmd {
                AttributeUVCommand::Create(entity, value) => {
                    match colors.get_mut(entity) {
                        Some(mut color) => {
                            color.insert_no_notify(value);
                        },
                        None => {
                            
                        },
                    }
                },
            }
        });
    }
}

#[derive(Debug)]
pub enum IDAttributeUVCommand {
    Create(ObjectID, IDAttributeUV)
}

#[derive(Debug, Default)]
pub struct SingleIDAttributeUVCommandList {
    pub list: Vec<IDAttributeUVCommand>,
}

pub struct SysIDAttributeUVCommand;
#[setup]
impl SysIDAttributeUVCommand {
    #[system]
    fn sys(
        mut commands: ResMut<SingleIDAttributeUVCommandList>,
        mut colors: Query<GameObject, Write<IDAttributeUV>>,
    ) {
        commands.list.drain(..).for_each(|cmd| {
            match cmd {
                IDAttributeUVCommand::Create(entity, value) => {
                    match colors.get_mut(entity) {
                        Some(mut color) => {
                            color.insert_no_notify(value);
                        },
                        None => {
                            
                        },
                    }
                },
            }
        });
    }
}

pub struct PluginAttributeUV;
impl Plugin for PluginAttributeUV {
    fn init(
        &mut self,
        engine: &mut crate::engine::Engine,
        stages: &mut crate::run_stage::RunStage,
    ) -> Result<(), crate::plugin::ErrorPlugin> {
        let world = engine.world_mut();

        SysAttributeUVCommand::setup(world, stages.command_stage());
        SysIDAttributeUVCommand::setup(world, stages.command_stage());

        world.insert_resource(SingleAttributeUVCommandList::default());
        world.insert_resource(SingleIDAttributeUVCommandList::default());

        Ok(())
    }
}

pub trait InterfaceAttributeUV {
    fn create_vertex_data_uv(
        & self,
        data: GeometryBuffer,
    ) -> ObjectID;
}
impl InterfaceAttributeUV for crate::engine::Engine {
    fn create_vertex_data_uv(
        & self,
        data: GeometryBuffer,
    ) -> ObjectID {
        let entity = self.new_object();
        let world = self.world();

        let data_size = data.size();
        let gbp = world.get_resource_mut::<SingleGeometryBufferPool>().unwrap();
        let buffer_id = gbp.insert(data);

        let data = AttributeUV {
            meta: VertexAttributeBufferMeta {
                buffer_id,
                start: 0,
                end: data_size * 4,
                data_bytes_size: 3 * 4,
                data_count: data_size / 3,
            },
        };

        let commands = world.get_resource_mut::<SingleAttributeUVCommandList>().unwrap();
        commands.list.push(AttributeUVCommand::Create(entity, data));

        entity
    }
}
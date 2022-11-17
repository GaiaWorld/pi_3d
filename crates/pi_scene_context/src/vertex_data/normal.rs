use pi_ecs::{prelude::{ResMut, Query, Setup}, query::Write};
use pi_ecs_macros::setup;
use pi_engine_shell::object::InterfaceObject;
use render_data_container::{EVertexDataFormat, GeometryBuffer, GeometryBufferPool};
use render_geometry::geometry::{VertexAttributeBufferMeta, VertexAttributeMeta};

use crate::{object::{ObjectID, GameObject}, geometry::GBID, plugin::Plugin, resources::SingleGeometryBufferPool};


#[derive(Debug, Clone, Copy)]
pub struct IDAttributeNormal(pub ObjectID);
#[derive(Debug)]
pub struct AttributeNormal {
    pub meta: VertexAttributeBufferMeta<GBID>,
}
impl AttributeNormal {
    pub const NORMAL: u32 = 3;
    pub const NORMAL_OFFSET: u32 = 0 * 4;
    pub const NORMAL_FORMAT: wgpu::VertexFormat = wgpu::VertexFormat::Float32x3;
    pub const NORMAL_LOCATION: u32 = 1;
    pub const ATTRIBUTES: [wgpu::VertexAttribute; 1] = [
        wgpu::VertexAttribute {
            format: Self::NORMAL_FORMAT,
            offset: Self::NORMAL_OFFSET as wgpu::BufferAddress,
            shader_location: Self::NORMAL_LOCATION,
        }
    ];
}
impl VertexAttributeMeta for AttributeNormal {
    const SLOT: u32 = 1;
    const SIZE_PER_VERTEX: u32 = Self::NORMAL_OFFSET + Self::NORMAL * 4;
    const STEP_MODE: wgpu::VertexStepMode = wgpu::VertexStepMode::Vertex;
    const FORMAT: EVertexDataFormat = EVertexDataFormat::F32;
}

#[derive(Debug)]
pub enum AttributeNormalCommand {
    Create(ObjectID, AttributeNormal)
}

#[derive(Debug, Default)]
pub struct SingleAttributeNormalCommandList {
    pub list: Vec<AttributeNormalCommand>,
}

pub struct SysAttributeNormalCommand;
#[setup]
impl SysAttributeNormalCommand {
    #[system]
    fn sys(
        mut commands: ResMut<SingleAttributeNormalCommandList>,
        mut colors: Query<GameObject, Write<AttributeNormal>>,
    ) {
        commands.list.drain(..).for_each(|cmd| {
            match cmd {
                AttributeNormalCommand::Create(entity, value) => {
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
pub enum IDAttributeNormalCommand {
    Create(ObjectID, IDAttributeNormal)
}

#[derive(Debug, Default)]
pub struct SingleIDAttributeNormalCommandList {
    pub list: Vec<IDAttributeNormalCommand>,
}

pub struct SysIDAttributeNormalCommand;
#[setup]
impl SysIDAttributeNormalCommand {
    #[system]
    fn sys(
        mut commands: ResMut<SingleIDAttributeNormalCommandList>,
        mut colors: Query<GameObject, Write<IDAttributeNormal>>,
    ) {
        commands.list.drain(..).for_each(|cmd| {
            match cmd {
                IDAttributeNormalCommand::Create(entity, value) => {
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

pub struct PluginAttributeNormal;
impl Plugin for PluginAttributeNormal {
    fn init(
        &mut self,
        engine: &mut crate::engine::Engine,
        stages: &mut crate::run_stage::RunStage,
    ) -> Result<(), crate::plugin::ErrorPlugin> {
        let world = engine.world_mut();

        SysAttributeNormalCommand::setup(world, stages.command_stage());
        SysIDAttributeNormalCommand::setup(world, stages.command_stage());

        world.insert_resource(SingleAttributeNormalCommandList::default());
        world.insert_resource(SingleIDAttributeNormalCommandList::default());

        Ok(())
    }
}

pub trait InterfaceAttributeNormal {
    fn create_vertex_data_normal(
        &self,
        data: GeometryBuffer,
    ) -> ObjectID;
}
impl InterfaceAttributeNormal for crate::engine::Engine {
    fn create_vertex_data_normal(
        &self,
        data: GeometryBuffer,
    ) -> ObjectID {
        let entity = self.new_object();
        let world = self.world();

        let data_size = data.size();
        let gbp = world.get_resource_mut::<SingleGeometryBufferPool>().unwrap();
        let buffer_id = gbp.insert(data);

        let data = AttributeNormal {
            meta: VertexAttributeBufferMeta {
                buffer_id,
                start: 0,
                end: data_size * 4,
                data_bytes_size: 3 * 4,
                data_count: data_size / 3,
            },
        };

        let commands = world.get_resource_mut::<SingleAttributeNormalCommandList>().unwrap();
        commands.list.push(AttributeNormalCommand::Create(entity, data));

        entity
    }
}
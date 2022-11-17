use pi_ecs::{prelude::{ResMut, Query, Setup}, query::Write};
use pi_ecs_macros::setup;
use pi_engine_shell::object::InterfaceObject;
use render_data_container::{EVertexDataFormat, GeometryBuffer, GeometryBufferPool};
use render_geometry::geometry::{VertexAttributeBufferMeta, VertexAttributeMeta};

use crate::{object::{ObjectID, GameObject}, geometry::GBID, plugin::Plugin, engine::Engine, resources::SingleGeometryBufferPool};

#[derive(Debug, Clone, Copy)]
pub struct IDAttributeColor4(pub ObjectID);
#[derive(Debug)]
pub struct AttributeColor4 {
    pub meta: VertexAttributeBufferMeta<GBID>,
}
impl AttributeColor4 {
    pub const COLOR: u32 = 4;
    pub const COLOR_OFFSET: u32 = 0 * 4;
    pub const COLOR_FORMAT: wgpu::VertexFormat = wgpu::VertexFormat::Float32x4;
    pub const COLOR_LOCATION: u32 = 3;

    pub const ATTRIBUTES: [wgpu::VertexAttribute; 1] = [
        wgpu::VertexAttribute {
            format: Self::COLOR_FORMAT,
            offset: Self::COLOR_OFFSET as wgpu::BufferAddress,
            shader_location: Self::COLOR_LOCATION,
        }
    ];
}
impl VertexAttributeMeta for AttributeColor4 {
    const SLOT: u32 = 3;

    const SIZE_PER_VERTEX: u32 = Self::COLOR_OFFSET + Self::COLOR * 4;

    const STEP_MODE: wgpu::VertexStepMode = wgpu::VertexStepMode::Vertex;
    const FORMAT: EVertexDataFormat = EVertexDataFormat::F32;
}


#[derive(Debug)]
pub enum AttributeColor4Command {
    Create(ObjectID, AttributeColor4)
}

#[derive(Debug, Default)]
pub struct SingleAttributeColor4CommandList {
    pub list: Vec<AttributeColor4Command>,
}

pub struct SysAttributeColor4Command;
#[setup]
impl SysAttributeColor4Command {
    #[system]
    fn sys(
        mut commands: ResMut<SingleAttributeColor4CommandList>,
        mut colors: Query<GameObject, Write<AttributeColor4>>,
    ) {
        commands.list.drain(..).for_each(|cmd| {
            match cmd {
                AttributeColor4Command::Create(entity, value) => {
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
pub enum IDAttributeColor4Command {
    Create(ObjectID, IDAttributeColor4)
}

#[derive(Debug, Default)]
pub struct SingleIDAttributeColor4CommandList {
    pub list: Vec<IDAttributeColor4Command>,
}

pub struct SysIDAttributeColor4Command;
#[setup]
impl SysIDAttributeColor4Command {
    #[system]
    fn sys(
        mut commands: ResMut<SingleIDAttributeColor4CommandList>,
        mut colors: Query<GameObject, Write<IDAttributeColor4>>,
    ) {
        commands.list.drain(..).for_each(|cmd| {
            match cmd {
                IDAttributeColor4Command::Create(entity, value) => {
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

pub struct PluginAttributeColor4;
impl Plugin for PluginAttributeColor4 {
    fn init(
        &mut self,
        engine: &mut crate::engine::Engine,
        stages: &mut crate::run_stage::RunStage,
    ) -> Result<(), crate::plugin::ErrorPlugin> {
        let world = engine.world_mut();

        SysAttributeColor4Command::setup(world, stages.command_stage());
        SysIDAttributeColor4Command::setup(world, stages.command_stage());

        world.insert_resource(SingleAttributeColor4CommandList::default());
        world.insert_resource(SingleIDAttributeColor4CommandList::default());

        Ok(())
    }
}

pub trait InterfaceAttributeColor4 {
    fn create_vertex_data_color4(
        &self,
        data: GeometryBuffer,
    ) -> ObjectID;
}
impl InterfaceAttributeColor4 for Engine {
    fn create_vertex_data_color4(
        &self,
        data: GeometryBuffer,
    ) -> ObjectID {
        let entity = self.new_object();
        let world = self.world();

        let data_size = data.size();
        let gbp = world.get_resource_mut::<SingleGeometryBufferPool>().unwrap();
        let id_indices = gbp.insert(data);

        let data = AttributeColor4 {
            meta: VertexAttributeBufferMeta {
                buffer_id: id_indices,
                start: 0,
                end: data_size * 4,
                data_bytes_size: 4 * 4,
                data_count: data_size / 4,
            }
        };

        let commands = world.get_resource_mut::<SingleAttributeColor4CommandList>().unwrap();
        commands.list.push(AttributeColor4Command::Create(entity, data));

        entity
    }
}
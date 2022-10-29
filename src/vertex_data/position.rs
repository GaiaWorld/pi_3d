use pi_ecs::{prelude::{ResMut, Query, Setup}, query::Write};
use pi_ecs_macros::setup;
use render_data_container::{EVertexDataFormat, GeometryBuffer, GeometryBufferPool};
use render_geometry::geometry::{VertexAttributeBufferMeta, VertexAttributeMeta};

use crate::{object::{ObjectID, GameObject}, geometry::GBID, plugin::Plugin, resources::SingleGeometryBufferPool};


#[derive(Debug, Clone, Copy)]
pub struct IDAttributePosition(pub ObjectID);
#[derive(Debug)]
pub struct AttributePosition {
    pub meta: VertexAttributeBufferMeta<GBID>,
}
impl AttributePosition {
    pub const POSITION: u32 = 3;
    pub const POSITION_OFFSET: u32 = 0 * 4;
    pub const POSITION_FORMAT: wgpu::VertexFormat = wgpu::VertexFormat::Float32x3;
    pub const POSITION_LOCATION: u32 = 0;

    pub const ATTRIBUTES: [wgpu::VertexAttribute;1] = [
        wgpu::VertexAttribute {
            format: Self::POSITION_FORMAT,
            offset: Self::POSITION_OFFSET as wgpu::BufferAddress,
            shader_location: Self::POSITION_LOCATION,
        }
    ];
}
impl VertexAttributeMeta for AttributePosition {
    const SLOT: u32 = 0;

    const SIZE_PER_VERTEX: u32 = Self::POSITION_OFFSET + Self::POSITION * 4;

    const STEP_MODE: wgpu::VertexStepMode = wgpu::VertexStepMode::Vertex;

    const FORMAT: EVertexDataFormat = EVertexDataFormat::F32;
}

#[derive(Debug)]
pub enum AttributePositionCommand {
    Create(ObjectID, AttributePosition)
}

#[derive(Debug, Default)]
pub struct SingleAttributePositionCommandList {
    pub list: Vec<AttributePositionCommand>,
}

pub struct SysAttributePositionCommand;
#[setup]
impl SysAttributePositionCommand {
    #[system]
    fn sys(
        mut commands: ResMut<SingleAttributePositionCommandList>,
        mut colors: Query<GameObject, Write<AttributePosition>>,
    ) {
        commands.list.drain(..).for_each(|cmd| {
            match cmd {
                AttributePositionCommand::Create(entity, value) => {
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
pub enum IDAttributePositionCommand {
    Create(ObjectID, IDAttributePosition)
}

#[derive(Debug, Default)]
pub struct SingleIDAttributePositionCommandList {
    pub list: Vec<IDAttributePositionCommand>,
}

pub struct SysIDAttributePositionCommand;
#[setup]
impl SysIDAttributePositionCommand {
    #[system]
    fn sys(
        mut commands: ResMut<SingleIDAttributePositionCommandList>,
        mut colors: Query<GameObject, Write<IDAttributePosition>>,
    ) {
        commands.list.drain(..).for_each(|cmd| {
            match cmd {
                IDAttributePositionCommand::Create(entity, value) => {
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

pub struct PluginAttributePosition;
impl Plugin for PluginAttributePosition {
    fn init(
        engine: &mut crate::engine::Engine,
        stages: &mut crate::run_stage::RunStage,
    ) -> Result<(), crate::plugin::ErrorPlugin> {
        let world = engine.world_mut();

        SysAttributePositionCommand::setup(world, stages.command_stage());
        SysIDAttributePositionCommand::setup(world, stages.command_stage());

        world.insert_resource(SingleAttributePositionCommandList::default());
        world.insert_resource(SingleIDAttributePositionCommandList::default());

        Ok(())
    }
}

pub trait InterfaceAttributePosition {
    fn create_vertex_data_position(
        &mut self,
        data: GeometryBuffer,
    ) -> ObjectID;
}
impl InterfaceAttributePosition for crate::engine::Engine {
    fn create_vertex_data_position(
        &mut self,
        data: GeometryBuffer,
    ) -> ObjectID {
        let entity = self.new_object();
        let world = self.world_mut();

        let data_size = data.size();
        let gbp = world.get_resource_mut::<SingleGeometryBufferPool>().unwrap();
        let buffer_id = gbp.insert(data);

        let data = AttributePosition {
            meta: VertexAttributeBufferMeta {
                buffer_id,
                start: 0,
                end: data_size * 4,
                data_bytes_size: 3 * 4,
                data_count: data_size / 3,
            },
        };

        let commands = world.get_resource_mut::<SingleAttributePositionCommandList>().unwrap();
        commands.list.push(AttributePositionCommand::Create(entity, data));

        entity
    }
}
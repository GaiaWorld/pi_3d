use pi_ecs::{prelude::{ResMut, Query, Setup}, query::Write};
use pi_ecs_macros::setup;
use render_data_container::{GeometryBuffer, GeometryBufferPool};
use render_geometry::geometry::{VertexAttributeBufferMeta};

use crate::{object::{ObjectID, GameObject}, geometry::GBID, plugin::Plugin, resources::SingleGeometryBufferPool, engine::Engine};

#[derive(Debug, Clone, Copy)]
pub struct IDAttributeIndices(pub ObjectID);

#[derive(Debug)]
pub struct AttributeIndices {
    pub meta: VertexAttributeBufferMeta<GBID>,
    pub format: wgpu::IndexFormat,
}


#[derive(Debug)]
pub enum AttributeIndicesCommand {
    Create(ObjectID, AttributeIndices)
}

#[derive(Debug, Default)]
pub struct SingleAttributeIndicesCommandList {
    pub list: Vec<AttributeIndicesCommand>,
}

pub struct SysAttributeIndicesCommand;
#[setup]
impl SysAttributeIndicesCommand {
    #[system]
    fn sys(
        mut commands: ResMut<SingleAttributeIndicesCommandList>,
        mut colors: Query<GameObject, Write<AttributeIndices>>,
    ) {
        commands.list.drain(..).for_each(|cmd| {
            match cmd {
                AttributeIndicesCommand::Create(entity, value) => {
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
pub enum IDAttributeIndicesCommand {
    Create(ObjectID, IDAttributeIndices)
}

#[derive(Debug, Default)]
pub struct SingleIDAttributeIndicesCommandList {
    pub list: Vec<IDAttributeIndicesCommand>,
}
pub struct SysIDAttributeIndicesCommand;
#[setup]
impl SysIDAttributeIndicesCommand {
    #[system]
    fn sys(
        mut commands: ResMut<SingleIDAttributeIndicesCommandList>,
        mut colors: Query<GameObject, Write<IDAttributeIndices>>,
    ) {
        commands.list.drain(..).for_each(|cmd| {
            match cmd {
                IDAttributeIndicesCommand::Create(entity, value) => {
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

pub struct PluginAttributeIndices;
impl Plugin for PluginAttributeIndices {
    fn init(
        engine: &mut crate::engine::Engine,
        stages: &mut crate::run_stage::RunStage,
    ) -> Result<(), crate::plugin::ErrorPlugin> {
        let world = engine.world_mut();

        SysAttributeIndicesCommand::setup(world, stages.command_stage());
        SysIDAttributeIndicesCommand::setup(world, stages.command_stage());

        world.insert_resource(SingleAttributeIndicesCommandList::default());
        world.insert_resource(SingleIDAttributeIndicesCommandList::default());

        Ok(())
    }
}

pub trait InterfaceAttributeIndices {
    fn create_vertex_data_indices(
        &mut self,
        data: GeometryBuffer,
    ) -> ObjectID;
}
impl InterfaceAttributeIndices for Engine {
    fn create_vertex_data_indices(
        &mut self,
        data: GeometryBuffer,
    ) -> ObjectID {

        let entity = self.new_object();
        let world = self.world_mut();

        let data_size = data.size();
        let gbp = world.get_resource_mut::<SingleGeometryBufferPool>().unwrap();
        let buffer_id = gbp.insert(data);

        let data = AttributeIndices {
            meta: VertexAttributeBufferMeta {
                buffer_id,
                start: 0,
                end: data_size * 2,
                data_bytes_size: 1 * 2,
                data_count: data_size / 1,
            },
            format: wgpu::IndexFormat::Uint16,
        };

        let commands = world.get_resource_mut::<SingleAttributeIndicesCommandList>().unwrap();
        commands.list.push(AttributeIndicesCommand::Create(entity, data));

        entity
    }
}
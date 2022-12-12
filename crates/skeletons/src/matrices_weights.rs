use pi_ecs::{prelude::{ResMut, Query, Setup}, query::Write};
use pi_ecs_macros::setup;
use pi_engine_shell::{object::InterfaceObject, run_stage::RunStage, plugin::ErrorPlugin};
use render_data_container::{EVertexDataFormat, GeometryBuffer, GeometryBufferPool};
use render_geometry::geometry::{VertexAttributeBufferMeta, VertexAttributeMeta};

use pi_scene_context::{object::{ObjectID, GameObject}, geometry::GBID, plugin::Plugin, engine::Engine, resources::SingleGeometryBufferPool};


#[derive(Debug)]
pub struct AttributeMatricesWeights {
    pub meta: VertexAttributeBufferMeta<GBID>,
}
impl AttributeMatricesWeights {
    pub const MATRICES_WEIGHTS: u32 = 4;
    pub const MATRICES_WEIGHTS_OFFSET: u32 = 0 * 4;
    pub const MATRICES_WEIGHTS_FORMAT: wgpu::VertexFormat = wgpu::VertexFormat::Float32x4;
    pub const MATRICES_WEIGHTS_LOCATION: u32 = 3;

    pub const ATTRIBUTES: [wgpu::VertexAttribute; 1] = [
        wgpu::VertexAttribute {
            format: Self::MATRICES_WEIGHTS_FORMAT,
            offset: Self::MATRICES_WEIGHTS_OFFSET as wgpu::BufferAddress,
            shader_location: Self::MATRICES_WEIGHTS_LOCATION,
        }
    ];
}
impl VertexAttributeMeta for AttributeMatricesWeights {
    const SLOT: u32 = 4;

    const SIZE_PER_VERTEX: u32 = Self::MATRICES_WEIGHTS_OFFSET + Self::MATRICES_WEIGHTS * 4;

    const STEP_MODE: wgpu::VertexStepMode = wgpu::VertexStepMode::Vertex;
    const FORMAT: EVertexDataFormat = EVertexDataFormat::F32;
}


#[derive(Debug)]
pub enum AttributeMatricesWeightsCommand {
    Create(ObjectID, AttributeMatricesWeights)
}

#[derive(Debug, Default)]
pub struct SingleAttributeMatricesWeightsCommandList {
    pub list: Vec<AttributeMatricesWeightsCommand>,
}

pub struct SysAttributeMatricesWeightsCommand;
#[setup]
impl SysAttributeMatricesWeightsCommand {
    #[system]
    fn sys(
        mut commands: ResMut<SingleAttributeMatricesWeightsCommandList>,
        mut matrices_weights: Query<GameObject, Write<AttributeMatricesWeights>>,
    ) {
        commands.list.drain(..).for_each(|cmd| {
            match cmd {
                AttributeMatricesWeightsCommand::Create(entity, value) => {
                    match matrices_weights.get_mut(entity) {
                        Some(mut matrices_weights) => {
                            matrices_weights.insert_no_notify(value);
                        },
                        None => {
                            
                        },
                    }
                },
            }
        });
    }
}

#[derive(Debug, Clone, Copy)]
pub struct IDAttributeMatricesWeights(pub ObjectID);

#[derive(Debug)]
pub enum IDAttributeMatricesWeightsCommand {
    Create(ObjectID, IDAttributeMatricesWeights)
}

#[derive(Debug, Default)]
pub struct SingleIDAttributeMatricesWeightsCommandList {
    pub list: Vec<IDAttributeMatricesWeightsCommand>,
}

pub struct SysIDAttributeMatricesWeightsCommand;
#[setup]
impl SysIDAttributeMatricesWeightsCommand {
    #[system]
    fn sys(
        mut commands: ResMut<SingleIDAttributeMatricesWeightsCommandList>,
        mut colors: Query<GameObject, Write<IDAttributeMatricesWeights>>,
    ) {
        commands.list.drain(..).for_each(|cmd| {
            match cmd {
                IDAttributeMatricesWeightsCommand::Create(entity, value) => {
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

pub struct PluginAttributeMatricesWeights;
impl Plugin for PluginAttributeMatricesWeights {
    fn init(
        &mut self,
        engine: &mut Engine,
        stages: &mut RunStage,
    ) -> Result<(), ErrorPlugin> {
        let world = engine.world_mut();

        SysAttributeMatricesWeightsCommand::setup(world, stages.command_stage());
        SysIDAttributeMatricesWeightsCommand::setup(world, stages.command_stage());

        world.insert_resource(SingleAttributeMatricesWeightsCommandList::default());
        world.insert_resource(SingleIDAttributeMatricesWeightsCommandList::default());

        Ok(())
    }
}

pub trait InterfaceAttributeMatricesWeights {
    fn create_vertex_data_matrices_weights(
        &self,
        data: GeometryBuffer,
    ) -> ObjectID;
}
impl InterfaceAttributeMatricesWeights for Engine {
    fn create_vertex_data_matrices_weights(
        &self,
        data: GeometryBuffer,
    ) -> ObjectID {
        let entity = self.new_object();
        let world = self.world();

        let data_size = data.size();
        let gbp = world.get_resource_mut::<SingleGeometryBufferPool>().unwrap();
        let id_indices = gbp.insert(data);

        let data = AttributeMatricesWeights {
            meta: VertexAttributeBufferMeta {
                buffer_id: id_indices,
                start: 0,
                end: data_size * 4,
                data_bytes_size: 4 * 4,
                data_count: data_size / 4,
            }
        };

        let commands = world.get_resource_mut::<SingleAttributeMatricesWeightsCommandList>().unwrap();
        commands.list.push(AttributeMatricesWeightsCommand::Create(entity, data));

        entity
    }
}
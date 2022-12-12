use pi_ecs::{
    prelude::{Query, ResMut, Setup},
    query::Write,
};
use pi_ecs_macros::setup;
use pi_engine_shell::{object::InterfaceObject, run_stage::RunStage, plugin::ErrorPlugin};
use render_data_container::{EVertexDataFormat, GeometryBuffer, GeometryBufferPool};
use render_geometry::geometry::{VertexAttributeBufferMeta, VertexAttributeMeta};

use pi_scene_context::{
    engine::Engine,
    geometry::GBID,
    object::{GameObject, ObjectID},
    plugin::Plugin,
    resources::SingleGeometryBufferPool,
};

#[derive(Debug)]
pub struct AttributeMatricesIndices {
    pub meta: VertexAttributeBufferMeta<GBID>,
}

impl AttributeMatricesIndices {
    pub const INDICES: u32 = 4;
    pub const INDICES_OFFSET: u32 = 0 * 4;
    pub const INDICES_FORMAT: wgpu::VertexFormat = wgpu::VertexFormat::Float32x4;
    pub const INDICES_LOCATION: u32 = 3;

    pub const ATTRIBUTES: [wgpu::VertexAttribute; 1] = [wgpu::VertexAttribute {
        format: Self::INDICES_FORMAT,
        offset: Self::INDICES_OFFSET as wgpu::BufferAddress,
        shader_location: Self::INDICES_LOCATION,
    }];
}

impl VertexAttributeMeta for AttributeMatricesIndices {
    const SLOT: u32 = 3;

    const SIZE_PER_VERTEX: u32 = Self::INDICES_OFFSET + Self::INDICES * 4;

    const STEP_MODE: wgpu::VertexStepMode = wgpu::VertexStepMode::Vertex;
    const FORMAT: EVertexDataFormat = EVertexDataFormat::F32;
}

#[derive(Debug)]
pub enum AttributeMatricesIndicesCommand {
    Create(ObjectID, AttributeMatricesIndices),
}

#[derive(Debug, Default)]
pub struct SingleAttributeMatricesIndicesCommandList {
    pub list: Vec<AttributeMatricesIndicesCommand>,
}

pub struct SysAttributeMatricesIndicesCommand;
#[setup]
impl SysAttributeMatricesIndicesCommand {
    #[system]
    fn sys(
        mut commands: ResMut<SingleAttributeMatricesIndicesCommandList>,
        mut colors: Query<GameObject, Write<AttributeMatricesIndices>>,
    ) {
        commands.list.drain(..).for_each(|cmd| match cmd {
            AttributeMatricesIndicesCommand::Create(entity, value) => {
                match colors.get_mut(entity) {
                    Some(mut color) => {
                        color.insert_no_notify(value);
                    }
                    None => {}
                }
            }
        });
    }
}

#[derive(Debug, Clone, Copy)]
pub struct IDAttributeMatricesIndices(pub ObjectID);

#[derive(Debug)]
pub enum IDAttributeMatricesIndicesCommand {
    Create(ObjectID, IDAttributeMatricesIndices),
}

#[derive(Debug, Default)]
pub struct SingleIDAttributeMatricesIndicesCommandList {
    pub list: Vec<IDAttributeMatricesIndicesCommand>,
}

pub struct SysIDAttributeMatricesIndicesCommand;
#[setup]
impl SysIDAttributeMatricesIndicesCommand {
    #[system]
    fn sys(
        mut commands: ResMut<SingleIDAttributeMatricesIndicesCommandList>,
        mut colors: Query<GameObject, Write<IDAttributeMatricesIndices>>,
    ) {
        commands.list.drain(..).for_each(|cmd| match cmd {
            IDAttributeMatricesIndicesCommand::Create(entity, value) => {
                match colors.get_mut(entity) {
                    Some(mut color) => {
                        color.insert_no_notify(value);
                    }
                    None => {}
                }
            }
        });
    }
}

pub struct PluginAttributeMatricesIndices;
impl Plugin for PluginAttributeMatricesIndices {
    fn init(
        &mut self,
        engine: &mut Engine,
        stages: &mut RunStage,
    ) -> Result<(), ErrorPlugin> {
        let world = engine.world_mut();

        SysAttributeMatricesIndicesCommand::setup(world, stages.command_stage());
        SysIDAttributeMatricesIndicesCommand::setup(world, stages.command_stage());

        world.insert_resource(SingleAttributeMatricesIndicesCommandList::default());
        world.insert_resource(SingleIDAttributeMatricesIndicesCommandList::default());

        Ok(())
    }
}

pub trait InterfaceAttributeMatricesIndices {
    fn create_vertex_data_matrices_indices(&self, data: GeometryBuffer) -> ObjectID;
}

impl InterfaceAttributeMatricesIndices for Engine {
    fn create_vertex_data_matrices_indices(&self, data: GeometryBuffer) -> ObjectID {
        let entity = self.new_object();
        let world = self.world();

        let data_size = data.size();
        let gbp = world
            .get_resource_mut::<SingleGeometryBufferPool>()
            .unwrap();
        let id_indices = gbp.insert(data);

        let data = AttributeMatricesIndices {
            meta: VertexAttributeBufferMeta {
                buffer_id: id_indices,
                start: 0,
                end: data_size * 4,
                data_bytes_size: 4 * 4,
                data_count: data_size / 4,
            },
        };

        let commands = world
            .get_resource_mut::<SingleAttributeMatricesIndicesCommandList>()
            .unwrap();
        commands
            .list
            .push(AttributeMatricesIndicesCommand::Create(entity, data));

        entity
    }
}

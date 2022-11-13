use pi_ecs::{world::World, query::{QueryState, Write}, prelude::{StageBuilder, ArchetypeId, ResMut, Query, Setup}};
use pi_ecs_macros::setup;
use pi_idtree::Node;
use pi_render::rhi::{device::RenderDevice, RenderQueue};
use render_data_container::{GeometryBuffer, EVertexDataFormat, GeometryBufferPool};
use render_geometry::geometry::VertexAttributeBufferMeta;

use crate::{resources::{SingleGeometryBufferPool}, geometry::GBID, plugin::{Plugin, ErrorPlugin}, object::{ObjectID, GameObject}, engine::Engine, vertex_data::{position::{IDAttributePosition, AttributePosition, SingleAttributePositionCommandList, AttributePositionCommand, SingleIDAttributePositionCommandList, IDAttributePositionCommand}, normal::{IDAttributeNormal, AttributeNormal, SingleAttributeNormalCommandList, AttributeNormalCommand, IDAttributeNormalCommand, SingleIDAttributeNormalCommandList}, indices::{IDAttributeIndices, AttributeIndices, SingleAttributeIndicesCommandList, AttributeIndicesCommand, SingleIDAttributeIndicesCommandList, IDAttributeIndicesCommand}}, scene::{ command::{SingleSceneCommandList, SceneCommand}, interface::InterfaceScene}, flags::SceneID, transforms::{command::{SingleTransformNodeCommandList, TransformNodeCommand, SingleTreeCommandList, TreeCommand}, interface::InterfaceTransformNode}, default_render::interface::InterfaceDefaultMaterial};

use super::interface::InterfaceMesh;

pub struct SingleBaseCube {
    position: IDAttributePosition,
    normal: IDAttributeNormal,
    indices: IDAttributeIndices,
}
impl SingleBaseCube {
    pub fn position(&self) -> IDAttributePosition {
        self.position
    }
    pub fn normal(&self) -> IDAttributeNormal {
        self.normal
    }
    pub fn indices(&self) -> IDAttributeIndices {
        self.indices
    }
}

pub struct CubeBuilder;
impl CubeBuilder {
    pub fn position(
        device: &RenderDevice,
        queue: &RenderQueue,
        gbp: &mut SingleGeometryBufferPool,
    ) -> AttributePosition {
        let mut position = GeometryBuffer::new(true, EVertexDataFormat::F32, false);
        let mut data = [
            1., -1., 1., -1., -1., 1., -1., 1., 1., 1., 1., 1., 
            1., 1., -1., -1., 1., -1., -1., -1., -1.,  1., -1., -1., 
            1., 1., -1., 1., -1., -1., 1., -1., 1., 1., 1., 1.,
            -1., 1., 1., -1., -1., 1., -1., -1., -1., -1., 1., -1.,
            -1., 1., 1., -1., 1., -1., 1., 1., -1., 1., 1., 1., 
            1., -1., 1., 1., -1., -1., -1., -1., -1., -1., -1., 1.
        ];

        position.update_f32(&data, 0);
        position.update_buffer(device, queue);
        let id_position = gbp.insert(position);

        AttributePosition {
            meta: VertexAttributeBufferMeta {
                buffer_id: id_position,
                start: 0,
                end: 72 * 4,
                data_bytes_size: 3 * 4,
                data_count: 24,
            },
        }
    }
    pub fn normal(
        device: &RenderDevice,
        queue: &RenderQueue,
        gbp: &mut SingleGeometryBufferPool,
    ) -> AttributeNormal {

        let data = [
            0., 0., 1., 0., 0., 1., 0., 0., 1., 0., 0., 1., 
            0., 0., -1., 0., 0., -1., 0., 0., -1., 0., 0., -1., 
            1., 0., 0., 1., 0., 0., 1., 0., 0., 1., 0., 0., 
            -1., 0., 0., -1., 0., 0., -1., 0., 0., -1., 0., 0., 
            0., 1., 0., 0., 1., 0., 0., 1., 0., 0., 1., 0., 
            0., -1., 0., 0., -1., 0., 0., -1., 0., 0., -1., 0.
        ];
        let mut normals = GeometryBuffer::new(true, EVertexDataFormat::F32, false);
        normals.update_f32(&data, 0);
        normals.update_buffer(device, queue);
        let id_normal = gbp.insert(normals);

        AttributeNormal {
            meta: VertexAttributeBufferMeta {
                buffer_id: id_normal,
                start: 0,
                end: 72 * 4,
                data_bytes_size: 3 * 4,
                data_count: 24,
            },
        }
    }
    pub fn indices(
        device: &RenderDevice,
        queue: &RenderQueue,
        gbp: &mut SingleGeometryBufferPool,
    ) -> AttributeIndices {
        let data = [
            0, 1, 2, 0, 2, 3,
            4, 5, 6, 4, 6, 7,
            8, 9, 10, 8, 10, 11,
            12, 13, 14, 12, 14, 15,
            16, 17, 18, 16, 18, 19,
            20, 21, 22, 20, 22, 23
        ];
        let mut indices = GeometryBuffer::new(true, EVertexDataFormat::U16, true);
        indices.update_u16(&data, 0);
        indices.update_buffer(device, queue);
        let id_indices = gbp.insert(indices);

        AttributeIndices {
            meta: VertexAttributeBufferMeta {
                buffer_id: id_indices,
                start: 0,
                end: 36 * 2,
                data_bytes_size: 1 * 2,
                data_count: 36,
            },
            format: wgpu::IndexFormat::Uint16,
        }
    }
}

pub enum CubeBuilderCommand {
    Base(ObjectID, IDAttributePosition, IDAttributeNormal, IDAttributeIndices)
}

pub struct SingleCubeBuilderCommandList {
    pub list: Vec<CubeBuilderCommand>,
}
pub trait InterfaceCube {
    fn new_cube(
        & self,
        scene: ObjectID,
    ) -> ObjectID;
}

impl InterfaceCube for Engine {
    fn new_cube(
        & self,
        scene: ObjectID,
    ) -> ObjectID {

        let entity = self.new_object();
        let world = self.add_to_scene(entity, scene)
                                    .as_transform_node(entity)
                                    .transform_parent(entity, scene)
                                    .as_mesh(entity)
                                    .use_default_material(entity)
                                    .world();
        
        let base_cube = world.get_resource_mut::<SingleBaseCube>().unwrap();
        let commands = world.get_resource_mut::<SingleIDAttributePositionCommandList>().unwrap();
        commands.list.push(IDAttributePositionCommand::Create(entity, base_cube.position()));
        let commands = world.get_resource_mut::<SingleIDAttributeNormalCommandList>().unwrap();
        commands.list.push(IDAttributeNormalCommand::Create(entity, base_cube.normal()));
        let commands = world.get_resource_mut::<SingleIDAttributeIndicesCommandList>().unwrap();
        commands.list.push(IDAttributeIndicesCommand::Create(entity, base_cube.indices()));

        entity
    }
}

pub struct PluginCubeBuilder;
impl Plugin for PluginCubeBuilder {
    fn init(
        engine: &mut Engine,
        stages: &mut crate::run_stage::RunStage,
    ) -> Result<(), ErrorPlugin> {
        let position_id = engine.new_object();
        let normal_id = engine.new_object();
        let indices_id = engine.new_object();
        let world = engine.world_mut();

        let device = world.get_resource::<RenderDevice>().unwrap();
        let queue = world.get_resource::<RenderQueue>().unwrap();
        let gbp = world.get_resource_mut::<SingleGeometryBufferPool>().unwrap();
    

        let position = CubeBuilder::position(device, queue, gbp);
        let normal = CubeBuilder::normal(device, queue, gbp);
        let indices = CubeBuilder::indices(device, queue, gbp);

        let commands = world.get_resource_mut::<SingleAttributePositionCommandList>().unwrap();
        commands.list.push(AttributePositionCommand::Create(position_id, position));
        let commands = world.get_resource_mut::<SingleAttributeNormalCommandList>().unwrap();
        commands.list.push(AttributeNormalCommand::Create(normal_id, normal));
        let commands = world.get_resource_mut::<SingleAttributeIndicesCommandList>().unwrap();
        commands.list.push(AttributeIndicesCommand::Create(indices_id, indices));

        world.insert_resource::<SingleBaseCube>(
            SingleBaseCube { position: IDAttributePosition(position_id), normal: IDAttributeNormal(normal_id), indices: IDAttributeIndices(indices_id) }
        );

        Ok(())
    }
}
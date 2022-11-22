
use pi_engine_shell::object::InterfaceObject;
use pi_render::rhi::{device::RenderDevice, RenderQueue};
use render_data_container::{GeometryBuffer, EVertexDataFormat, GeometryBufferPool};
use render_geometry::geometry::VertexAttributeBufferMeta;

use crate::{resources::{SingleGeometryBufferPool}, plugin::{Plugin, ErrorPlugin}, object::{ObjectID}, engine::Engine, vertex_data::{position::{IDAttributePosition, AttributePosition, SingleAttributePositionCommandList, AttributePositionCommand, SingleIDAttributePositionCommandList, IDAttributePositionCommand}, normal::{IDAttributeNormal, AttributeNormal, SingleAttributeNormalCommandList, AttributeNormalCommand, IDAttributeNormalCommand, SingleIDAttributeNormalCommandList}, indices::{IDAttributeIndices, AttributeIndices, SingleAttributeIndicesCommandList, AttributeIndicesCommand, SingleIDAttributeIndicesCommandList, IDAttributeIndicesCommand}, uv::{SingleAttributeUVCommandList, IDAttributeUVCommand, IDAttributeUV, AttributeUV, AttributeUVCommand, SingleIDAttributeUVCommandList}}, scene::{ interface::InterfaceScene}, transforms::interface::InterfaceTransformNode};

use super::interface::InterfaceMesh;

pub struct SingleBaseQuad {
    position: IDAttributePosition,
    normal: IDAttributeNormal,
    indices: IDAttributeIndices,
    uvs: IDAttributeUV,
}
impl SingleBaseQuad {
    pub fn position(&self) -> IDAttributePosition {
        self.position
    }
    pub fn normal(&self) -> IDAttributeNormal {
        self.normal
    }
    pub fn indices(&self) -> IDAttributeIndices {
        self.indices
    }
    pub fn uvs(&self) -> IDAttributeUV {
        self.uvs
    }
}

pub struct QuadBuilder;
impl QuadBuilder {
    pub fn position(
        device: &RenderDevice,
        queue: &RenderQueue,
        gbp: &mut SingleGeometryBufferPool,
    ) -> AttributePosition {
        let mut position = GeometryBuffer::new(true, EVertexDataFormat::F32, false);
        let mut data = [
            -1., -1., 0.,   1., -1., 0.,   1., 1., 0.,  -1., 1., 0.,  
        ];

        position.update_f32(&data, 0);
        position.update_buffer(device, queue);
        let id_position = gbp.insert(position);

        AttributePosition {
            meta: VertexAttributeBufferMeta {
                buffer_id: id_position,
                start: 0,
                end: 12 * 4,
                data_bytes_size: 3 * 4,
                data_count: 4,
            },
        }
    }
    pub fn normal(
        device: &RenderDevice,
        queue: &RenderQueue,
        gbp: &mut SingleGeometryBufferPool,
    ) -> AttributeNormal {

        let data = [
            0., 0., 1.,     0., 0., 1.,     0., 0., 1.,     0., 0., 1.,
        ];
        let mut normals = GeometryBuffer::new(true, EVertexDataFormat::F32, false);
        normals.update_f32(&data, 0);
        normals.update_buffer(device, queue);
        let id_normal = gbp.insert(normals);

        AttributeNormal {
            meta: VertexAttributeBufferMeta {
                buffer_id: id_normal,
                start: 0,
                end: 12 * 4,
                data_bytes_size: 3 * 4,
                data_count: 4,
            },
        }
    }
    pub fn indices(
        device: &RenderDevice,
        queue: &RenderQueue,
        gbp: &mut SingleGeometryBufferPool,
    ) -> AttributeIndices {
        let data = [
            0, 1, 2, 0, 2, 3
        ];
        let mut indices = GeometryBuffer::new(true, EVertexDataFormat::U16, true);
        indices.update_u16(&data, 0);
        indices.update_buffer(device, queue);
        let id_indices = gbp.insert(indices);

        AttributeIndices {
            meta: VertexAttributeBufferMeta {
                buffer_id: id_indices,
                start: 0,
                end: 6 * 2,
                data_bytes_size: 1 * 2,
                data_count: 6,
            },
            format: wgpu::IndexFormat::Uint16,
        }
    }
    pub fn uvs(
        device: &RenderDevice,
        queue: &RenderQueue,
        gbp: &mut SingleGeometryBufferPool,
    ) -> AttributeUV {
        let data = [
            0., 0.,   1., 0.,   1., 1.,     0., 1.,
        ];
        let mut indices = GeometryBuffer::new(true, EVertexDataFormat::F32, false);
        indices.update_f32(&data, 0);
        indices.update_buffer(device, queue);
        let id_indices = gbp.insert(indices);

        AttributeUV {
            meta: VertexAttributeBufferMeta {
                buffer_id: id_indices,
                start: 0,
                end: 8 * 4,
                data_bytes_size: 2 * 4,
                data_count: 4,
            }
        }
    }
}

pub enum QuadBuilderCommand {
    Base(ObjectID, IDAttributePosition, IDAttributeNormal, IDAttributeIndices)
}

pub struct SingleQuadBuilderCommandList {
    pub list: Vec<QuadBuilderCommand>,
}
pub trait InterfaceQuad {
    fn new_quad(
        & self,
        scene: ObjectID,
    ) -> ObjectID;
}

impl InterfaceQuad for Engine {
    fn new_quad(
        & self,
        scene: ObjectID,
    ) -> ObjectID {

        let entity = self.new_object();
        let world = self.add_to_scene(entity, scene)
                                    .as_transform_node(entity)
                                    .transform_parent(entity, scene)
                                    .as_mesh(entity)
                                    .world();
        
        let base_quad = world.get_resource_mut::<SingleBaseQuad>().unwrap();
        let commands = world.get_resource_mut::<SingleIDAttributePositionCommandList>().unwrap();
        commands.list.push(IDAttributePositionCommand::Create(entity, base_quad.position()));
        let commands = world.get_resource_mut::<SingleIDAttributeNormalCommandList>().unwrap();
        commands.list.push(IDAttributeNormalCommand::Create(entity, base_quad.normal()));
        let commands = world.get_resource_mut::<SingleIDAttributeIndicesCommandList>().unwrap();
        commands.list.push(IDAttributeIndicesCommand::Create(entity, base_quad.indices()));
        let commands = world.get_resource_mut::<SingleIDAttributeUVCommandList>().unwrap();
        commands.list.push(IDAttributeUVCommand::Create(entity, base_quad.uvs()));

        entity
    }
}

pub struct PluginQuadBuilder;
impl Plugin for PluginQuadBuilder {
    fn init(
        &mut self,
        engine: &mut Engine,
        stages: &mut crate::run_stage::RunStage,
    ) -> Result<(), ErrorPlugin> {

        let position_id = engine.new_object();
        let normal_id = engine.new_object();
        let indices_id = engine.new_object();
        let uvs_id = engine.new_object();

        let world = engine.world_mut();

        let device = world.get_resource::<RenderDevice>().unwrap();
        let queue = world.get_resource::<RenderQueue>().unwrap();
        let gbp = world.get_resource_mut::<SingleGeometryBufferPool>().unwrap();
    

        let position = QuadBuilder::position(device, queue, gbp);
        let normal = QuadBuilder::normal(device, queue, gbp);
        let indices = QuadBuilder::indices(device, queue, gbp);
        let uvs = QuadBuilder::uvs(device, queue, gbp);

        let commands = world.get_resource_mut::<SingleAttributePositionCommandList>().unwrap();
        commands.list.push(AttributePositionCommand::Create(position_id, position));
        let commands = world.get_resource_mut::<SingleAttributeNormalCommandList>().unwrap();
        commands.list.push(AttributeNormalCommand::Create(normal_id, normal));
        let commands = world.get_resource_mut::<SingleAttributeIndicesCommandList>().unwrap();
        commands.list.push(AttributeIndicesCommand::Create(indices_id, indices));
        let commands = world.get_resource_mut::<SingleAttributeUVCommandList>().unwrap();
        commands.list.push(AttributeUVCommand::Create(uvs_id, uvs));

        world.insert_resource::<SingleBaseQuad>(
            SingleBaseQuad { position: IDAttributePosition(position_id), normal: IDAttributeNormal(normal_id), indices: IDAttributeIndices(indices_id), uvs: IDAttributeUV(uvs_id) }
        );

        Ok(())
    }
}
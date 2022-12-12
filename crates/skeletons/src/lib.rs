pub mod bind_group;
pub mod command;
pub mod gltf;
pub mod interface;
pub mod material;
pub mod material_sys;
pub mod matrices_indices;
pub mod matrices_weights;
pub mod pipeline;
pub mod shader;

use matrices_indices::{
    AttributeMatricesIndices, AttributeMatricesIndicesCommand, IDAttributeMatricesIndices,
    IDAttributeMatricesIndicesCommand, SingleAttributeMatricesIndicesCommandList,
    SingleIDAttributeMatricesIndicesCommandList,
};
use matrices_weights::{
    AttributeMatricesWeights, AttributeMatricesWeightsCommand, IDAttributeMatricesWeights,
    IDAttributeMatricesWeightsCommand, SingleAttributeMatricesWeightsCommandList,
    SingleIDAttributeMatricesWeightsCommandList,
};
use pi_3d_loader::factory::GltfLoader;
use pi_engine_shell::object::InterfaceObject;
use pi_hal::runtime::MULTI_MEDIA_RUNTIME;
use pi_render::rhi::{device::RenderDevice, RenderQueue};
use render_data_container::{EVertexDataFormat, GeometryBuffer, GeometryBufferPool};
use render_geometry::geometry::VertexAttributeBufferMeta;

use pi_scene_context::{
    engine::Engine,
    meshes::interface::InterfaceMesh,
    object::ObjectID,
    plugin::{ErrorPlugin, Plugin},
    resources::SingleGeometryBufferPool,
    run_stage::RunStage,
    scene::interface::InterfaceScene,
    transforms::interface::InterfaceTransformNode,
    vertex_data::{
        indices::{
            AttributeIndices, AttributeIndicesCommand, IDAttributeIndices,
            IDAttributeIndicesCommand, SingleAttributeIndicesCommandList,
            SingleIDAttributeIndicesCommandList,
        },
        normal::{
            AttributeNormal, AttributeNormalCommand, IDAttributeNormal, IDAttributeNormalCommand,
            SingleAttributeNormalCommandList, SingleIDAttributeNormalCommandList,
        },
        position::{
            AttributePosition, AttributePositionCommand, IDAttributePosition,
            IDAttributePositionCommand, SingleAttributePositionCommandList,
            SingleIDAttributePositionCommandList,
        },
    },
};

pub struct Skeletons {
    position: IDAttributePosition,
    normal: IDAttributeNormal,
    indices: IDAttributeIndices,
    matrices_weights: IDAttributeMatricesWeights,
    matrices_indices: IDAttributeMatricesIndices,
}
impl Skeletons {
    pub fn position(&self) -> IDAttributePosition {
        self.position
    }

    pub fn normal(&self) -> IDAttributeNormal {
        self.normal
    }

    pub fn indices(&self) -> IDAttributeIndices {
        self.indices
    }

    pub fn matrices_weights(&self) -> IDAttributeMatricesWeights {
        self.matrices_weights
    }

    pub fn matrices_indices(&self) -> IDAttributeMatricesIndices {
        self.matrices_indices
    }
}

pub struct SkeletonsBuilder;
impl SkeletonsBuilder {
    pub fn position(
        device: &RenderDevice,
        queue: &RenderQueue,
        gbp: &mut SingleGeometryBufferPool,
    ) -> AttributePosition {
        let mut position = GeometryBuffer::new(true, EVertexDataFormat::F32, false);
        let data = [
            -0.5, 0.0, 0.0, 0.5, 0.0, 0.0, -0.5, 0.5, 0.0, 0.5, 0.5, 0.0, -0.5, 1.0, 0.0, 0.5, 1.0,
            0.0, -0.5, 1.5, 0.0, 0.5, 1.5, 0.0, -0.5, 2.0, 0.0, 0.5, 2.0, 0.0,
        ];
        position.update_f32(&data, 0);
        position.update_buffer(device, queue);
        let id_position = gbp.insert(position);

        AttributePosition {
            meta: VertexAttributeBufferMeta {
                buffer_id: id_position,
                start: 0,
                end: 30 * 4,
                data_bytes_size: 3 * 4,
                data_count: 10,
            },
        }
    }

    pub fn normal(
        device: &RenderDevice,
        queue: &RenderQueue,
        gbp: &mut SingleGeometryBufferPool,
    ) -> AttributeNormal {
        let data = [
            -0.7071, 0., 0.7071, -0.5773, -0.5773, 0.5773, -0.5773, -0.5773, -0.5773, 0.7071, 0.,
            0.7071, 0.5773, -0.5773, 0.5773, 0.7071, 0.0, -0.7071, 0.5773, -0.5773, -0.5773,
            -0.7071, 0., -0.7071, -0.7071, 0., -0.7071, -0.7071, 0., 0.7071,
        ];
        // let mut indices = GeometryBuffer::new(false, EVertexDataFormat::U16, true);
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
                data_count: 10,
            },
        }
    }

    pub fn indices(
        device: &RenderDevice,
        queue: &RenderQueue,
        gbp: &mut SingleGeometryBufferPool,
    ) -> AttributeIndices {
        let data = [
            0, 1, 3, 0, 3, 2, 2, 3, 5, 2, 5, 4, 4, 5, 7, 4, 7, 6, 6, 7, 9, 6, 9, 8,
        ];
        let mut indices = GeometryBuffer::new(true, EVertexDataFormat::U16, true);
        indices.update_u16(&data, 0);
        indices.update_buffer(device, queue);
        let id_indices = gbp.insert(indices);

        AttributeIndices {
            meta: VertexAttributeBufferMeta {
                buffer_id: id_indices,
                start: 0,
                end: 24 * 2,
                data_bytes_size: 1 * 2,
                data_count: 24,
            },
            format: wgpu::IndexFormat::Uint16,
        }
    }

    pub fn matrices_weights(
        device: &RenderDevice,
        queue: &RenderQueue,
        gbp: &mut SingleGeometryBufferPool,
    ) -> AttributeMatricesWeights {
        let data = [
            0.8258, 0.1712, 0.0, 0.0, 0.9877, 0.0, 0.0, 0.0, 0.9868, 0.0, 0.0, 0.0, 0.8257, 0.1712,
            0.0, 0.0, 0.9868, 0.0, 0.0, 0.0, 0.8258, 0.1712, 0.0, 0.0, 0.9877, 0.0, 0.0, 0.0,
            0.8257, 0.1712, 0.0, 0.0, 0.2042, 0.7818, 0.0, 0.0, 0.2042, 0.7818, 0.0, 0.0, 0.2042,
            0.7818, 0.0, 0.0, 0.8013, 0.1831, 0.0, 0.0, 0.8013, 0.1831, 0.0, 0.0, 0.2042, 0.7818,
            0.0, 0.0, 0.1482, 0.849, 0.0, 0.0, 0.1482, 0.8489, 0.0, 0.0, 0.8013, 0.1831, 0.0, 0.0,
            0.8013, 0.1831, 0.0, 0.0, 0.1482, 0.8489, 0.0, 0.0, 0.9894, 0.0, 0.0, 0.0, 0.9903, 0.0,
            0.0, 0.0, 0.1482, 0.849, 0.0, 0.0, 0.9903, 0.0, 0.0, 0.0, 0.9894, 0.0, 0.0, 0.0,
        ];
        let mut indices = GeometryBuffer::new(true, EVertexDataFormat::F32, true);
        indices.update_f32(&data, 0);
        indices.update_buffer(device, queue);
        let id_indices = gbp.insert(indices);

        AttributeMatricesWeights {
            meta: VertexAttributeBufferMeta {
                buffer_id: id_indices,
                start: 0,
                end: 96 * 4,
                data_bytes_size: 4 * 4,
                data_count: 24,
            },
        }
    }

    pub fn matrices_indices(
        device: &RenderDevice,
        queue: &RenderQueue,
        gbp: &mut SingleGeometryBufferPool,
    ) -> AttributeMatricesIndices {
        let data = [
            0.8258, 0.1712, 0., 0., 0.9877, 0., 0., 0., 0.9868, 0., 0., 0., 0.8257, 0.1712, 0., 0.,
            0.9868, 0., 0., 0., 0.8258, 0.1712, 0., 0., 0.9877, 0., 0., 0., 0.8257, 0.1712, 0., 0.,
            0.2042, 0.7818, 0., 0., 0.2042, 0.7818, 0., 0., 0.2042, 0.7818, 0., 0., 0.8013, 0.1831,
            0., 0., 0.8013, 0.1831, 0., 0., 0.2042, 0.7818, 0., 0., 0.1482, 0.849, 0., 0., 0.1482,
            0.8489, 0., 0., 0.8013, 0.1831, 0., 0., 0.8013, 0.1831, 0., 0., 0.1482, 0.8489, 0., 0.,
            0.9894, 0., 0., 0., 0.9903, 0., 0., 0., 0.1482, 0.849, 0., 0., 0.9903, 0., 0., 0.,
            0.9894, 0., 0., 0.,
        ];
        let mut indices = GeometryBuffer::new(true, EVertexDataFormat::F32, true);
        indices.update_f32(&data, 0);
        indices.update_buffer(device, queue);
        let id_indices = gbp.insert(indices);

        AttributeMatricesIndices {
            meta: VertexAttributeBufferMeta {
                buffer_id: id_indices,
                start: 0,
                end: 96 * 4,
                data_bytes_size: 4 * 4,
                data_count: 24,
            },
        }
    }
}

pub trait InterfaceSkeletons {
    fn new_skeletons(&self, scene: ObjectID) -> ObjectID;
}

impl InterfaceSkeletons for Engine {
    fn new_skeletons(&self, scene: ObjectID) -> ObjectID {
        let entity = self.new_object();
        let world = self
            .add_to_scene(entity, scene)
            .as_transform_node(entity)
            .transform_parent(entity, scene)
            .as_mesh(entity)
            .world();

        // let gltf = GltfLoader::from_gltf("");

        let skeletons = world.get_resource_mut::<Skeletons>().unwrap();

        let commands = world
            .get_resource_mut::<SingleIDAttributePositionCommandList>()
            .unwrap();
        commands.list.push(IDAttributePositionCommand::Create(
            entity,
            skeletons.position(),
        ));

        let commands = world
            .get_resource_mut::<SingleIDAttributeNormalCommandList>()
            .unwrap();
        commands
            .list
            .push(IDAttributeNormalCommand::Create(entity, skeletons.normal()));

        let commands = world
            .get_resource_mut::<SingleIDAttributeIndicesCommandList>()
            .unwrap();
        commands.list.push(IDAttributeIndicesCommand::Create(
            entity,
            skeletons.indices(),
        ));

        let commands = world
            .get_resource_mut::<SingleIDAttributeMatricesIndicesCommandList>()
            .unwrap();
        commands
            .list
            .push(IDAttributeMatricesIndicesCommand::Create(
                entity,
                skeletons.matrices_indices(),
            ));

        let commands = world
            .get_resource_mut::<SingleIDAttributeMatricesWeightsCommandList>()
            .unwrap();
        commands
            .list
            .push(IDAttributeMatricesWeightsCommand::Create(
                entity,
                skeletons.matrices_weights(),
            ));

        entity
    }
}

pub struct PluginSkeletons;
impl Plugin for PluginSkeletons {
    fn init(&mut self, engine: &mut Engine, _: &mut RunStage) -> Result<(), ErrorPlugin> {
        let position_id = engine.new_object();
        let normal_id = engine.new_object();
        let indices_id = engine.new_object();
        let matrices_weights_id = engine.new_object();
        let matrices_indices_id = engine.new_object();

        let world = engine.world_mut();

        let device = world.get_resource::<RenderDevice>().unwrap();
        let queue = world.get_resource::<RenderQueue>().unwrap();
        let gbp = world
            .get_resource_mut::<SingleGeometryBufferPool>()
            .unwrap();

        let position = SkeletonsBuilder::position(device, queue, gbp);
        let normal = SkeletonsBuilder::normal(device, queue, gbp);
        let indices = SkeletonsBuilder::indices(device, queue, gbp);
        let matrices_indices = SkeletonsBuilder::matrices_indices(device, queue, gbp);
        let matrices_weights = SkeletonsBuilder::matrices_weights(device, queue, gbp);

        let commands = world
            .get_resource_mut::<SingleAttributePositionCommandList>()
            .unwrap();
        commands
            .list
            .push(AttributePositionCommand::Create(position_id, position));

        let commands = world
            .get_resource_mut::<SingleAttributeNormalCommandList>()
            .unwrap();
        commands
            .list
            .push(AttributeNormalCommand::Create(normal_id, normal));

        let commands = world
            .get_resource_mut::<SingleAttributeIndicesCommandList>()
            .unwrap();
        commands
            .list
            .push(AttributeIndicesCommand::Create(indices_id, indices));

        let commands = world
            .get_resource_mut::<SingleAttributeMatricesIndicesCommandList>()
            .unwrap();
        commands.list.push(AttributeMatricesIndicesCommand::Create(
            matrices_indices_id,
            matrices_indices,
        ));

        let commands = world
            .get_resource_mut::<SingleAttributeMatricesWeightsCommandList>()
            .unwrap();
        commands.list.push(AttributeMatricesWeightsCommand::Create(
            matrices_indices_id,
            matrices_weights,
        ));

        world.insert_resource::<Skeletons>(Skeletons {
            position: IDAttributePosition(position_id),
            normal: IDAttributeNormal(normal_id),
            indices: IDAttributeIndices(indices_id),
            matrices_indices: IDAttributeMatricesIndices(matrices_indices_id),
            matrices_weights: IDAttributeMatricesWeights(matrices_weights_id),
        });

        Ok(())
    }
}

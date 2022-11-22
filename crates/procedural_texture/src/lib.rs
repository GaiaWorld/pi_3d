pub mod cloud;
pub mod perlin_noise;

use cloud::interface::InterfaceCloudMaterial;
use perlin_noise::interface::InterfacePerlinNoiseMaterial;
use pi_engine_shell::object::InterfaceObject;
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

pub struct Skybox {
    position: IDAttributePosition,
    normal: IDAttributeNormal,
    indices: IDAttributeIndices,
}
impl Skybox {
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

pub struct SkyboxBuilder;
impl SkyboxBuilder {
    pub fn position(
        device: &RenderDevice,
        queue: &RenderQueue,
        gbp: &mut SingleGeometryBufferPool,
    ) -> AttributePosition {
        let mut position = GeometryBuffer::new(true, EVertexDataFormat::F32, false);

        let data = [
            1., -1., 1., -1., -1., 1., -1., 1., 1., 1., 1., 1., 1., 1., -1., -1., 1., -1., -1.,
            -1., -1., 1., -1., -1., 1., 1., -1., 1., -1., -1., 1., -1., 1., 1., 1., 1., -1., 1.,
            1., -1., -1., 1., -1., -1., -1., -1., 1., -1., -1., 1., 1., -1., 1., -1., 1., 1., -1.,
            1., 1., 1., 1., -1., 1., 1., -1., -1., -1., -1., -1., -1., -1., 1.,
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
            0., 0., 1., 0., 0., 1., 0., 0., 1., 0., 0., 1., 0., 0., -1., 0., 0., -1., 0., 0., -1.,
            0., 0., -1., 1., 0., 0., 1., 0., 0., 1., 0., 0., 1., 0., 0., -1., 0., 0., -1., 0., 0.,
            -1., 0., 0., -1., 0., 0., 0., 1., 0., 0., 1., 0., 0., 1., 0., 0., 1., 0., 0., -1., 0.,
            0., -1., 0., 0., -1., 0., 0., -1., 0.,
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
            0, 1, 2, 0, 2, 3, 4, 5, 6, 4, 6, 7, 8, 9, 10, 8, 10, 11, 12, 13, 14, 12, 14, 15, 16,
            17, 18, 16, 18, 19, 20, 21, 22, 20, 22, 23,
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

pub trait InterfaceTestPerlinNoise {
    fn new_skybox(&self, scene: ObjectID) -> ObjectID;
}

impl InterfaceTestPerlinNoise for Engine {
    fn new_skybox(&self, scene: ObjectID) -> ObjectID {
        let entity = self.new_object();
        let world = self
            .add_to_scene(entity, scene)
            .as_transform_node(entity)
            .transform_parent(entity, scene)
            .as_mesh(entity)
            .world();

        let sky_box = world.get_resource_mut::<Skybox>().unwrap();

        let commands = world
            .get_resource_mut::<SingleIDAttributePositionCommandList>()
            .unwrap();
        commands.list.push(IDAttributePositionCommand::Create(
            entity,
            sky_box.position(),
        ));

        let commands = world
            .get_resource_mut::<SingleIDAttributeNormalCommandList>()
            .unwrap();
        commands
            .list
            .push(IDAttributeNormalCommand::Create(entity, sky_box.normal()));

        let commands = world
            .get_resource_mut::<SingleIDAttributeIndicesCommandList>()
            .unwrap();
        commands
            .list
            .push(IDAttributeIndicesCommand::Create(entity, sky_box.indices()));

        entity
    }
}

pub struct PluginTestPerlinNoise;
impl Plugin for PluginTestPerlinNoise {
    fn init(&mut self, engine: &mut Engine, _: &mut RunStage) -> Result<(), ErrorPlugin> {
        let position_id = engine.new_object();
        let normal_id = engine.new_object();
        let indices_id = engine.new_object();

        let world = engine.world_mut();

        let device = world.get_resource::<RenderDevice>().unwrap();
        let queue = world.get_resource::<RenderQueue>().unwrap();
        let gbp = world
            .get_resource_mut::<SingleGeometryBufferPool>()
            .unwrap();

        let position = SkyboxBuilder::position(device, queue, gbp);
        let normal = SkyboxBuilder::normal(device, queue, gbp);
        let indices = SkyboxBuilder::indices(device, queue, gbp);

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

        world.insert_resource::<Skybox>(Skybox {
            position: IDAttributePosition(position_id),
            normal: IDAttributeNormal(normal_id),
            indices: IDAttributeIndices(indices_id),
        });

        Ok(())
    }
}

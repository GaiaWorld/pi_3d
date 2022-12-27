use pi_engine_shell::{
    engine_shell::EnginShell,
    object::{InterfaceObject, ObjectID},
    plugin::{ErrorPlugin, Plugin},
    run_stage::RunStage,
};
use pi_render::rhi::{device::RenderDevice, RenderQueue};
use pi_scene_context::{
    geometry::{indices::InterfaceBufferIndices, TInterfaceGeomtery},
    meshes::interface::InterfaceMesh,
    scene::interface::InterfaceScene,
    transforms::interface::InterfaceTransformNode,
};
use render_data_container::{EVertexDataFormat, KeyVertexBuffer, VertexBuffer};
use render_geometry::{vertex_data::{VertexAttribute, EVertexDataKind, VertexBufferDesc}, indices::IndicesBufferDesc};

pub struct SkinBuilder;
impl SkinBuilder {
    const KEY_BUFFER_POSITION: &'static str = "SkinPosition";
    const KEY_BUFFER_JOINT: &'static str = "SkinJoint";
    const KEY_BUFFER_WEIGHT: &'static str = "SkinWeight";
    const KEY_BUFFER_INDICES: &'static str = "SkinIndices";

    pub fn position(device: &RenderDevice, queue: &RenderQueue) -> VertexBuffer {
        let mut position = VertexBuffer::new(false, EVertexDataFormat::F32, false);
        let mut data = [
            -0.125, 0.0, 0.0, 0.125, 0.0, 0.0, -0.125, 0.25, 0.0, 0.125, 0.25, 0.0, -0.125, 0.5,
            0.0, 0.125, 0.5, 0.0, -0.125, 0.75, 0.0, 0.125, 0.75, 0.0, -0.125, 1.0, 0.0, 0.125,
            1.0, 0.0,
        ];

        position.update_f32(&data, 0);
        position.update_buffer(device, queue);
        position
    }

    pub fn indices(device: &RenderDevice, queue: &RenderQueue) -> VertexBuffer {
        let data = [
            0, 1, 3, 0, 3, 2, 2, 3, 5, 2, 5, 4, 4, 5, 7, 4, 7, 6, 6, 7, 9, 6, 9, 8,
        ];
        let mut indices = VertexBuffer::new(false, EVertexDataFormat::U16, true);
        indices.update_u16(&data, 0);
        indices.update_buffer(device, queue);
        indices
    }
    pub fn joints(device: &RenderDevice, queue: &RenderQueue) -> VertexBuffer {
        let data = [
            0., 0., 0., 0., 0., 0., 0., 0., 0., 1., 0., 0., 0., 1., 0., 0., 0., 1., 0., 0., 0., 1.,
            0., 0., 0., 1., 0., 0., 0., 1., 0., 0., 0., 1., 0., 0., 0., 1., 0., 0.,
        ];
        let mut indices = VertexBuffer::new(false, EVertexDataFormat::F32, false);
        indices.update_f32(&data, 0);
        indices.update_buffer(device, queue);
        indices
    }

    pub fn weights(device: &RenderDevice, queue: &RenderQueue) -> VertexBuffer {
        let data = [
            1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.75, 0.25, 0.0, 0.0, 0.75, 0.25, 0.0, 0.0,
            0.5, 0.5, 0.0, 0.0, 0.5, 0.5, 0.0, 0.0, 0.25, 0.75, 0.0, 0.0, 0.25, 0.75, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0,
        ];
        let mut indices = VertexBuffer::new(false, EVertexDataFormat::F32, false);
        indices.update_f32(&data, 0);
        indices.update_buffer(device, queue);
        indices
    }
}

pub trait InterfaceSkin {
    fn regist_skin(&self) -> &Self;
    fn new_skin(&self, scene: ObjectID) -> ObjectID;
}

impl InterfaceSkin for EnginShell {
    fn regist_skin(&self) -> &Self {
        let world = self.world();
        let device = world.get_resource::<RenderDevice>().unwrap();
        let queue = world.get_resource::<RenderQueue>().unwrap();

        let keypos = KeyVertexBuffer::from(SkinBuilder::KEY_BUFFER_POSITION);
        self.create_vertex_buffer(keypos.clone(), SkinBuilder::position(device, queue));

        let keyjoint = KeyVertexBuffer::from(SkinBuilder::KEY_BUFFER_JOINT);
        self.create_vertex_buffer(keyjoint.clone(), SkinBuilder::joints(device, queue));

        let keyweight = KeyVertexBuffer::from(SkinBuilder::KEY_BUFFER_WEIGHT);
        self.create_vertex_buffer(keyweight.clone(), SkinBuilder::weights(device, queue));

        let key = KeyVertexBuffer::from(SkinBuilder::KEY_BUFFER_INDICES);
        self.create_vertex_buffer(key.clone(), SkinBuilder::indices(device, queue));

        self
    }

    fn new_skin(&self, scene: ObjectID) -> ObjectID {
        let entity = self.new_object();
        self.add_to_scene(entity, scene)
            .as_transform_node(entity)
            .transform_parent(entity, scene)
            .as_mesh(entity);

        let keypos = KeyVertexBuffer::from(SkinBuilder::KEY_BUFFER_POSITION);
        let keyjoint = KeyVertexBuffer::from(SkinBuilder::KEY_BUFFER_JOINT);
        let keyweight = KeyVertexBuffer::from(SkinBuilder::KEY_BUFFER_WEIGHT);
        let key = KeyVertexBuffer::from(SkinBuilder::KEY_BUFFER_INDICES);

        self.use_geometry(
            entity,
            vec![
                VertexBufferDesc::vertices(
                    keypos,
                    None,
                    vec![VertexAttribute {
                        kind: EVertexDataKind::Position,
                        format: wgpu::VertexFormat::Float32x3,
                    }]
                ),
                VertexBufferDesc::vertices(
                    keyjoint,
                    None,
                    vec![VertexAttribute {
                        kind: EVertexDataKind::MatricesIndices,
                        format: wgpu::VertexFormat::Float32x4,
                    }]
                ),
                VertexBufferDesc::vertices(
                    keyweight,
                    None,
                    vec![VertexAttribute {
                        kind: EVertexDataKind::MatricesWeights,
                        format: wgpu::VertexFormat::Float32x4,
                    }]
                ),
            ],
        );
        self.use_indices(
            entity,
            IndicesBufferDesc {
                format: wgpu::IndexFormat::Uint16,
                buffer_range: None,
                buffer: key,
            },
        );

        entity
    }
}

pub struct PluginSkinBuilder;
impl Plugin for PluginSkinBuilder {
    fn init(&mut self, engine: &mut EnginShell, stages: &mut RunStage) -> Result<(), ErrorPlugin> {
        engine.regist_skin();

        Ok(())
    }
}

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
use render_geometry::{
    indices::IndicesBufferDesc,
    vertex_data::{EVertexDataKind, VertexAttribute, VertexBufferDesc},
};

pub struct AxisBuilder;
impl AxisBuilder {
    const KEY_BUFFER_POSITION: &'static str = "AxisPosition";
    const KEY_BUFFER_COLOR4: &'static str = "AxisColor";
    const KEY_BUFFER_INDICES: &'static str = "AxisIndices";

    pub fn position(device: &RenderDevice, queue: &RenderQueue) -> VertexBuffer {
        let mut position = VertexBuffer::new(false, EVertexDataFormat::F32, false);
        let mut x_axis = vec![
            -0.03, 0.03, 0.03, -0.03, -0.03, 0.03, -0.03, 0., -0.03,
             0.9,  0.03, 0.03,  0.9,  -0.03, 0.03,  0.9,  0., -0.03,
             0.9,  0.06, 0.06,  0.9,  -0.06, 0.06,  0.9,  0., -0.06, 1., 0., 0.,
        ];

        let mut y_axis = vec![
            -0.03, -0.03, 0.03, 0.03, -0.03, 0.03, 0., -0.03, -0.03, 
            -0.03,  0.9,  0.03, 0.03,  0.9,  0.03, 0.,  0.9,  -0.03,
            -0.06,  0.9,  0.06, 0.06,  0.9,  0.06, 0.0, 0.9,  -0.06, 0., 1., 0.,
        ];

        let mut z_axis = vec![
            -0.03, -0.03, -0.03, 0.03, -0.03, -0.03, 0., 0.03, -0.03, 
            -0.03, -0.03,  0.9,  0.03, -0.03,  0.9,  0., 0.03,  0.9,
            -0.06, -0.06,  0.9,  0.06, -0.06,  0.9,  0., 0.06,  0.9, 0., 0., 1.,
        ];

        let mut data = vec![];
        data.append(&mut x_axis);
        data.append(&mut y_axis);
        data.append(&mut z_axis);

        position.update_f32(&data, 0);
        position.update_buffer(device, queue);
        position
    }

    pub fn indices(device: &RenderDevice, queue: &RenderQueue) -> VertexBuffer {
        let mut x_axis = vec![
            0, 1, 2,
            0, 1, 4, 0, 4, 3,
            1, 2, 5, 1, 5, 4, 
            2, 0, 3, 2, 3, 5, 
            6, 7, 9,
            7, 8, 9,
            8, 6, 9,
            6, 7, 8
        ];
        let mut y_axis = vec![
            10, 11, 12,
            10, 11, 14, 10, 14, 13,
            11, 12, 15, 11, 15, 14, 
            12, 10, 13, 12, 13, 15, 
            16, 17, 19,
            17, 18, 19,
            18, 16, 19,
            16, 17, 18
        ];
        let mut z_axis = vec![
            20, 21, 22,
            20, 21, 24, 20, 24, 23,
            21, 22, 25, 21, 25, 24, 
            22, 20, 23, 22, 23, 25, 
            26, 27, 29,
            27, 28, 29,
            28, 26, 29,
            26, 27, 28
        ];

        let mut data = vec![];
        data.append(&mut x_axis);
        data.append(&mut y_axis);
        data.append(&mut z_axis);
        
        let mut indices = VertexBuffer::new(false, EVertexDataFormat::U16, true);
        indices.update_u16(&data, 0);
        indices.update_buffer(device, queue);
        indices
    }
    pub fn colors(device: &RenderDevice, queue: &RenderQueue) -> VertexBuffer {
        let data = [
            1., 0., 0., 1., 1., 0., 0., 1., 1., 0., 0., 1., 1., 0., 0., 1.,1., 0., 0., 1., 1., 0., 0., 1.,1., 0., 0., 1., 1., 0., 0., 1.,1., 0., 0., 1., 1., 0., 0., 1.,
            0., 1., 0., 1., 0., 1., 0., 1., 0., 1., 0., 1., 0., 1., 0., 1.,0., 1., 0., 1., 0., 1., 0., 1.,0., 1., 0., 1., 0., 1., 0., 1.,0., 1., 0., 1., 0., 1., 0., 1.,
            0., 0., 1., 1., 0., 0., 1., 1., 0., 0., 1., 1., 0., 0., 1., 1.,0., 0., 1., 1., 0., 0., 1., 1.,0., 0., 1., 1., 0., 0., 1., 1.,0., 0., 1., 1., 0., 0., 1., 1.,
        ];
        let mut indices = VertexBuffer::new(false, EVertexDataFormat::F32, false);
        indices.update_f32(&data, 0);
        indices.update_buffer(device, queue);
        indices
    }
}

pub trait InterfaceAxis {
    fn regist_axis(&self) -> &Self;
    fn new_axis(&self, scene: ObjectID) -> ObjectID;
}

impl InterfaceAxis for EnginShell {
    fn regist_axis(&self) -> &Self {
        let world = self.world();
        let device = world.get_resource::<RenderDevice>().unwrap();
        let queue = world.get_resource::<RenderQueue>().unwrap();

        let keypos = KeyVertexBuffer::from(AxisBuilder::KEY_BUFFER_POSITION);
        self.create_vertex_buffer(keypos.clone(), AxisBuilder::position(device, queue));

        let keycolor = KeyVertexBuffer::from(AxisBuilder::KEY_BUFFER_COLOR4);
        self.create_vertex_buffer(keycolor.clone(), AxisBuilder::colors(device, queue));

        let key = KeyVertexBuffer::from(AxisBuilder::KEY_BUFFER_INDICES);
        self.create_vertex_buffer(key.clone(), AxisBuilder::indices(device, queue));

        self
    }

    fn new_axis(&self, scene: ObjectID) -> ObjectID {
        let entity = self.new_object();
        self.add_to_scene(entity, scene)
            .as_transform_node(entity)
            .transform_parent(entity, scene)
            .as_mesh(entity);

        let keypos = KeyVertexBuffer::from(AxisBuilder::KEY_BUFFER_POSITION);
        let keycolor = KeyVertexBuffer::from(AxisBuilder::KEY_BUFFER_COLOR4);
        let key = KeyVertexBuffer::from(AxisBuilder::KEY_BUFFER_INDICES);

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
                    keycolor,
                    None,
                    vec![VertexAttribute {
                        kind: EVertexDataKind::Color4,
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

pub struct PluginAxisBuilder;
impl Plugin for PluginAxisBuilder {
    fn init(&mut self, engine: &mut EnginShell, stages: &mut RunStage) -> Result<(), ErrorPlugin> {
        engine.regist_axis();

        Ok(())
    }
}

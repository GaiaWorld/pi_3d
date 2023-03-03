use pi_engine_shell::{
    engine_shell::EnginShell,
    object::{InterfaceObject, ObjectID},
    plugin::{ErrorPlugin, Plugin},
    run_stage::RunStage,
};
use pi_render::{rhi::{device::RenderDevice, RenderQueue}, renderer::{vertex_buffer::KeyVertexBuffer, vertex_buffer_desc::VertexBufferDesc, attributes::{VertexAttribute, EVertexDataKind}, indices::IndicesBufferDesc}};
use pi_scene_context::{
    geometry::{indices::InterfaceBufferIndices, TInterfaceGeomtery},
    meshes::interface::InterfaceMesh,
    scene::interface::InterfaceScene,
    transforms::interface::InterfaceTransformNode,
};

pub struct AxisBuilder;
impl AxisBuilder {
    const KEY_BUFFER_POSITION: &'static str = "AxisPosition";
    const KEY_BUFFER_COLOR4: &'static str = "AxisColor";
    const KEY_BUFFER_INDICES: &'static str = "AxisIndices";

    pub fn position() -> Vec<f32> {
        let mut x_axis: Vec<f32> = vec![
            -0.03, 0.03, 0.03, -0.03, -0.03, 0.03, -0.03, 0., -0.03,
             0.9,  0.03, 0.03,  0.9,  -0.03, 0.03,  0.9,  0., -0.03,
             0.9,  0.06, 0.06,  0.9,  -0.06, 0.06,  0.9,  0., -0.06, 1., 0., 0.,
        ];

        let mut y_axis: Vec<f32> = vec![
            -0.03, -0.03, 0.03, 0.03, -0.03, 0.03, 0., -0.03, -0.03, 
            -0.03,  0.9,  0.03, 0.03,  0.9,  0.03, 0.,  0.9,  -0.03,
            -0.06,  0.9,  0.06, 0.06,  0.9,  0.06, 0.0, 0.9,  -0.06, 0., 1., 0.,
        ];

        let mut z_axis: Vec<f32> = vec![
            -0.03, -0.03, -0.03, 0.03, -0.03, -0.03, 0., 0.03, -0.03, 
            -0.03, -0.03,  0.9,  0.03, -0.03,  0.9,  0., 0.03,  0.9,
            -0.06, -0.06,  0.9,  0.06, -0.06,  0.9,  0., 0.06,  0.9, 0., 0., 1.,
        ];

        let mut data = vec![];
        data.append(&mut x_axis);
        data.append(&mut y_axis);
        data.append(&mut z_axis);

        data
    }

    pub fn indices() -> Vec<u16> {
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
        
        data
    }
    pub fn colors() -> [f32; 120] {
        let data: [f32; 120] = [
            1., 0., 0., 1., 1., 0., 0., 1., 1., 0., 0., 1., 1., 0., 0., 1.,1., 0., 0., 1., 1., 0., 0., 1.,1., 0., 0., 1., 1., 0., 0., 1.,1., 0., 0., 1., 1., 0., 0., 1.,
            0., 1., 0., 1., 0., 1., 0., 1., 0., 1., 0., 1., 0., 1., 0., 1.,0., 1., 0., 1., 0., 1., 0., 1.,0., 1., 0., 1., 0., 1., 0., 1.,0., 1., 0., 1., 0., 1., 0., 1.,
            0., 0., 1., 1., 0., 0., 1., 1., 0., 0., 1., 1., 0., 0., 1., 1.,0., 0., 1., 1., 0., 0., 1., 1.,0., 0., 1., 1., 0., 0., 1., 1.,0., 0., 1., 1., 0., 0., 1., 1.,
        ];
        data
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
        self.create_vertex_buffer(keypos.clone(), bytemuck::cast_slice(&AxisBuilder::position()).iter().map(|v| *v).collect::<Vec<u8>>());

        let keycolor = KeyVertexBuffer::from(AxisBuilder::KEY_BUFFER_COLOR4);
        self.create_vertex_buffer(keycolor.clone(), bytemuck::cast_slice(&AxisBuilder::colors()).iter().map(|v| *v).collect::<Vec<u8>>());

        let key = KeyVertexBuffer::from(AxisBuilder::KEY_BUFFER_INDICES);
        self.create_vertex_buffer(key.clone(), bytemuck::cast_slice(&AxisBuilder::indices()).iter().map(|v| *v).collect::<Vec<u8>>());

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

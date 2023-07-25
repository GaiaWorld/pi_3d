use pi_engine_shell::prelude::*;
use pi_scene_context::prelude::*;

pub struct AxisBuilder;
impl AxisBuilder {
    pub(crate) const KEY_BUFFER_POSITION:   &'static str = "AxisPOSITION";
    pub(crate) const KEY_BUFFER_COLOR4:     &'static str = "AxisCOLOR";
    pub(crate) const KEY_BUFFER_INDICES:    &'static str = "AxisINDICES";

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

    pub fn attrs_meta() -> Vec<VertexBufferDesc> {
        vec![
            VertexBufferDesc::vertices(
                KeyVertexBuffer::from(AxisBuilder::KEY_BUFFER_POSITION),
                None,
                vec![VertexAttribute {
                    kind: EVertexDataKind::Position,
                    format: wgpu::VertexFormat::Float32x3,
                }]
            ),
            VertexBufferDesc::vertices(
                KeyVertexBuffer::from(AxisBuilder::KEY_BUFFER_COLOR4),
                None,
                vec![VertexAttribute {
                    kind: EVertexDataKind::Color4,
                    format: wgpu::VertexFormat::Float32x4,
                }]
            ),
        ]
    }
    pub fn indices_meta() -> Option<IndicesBufferDesc> {
        Some(IndicesBufferDesc {
            format: wgpu::IndexFormat::Uint16,
            buffer_range: None,
            buffer: KeyVertexBuffer::from(AxisBuilder::KEY_BUFFER_INDICES),
        })
    }
}

pub struct InterfaceAxis;
impl InterfaceAxis {
    pub fn new_axis(
        app: &mut App,
        scene: ObjectID
    ) -> ObjectID {
    
        let id_mesh = ActionMesh::create(app, scene, String::from("Axis"));
        ActionMesh::use_geometry(app, id_mesh, AxisBuilder::attrs_meta(), AxisBuilder::indices_meta());

        id_mesh
    }
}

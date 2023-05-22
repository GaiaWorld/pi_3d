

use std::ops::Range;

use pi_engine_shell::prelude::*;

use pi_scene_context::prelude::*;

pub struct CubeBuilder;
impl CubeBuilder {
    const KEY_BUFFER_COLOR4:    &'static str = "CubeColor4";
    const KEY_BUFFER_POSITION:  &'static str = "CubePosition";
    const KEY_BUFFER_NORMAL:    &'static str = "CubeNormal";
    const KEY_BUFFER_UV:        &'static str = "CubeUV";
    const KEY_BUFFER_INDICES:   &'static str = "CubeIndices";
    const KEY_BUFFER:           &'static str = "CubeBuildin";
    const VERTEX_COUNT:         usize = 24;
    const POSITION_OFFSET:      usize = 0;
    const POSITION_SIZE:        usize = 72 * 4;
    const NORMAL_OFFSET:        usize = Self::POSITION_OFFSET + Self::POSITION_SIZE;
    const NORMAL_SIZE:          usize = 72* 4 ;
    const UV_OFFSET:            usize = Self::NORMAL_OFFSET + Self::NORMAL_SIZE;
    const UV_SIZE:              usize = 48 * 4;
    const INDICES_OFFSET:       usize = Self::UV_OFFSET + Self::UV_SIZE;
    const INDICES_SIZE:         usize = 36 * 2;
    const TOTAL_SIZE:           usize = Self::INDICES_OFFSET + Self::INDICES_SIZE;
    pub fn attrs_meta() -> Vec<VertexBufferDesc> {
        let key = KeyVertexBuffer::from(CubeBuilder::KEY_BUFFER);
        vec![
            VertexBufferDesc::vertices(
                key,
                Some(Range { start: Self::POSITION_OFFSET as u64, end: Self::POSITION_SIZE as u64, }),
                vec![
                    VertexAttribute { kind: EVertexDataKind::Position, format: wgpu::VertexFormat::Float32x3 },
                    VertexAttribute { kind: EVertexDataKind::Normal, format: wgpu::VertexFormat::Float32x3 },
                    VertexAttribute { kind: EVertexDataKind::UV, format: wgpu::VertexFormat::Float32x2 }
                ]
            ),
        ]
    }
    pub fn indices_meta() -> IndicesBufferDesc {
        let key = KeyVertexBuffer::from(CubeBuilder::KEY_BUFFER_INDICES);
        IndicesBufferDesc { format: wgpu::IndexFormat::Uint16, buffer_range: None, buffer: key }
    }
    pub fn position() -> [f32; 72] {
        [
            // z = 1
            1., -1., 1., -1., -1., 1., -1., 1., 1., 1., 1., 1., 
            // z = -1
            1., 1., -1., -1., 1., -1., -1., -1., -1.,  1., -1., -1., 
            // x = 1
            1., 1., -1., 1., -1., -1., 1., -1., 1., 1., 1., 1.,
            // x = -1
            -1., 1., 1., -1., -1., 1., -1., -1., -1., -1., 1., -1.,
            // y = 1
            -1., 1., 1., -1., 1., -1., 1., 1., -1., 1., 1., 1., 
            // y = -1
            1., -1., 1., 1., -1., -1., -1., -1., -1., -1., -1., 1.
        ]
    }
    pub fn normal() -> [f32; 72] {
        [
            0., 0., 1., 0., 0., 1., 0., 0., 1., 0., 0., 1., 
            0., 0., -1., 0., 0., -1., 0., 0., -1., 0., 0., -1., 
            1., 0., 0., 1., 0., 0., 1., 0., 0., 1., 0., 0., 
            -1., 0., 0., -1., 0., 0., -1., 0., 0., -1., 0., 0., 
            0., 1., 0., 0., 1., 0., 0., 1., 0., 0., 1., 0., 
            0., -1., 0., 0., -1., 0., 0., -1., 0., 0., -1., 0.
        ]
    }
    pub fn uvs() -> [f32; 48] {
        [
            1., 0.,     0., 0.,     0., 1.,     1., 1., 
            1., 1.,     0., 1.,     0., 0.,     1., 0., 
            1., 0.,     0., 0.,     0., 1.,     1., 1.,
            1., 1.,     0., 1.,     0., 0.,     1., 0.,
            0., 1.,     0., 0.,     1., 0.,     1., 1., 
            1., 1.,     1., 0.,     0., 0.,     0., 1.
        ]
    }
    pub fn vertices() -> [f32; 192] {
        [
            // z = 1
             1., -1.,  1.,       0.,  0.,  1.,      1., 0.,
            -1., -1.,  1.,       0.,  0.,  1.,      0., 0.,
            -1.,  1.,  1.,       0.,  0.,  1.,      0., 1.,
             1.,  1.,  1.,       0.,  0.,  1.,      1., 1., 
            // z = -1
             1.,  1., -1.,       0.,  0., -1.,      1., 1.,
            -1.,  1., -1.,       0.,  0., -1.,      0., 1.,
            -1., -1., -1.,       0.,  0., -1.,      0., 0.,
             1., -1., -1.,       0.,  0., -1.,      1., 0., 
            // x = 1
             1.,  1., -1.,       1.,  0.,  0.,      1., 0.,
             1., -1., -1.,       1.,  0.,  0.,      0., 0.,
             1., -1.,  1.,       1.,  0.,  0.,      0., 1.,
             1.,  1.,  1.,       1.,  0.,  0.,      1., 1.,
            // x = -1
            -1.,  1.,  1.,      -1.,  0.,  0.,      1., 1.,
            -1., -1.,  1.,      -1.,  0.,  0.,      0., 1.,
            -1., -1., -1.,      -1.,  0.,  0.,      0., 0.,
            -1.,  1., -1.,      -1.,  0.,  0.,      1., 0.,
            // y = 1
            -1.,  1.,  1.,       0.,  1.,  0.,      0., 1.,
            -1.,  1., -1.,       0.,  1.,  0.,      0., 0.,
             1.,  1., -1.,       0.,  1.,  0.,      1., 0.,
             1.,  1.,  1.,       0.,  1.,  0.,      1., 1.,
             // y = -1
             1., -1.,  1.,       0., -1.,  0.,      1., 1.,
             1., -1., -1.,       0., -1.,  0.,      1., 0.,
            -1., -1., -1.,       0., -1.,  0.,      0., 0.,
            -1., -1.,  1.,       0., -1.,  0.,      0., 1.
        ]
    }
    pub fn indices() -> [u16; 36] {
        [
            0, 1, 2, 0, 2, 3,
            4, 5, 6, 4, 6, 7,
            8, 9, 10, 8, 10, 11,
            12, 13, 14, 12, 14, 15,
            16, 17, 18, 16, 18, 19,
            20, 21, 22, 20, 22, 23
        ]
    }
}

pub struct ActionCube;
impl ActionCube {
    pub fn new_cube(
        app: &mut App,
        scene: ObjectID,
        name: String,
    ) -> ObjectID {
        let id_mesh = ActionMesh::create(app, scene, name);
        ActionMesh::use_geometry(app, id_mesh, CubeBuilder::attrs_meta(), Some(CubeBuilder::indices_meta()));

        id_mesh
    }
}

// impl InterfaceCube for Engine {
    fn setup(
        asset_mgr: Res<ShareAssetMgr<EVertexBufferRange>>,
        mut data_map: ResMut<VertexBufferDataMap3D>,
    ) {
        if !ActionVertexBuffer::check(&asset_mgr, KeyVertexBuffer::from(CubeBuilder::KEY_BUFFER)) {
            ActionVertexBuffer::create(&mut data_map, KeyVertexBuffer::from(CubeBuilder::KEY_BUFFER), bytemuck::cast_slice(&CubeBuilder::vertices()).iter().map(|v| *v).collect::<Vec<u8>>());
        }
        if !ActionVertexBuffer::check(&asset_mgr, KeyVertexBuffer::from(CubeBuilder::KEY_BUFFER_INDICES)) {
            // log::warn!("CubeBuilder::KEY_BUFFER_INDICES");
            ActionVertexBuffer::create_indices(&mut data_map, KeyVertexBuffer::from(CubeBuilder::KEY_BUFFER_INDICES), bytemuck::cast_slice(&CubeBuilder::indices()).iter().map(|v| *v).collect::<Vec<u8>>());
        }
    }
    // fn new_cube(
    //     & self,
    //     scene: ObjectID,
    // ) -> ObjectID {

    //     let entity = self.new_object();
    //     self.add_to_scene(entity, scene)
    //                                 .as_transform_node(entity)
    //                                 .transform_parent(entity, scene)
    //                                 .as_mesh(entity);

    //     self.use_geometry(
    //         entity,
    //         CubeBuilder::attrs_meta(),
    //         Some(CubeBuilder::indices_meta())
    //     );

    //     entity
    // }
// }

pub struct PluginCubeBuilder;
impl Plugin for PluginCubeBuilder {
    // fn init(
    //     &mut self,
    //     engine: &mut Engine,
    //     stages: &mut pi_engine_shell::run_stage::RunStage,
    // ) -> Result<(), ErrorPlugin> {
    //     engine.regist_cube();

    //     Ok(())
    // }

    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}


use pi_assets::mgr::AssetMgr;
use pi_engine_shell::{object::InterfaceObject, assets::sync_load::{InterfaceAssetSyncCreate, AssetSyncWait}};
use pi_render::rhi::{device::RenderDevice, RenderQueue};
use pi_share::Share;
use render_data_container::{VertexBuffer, EVertexDataFormat, KeyVertexBuffer};
use render_geometry::{indices::{AssetKeyBufferIndices, AssetResBufferIndices, IndicesBufferDesc}, vertex_data::{VertexBufferDesc, VertexAttribute, EVertexDataKind}};

use crate::{
    plugin::{Plugin, ErrorPlugin},
    object::{ObjectID},
    engine::Engine, 
    scene::{ interface::InterfaceScene},
    transforms::interface::InterfaceTransformNode, geometry::{TInterfaceGeomtery, indices::InterfaceBufferIndices}
};

use super::interface::InterfaceMesh;

pub struct CubeBuilder;
impl CubeBuilder {
    const KEY_BUFFER_COLOR4:    &'static str = "CubeColor4";
    const KEY_BUFFER_POSITION:  &'static str = "CubePosition";
    const KEY_BUFFER_NORMAL:    &'static str = "CubeNormal";
    const KEY_BUFFER_UV:        &'static str = "CubeUV";
    const KEY_BUFFER_INDICES:   &'static str = "CubeIndices";
    pub fn attrs_desc() -> Vec<VertexBufferDesc> {
        let keypos = KeyVertexBuffer::from(CubeBuilder::KEY_BUFFER_POSITION);
        let keynormal = KeyVertexBuffer::from(CubeBuilder::KEY_BUFFER_NORMAL);
        let keyuv = KeyVertexBuffer::from(CubeBuilder::KEY_BUFFER_UV);
        vec![
            VertexBufferDesc::vertices(keypos, None, vec![VertexAttribute { kind: EVertexDataKind::Position, format: wgpu::VertexFormat::Float32x3 }]),
            VertexBufferDesc::vertices(keynormal, None, vec![VertexAttribute { kind: EVertexDataKind::Normal, format: wgpu::VertexFormat::Float32x3 }]),
            VertexBufferDesc::vertices(keyuv, None, vec![VertexAttribute { kind: EVertexDataKind::UV, format: wgpu::VertexFormat::Float32x2 }]),
        ]
    }
    pub fn indices_desc() -> IndicesBufferDesc {
        let key = KeyVertexBuffer::from(CubeBuilder::KEY_BUFFER_INDICES);
        IndicesBufferDesc { format: wgpu::IndexFormat::Uint16, buffer_range: None, buffer: key }
    }
    pub fn position(
        device: &RenderDevice,
        queue: &RenderQueue,
    ) -> VertexBuffer {
        let mut position = VertexBuffer::new(false, EVertexDataFormat::F32, false);
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
        position
    }
    pub fn normal(
        device: &RenderDevice,
        queue: &RenderQueue,
    ) -> VertexBuffer {

        let data = [
            0., 0., 1., 0., 0., 1., 0., 0., 1., 0., 0., 1., 
            0., 0., -1., 0., 0., -1., 0., 0., -1., 0., 0., -1., 
            1., 0., 0., 1., 0., 0., 1., 0., 0., 1., 0., 0., 
            -1., 0., 0., -1., 0., 0., -1., 0., 0., -1., 0., 0., 
            0., 1., 0., 0., 1., 0., 0., 1., 0., 0., 1., 0., 
            0., -1., 0., 0., -1., 0., 0., -1., 0., 0., -1., 0.
        ];
        let mut normals = VertexBuffer::new(false, EVertexDataFormat::F32, false);
        normals.update_f32(&data, 0);
        normals.update_buffer(device, queue);
        normals
    }
    pub fn indices(
        device: &RenderDevice,
        queue: &RenderQueue,
    ) -> VertexBuffer {
        let data = [
            0, 1, 2, 0, 2, 3,
            4, 5, 6, 4, 6, 7,
            8, 9, 10, 8, 10, 11,
            12, 13, 14, 12, 14, 15,
            16, 17, 18, 16, 18, 19,
            20, 21, 22, 20, 22, 23
        ];
        let mut indices = VertexBuffer::new(false, EVertexDataFormat::U16, true);
        indices.update_u16(&data, 0);
        indices.update_buffer(device, queue);
        indices
    }
    pub fn uvs(
        device: &RenderDevice,
        queue: &RenderQueue,
    ) -> VertexBuffer {
        let data = [
            1., 0.,     0., 0.,     0., 1.,     1., 1., 
            1., 1.,     0., 1.,     0., 0.,     1., 0., 
            1., 0.,     0., 0.,     0., 1.,     1., 1.,
            1., 1.,     0., 1.,     0., 0.,     1., 0.,
            0., 1.,     0., 0.,     1., 0.,     1., 1., 
            1., 1.,     1., 0.,     0., 0.,     0., 1.
        ];
        let mut indices = VertexBuffer::new(false, EVertexDataFormat::F32, false);
        indices.update_f32(&data, 0);
        indices.update_buffer(device, queue);
        indices
    }
}

pub trait InterfaceCube {
    fn regist_cube(
        &self
    ) -> &Self;
    fn new_cube(
        & self,
        scene: ObjectID,
    ) -> ObjectID;
}

impl InterfaceCube for Engine {
    fn regist_cube(
        &self
    ) -> &Self {
        let world = self.world();
        let device = world.get_resource::<RenderDevice>().unwrap();
        let queue = world.get_resource::<RenderQueue>().unwrap();

        let keypos = KeyVertexBuffer::from(CubeBuilder::KEY_BUFFER_POSITION);
        self.create_vertex_buffer(keypos.clone(), CubeBuilder::position(device, queue));

        let keynormal = KeyVertexBuffer::from(CubeBuilder::KEY_BUFFER_NORMAL);
        self.create_vertex_buffer(keynormal.clone(), CubeBuilder::normal(device, queue));
        
        let keyuv = KeyVertexBuffer::from(CubeBuilder::KEY_BUFFER_UV);
        self.create_vertex_buffer(keyuv.clone(), CubeBuilder::uvs(device, queue));

        let key = KeyVertexBuffer::from(CubeBuilder::KEY_BUFFER_INDICES);
        self.create_vertex_buffer(key.clone(), CubeBuilder::indices(device, queue));

        self
    }
    fn new_cube(
        & self,
        scene: ObjectID,
    ) -> ObjectID {

        let entity = self.new_object();
        self.add_to_scene(entity, scene)
                                    .as_transform_node(entity)
                                    .transform_parent(entity, scene)
                                    .as_mesh(entity);

        self.use_geometry(
            entity,
            CubeBuilder::attrs_desc()
        );
        self.use_indices(entity, CubeBuilder::indices_desc());

        entity
    }
}

pub struct PluginCubeBuilder;
impl Plugin for PluginCubeBuilder {
    fn init(
        &mut self,
        engine: &mut Engine,
        stages: &mut crate::run_stage::RunStage,
    ) -> Result<(), ErrorPlugin> {
        engine.regist_cube();

        Ok(())
    }
}
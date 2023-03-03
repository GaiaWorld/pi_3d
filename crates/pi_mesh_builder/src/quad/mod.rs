

use pi_assets::{mgr::AssetMgr, asset::Handle};
use pi_engine_shell::{object::InterfaceObject, assets::sync_load::{InterfaceAssetSyncCreate, AssetSyncWait}};
use pi_render::{rhi::{device::RenderDevice, RenderQueue}, renderer::{vertex_buffer_desc::VertexBufferDesc, vertex_buffer::{KeyVertexBuffer, EVertexBufferRange, VertexBufferAllocator}, attributes::{EVertexDataKind, VertexAttribute}, indices::IndicesBufferDesc}};
use pi_scene_context::{
    plugin::{Plugin, ErrorPlugin},
    object::{ObjectID},
    engine::Engine,
    scene::{ interface::InterfaceScene},
    transforms::interface::InterfaceTransformNode, geometry::{TInterfaceGeomtery, indices::InterfaceBufferIndices}, meshes::interface::InterfaceMesh
};

pub struct QuadBuilder;
impl QuadBuilder {
    const KEY_BUFFER_COLOR4:    &'static str = "QuadColor4";
    const KEY_BUFFER_POSITION:  &'static str = "QuadPosition";
    const KEY_BUFFER_NORMAL:    &'static str = "QuadNormal";
    const KEY_BUFFER_UV:        &'static str = "QuadUV";
    const KEY_BUFFER_INDICES:   &'static str = "QuadIndices";
    const KEY_BUFFER:           &'static str = "CubeBuildin";
    pub fn attrs_meta() -> Vec<VertexBufferDesc> {
        let keypos = KeyVertexBuffer::from(Self::KEY_BUFFER);
        vec![
            VertexBufferDesc::vertices(
                keypos, 
                None, 
                vec![
                    VertexAttribute { kind: EVertexDataKind::Position, format: wgpu::VertexFormat::Float32x3 },
                    VertexAttribute { kind: EVertexDataKind::Normal, format: wgpu::VertexFormat::Float32x3 },
                    VertexAttribute { kind: EVertexDataKind::UV, format: wgpu::VertexFormat::Float32x2 }
                ]
            ),
        ]
    }
    pub fn indices_meta() -> IndicesBufferDesc {
        let key = KeyVertexBuffer::from(Self::KEY_BUFFER_INDICES);
        IndicesBufferDesc { format: wgpu::IndexFormat::Uint16, buffer_range: None, buffer: key }
    }
    pub fn position() -> [f32; 12] {
        [
            -1., -1., 0.,   
            1., -1., 0.,   
            1., 1., 0.,  
            -1., 1., 0.,  
        ]
    }
    pub fn normal() -> [f32; 12] {
        [
            0., 0., 1.,     
            0., 0., 1.,     
            0., 0., 1.,     
            0., 0., 1.,
        ]
    }
    pub fn vertices() -> [f32; 32] {
        [
            -1., -1.,  0.,      0., 0., 1.,     0., 0.,    
             1., -1.,  0.,      0., 0., 1.,     1., 0.,   
             1.,  1.,  0.,      0., 0., 1.,     1., 1.,     
            -1.,  1.,  0.,      0., 0., 1.,     0., 1.,
        ]
    }
    pub fn indices() -> [u16;6] {
        [
            0, 1, 2, 0, 2, 3
        ]
    }
    pub fn uvs() -> [f32; 8] {
        [
            0., 0.,   
            1., 0.,   
            1., 1.,     
            0., 1.,
        ]
    }
}

pub trait InterfaceQuad {
    fn regist_quad(
        &self
    ) -> &Self;
    fn new_quad(
        & self,
        scene: ObjectID,
    ) -> ObjectID;
}

impl InterfaceQuad for Engine {
    fn regist_quad(
        &self
    ) -> &Self {
        self.create_vertex_buffer(KeyVertexBuffer::from(QuadBuilder::KEY_BUFFER), bytemuck::cast_slice(&QuadBuilder::vertices()).iter().map(|v| *v).collect::<Vec<u8>>());
        self.create_vertex_buffer(KeyVertexBuffer::from(QuadBuilder::KEY_BUFFER_INDICES), bytemuck::cast_slice(&QuadBuilder::indices()).iter().map(|v| *v).collect::<Vec<u8>>());

        self
    }
    fn new_quad(
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
            QuadBuilder::attrs_meta()
        );
        self.use_indices(entity, QuadBuilder::indices_meta());

        entity
    }
}

pub struct PluginQuadBuilder;
impl Plugin for PluginQuadBuilder {
    fn init(
        &mut self,
        engine: &mut Engine,
        stages: &mut pi_engine_shell::run_stage::RunStage,
    ) -> Result<(), ErrorPlugin> {
        engine.regist_quad();

        Ok(())
    }
}
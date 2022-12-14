

use pi_assets::mgr::AssetMgr;
use pi_engine_shell::{object::InterfaceObject, assets::sync_load::{InterfaceAssetSyncCreate, AssetSyncWait}};
use pi_render::rhi::{device::RenderDevice, RenderQueue};
use pi_share::Share;
use render_data_container::{VertexBuffer, EVertexDataFormat, KeyVertexBuffer};
use render_geometry::{indices::{IndicesBufferDesc}, vertex_data::{VertexBufferDesc, VertexAttribute, EVertexDataKind}};

use crate::{
    plugin::{Plugin, ErrorPlugin},
    object::{ObjectID},
    engine::Engine,
    scene::{ interface::InterfaceScene},
    transforms::interface::InterfaceTransformNode, geometry::{TInterfaceGeomtery, indices::InterfaceBufferIndices}
};

use super::interface::InterfaceMesh;

pub struct QuadBuilder;
impl QuadBuilder {
    const KEY_BUFFER_COLOR4:    &'static str = "QuadColor4";
    const KEY_BUFFER_POSITION:  &'static str = "QuadPosition";
    const KEY_BUFFER_NORMAL:    &'static str = "QuadNormal";
    const KEY_BUFFER_UV:        &'static str = "QuadUV";
    const KEY_BUFFER_INDICES:   &'static str = "QuadIndices";
    pub fn position(
        device: &RenderDevice,
        queue: &RenderQueue,
    ) -> VertexBuffer {
        let mut position = VertexBuffer::new(true, EVertexDataFormat::F32, false);
        let mut data = [
            -1., -1., 0.,   1., -1., 0.,   1., 1., 0.,  -1., 1., 0.,  
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
            0., 0., 1.,     0., 0., 1.,     0., 0., 1.,     0., 0., 1.,
        ];
        let mut normals = VertexBuffer::new(true, EVertexDataFormat::F32, false);
        normals.update_f32(&data, 0);
        normals.update_buffer(device, queue);
        normals
    }
    pub fn indices(
        device: &RenderDevice,
        queue: &RenderQueue,
    ) -> VertexBuffer {
        let data = [
            0, 1, 2, 0, 2, 3
        ];
        let mut indices = VertexBuffer::new(true, EVertexDataFormat::U16, true);
        indices.update_u16(&data, 0);
        indices.update_buffer(device, queue);

        indices
    }
    pub fn uvs(
        device: &RenderDevice,
        queue: &RenderQueue,
    ) -> VertexBuffer {
        let data = [
            0., 0.,   1., 0.,   1., 1.,     0., 1.,
        ];
        let mut indices = VertexBuffer::new(true, EVertexDataFormat::F32, false);
        indices.update_f32(&data, 0);
        indices.update_buffer(device, queue);
        indices
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
        let world = self.world();
        let device = world.get_resource::<RenderDevice>().unwrap();
        let queue = world.get_resource::<RenderQueue>().unwrap();

        let keypos = KeyVertexBuffer::from(QuadBuilder::KEY_BUFFER_POSITION);
        self.create_vertex_buffer(keypos.clone(), QuadBuilder::position(device, queue));

        let keynormal = KeyVertexBuffer::from(QuadBuilder::KEY_BUFFER_NORMAL);
        self.create_vertex_buffer(keynormal.clone(), QuadBuilder::normal(device, queue));
        
        let keyuv = KeyVertexBuffer::from(QuadBuilder::KEY_BUFFER_UV);
        self.create_vertex_buffer(keyuv.clone(), QuadBuilder::uvs(device, queue));

        let key = KeyVertexBuffer::from(QuadBuilder::KEY_BUFFER_INDICES);
        self.create_vertex_buffer(key.clone(), QuadBuilder::indices(device, queue));

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

        let keypos = KeyVertexBuffer::from(QuadBuilder::KEY_BUFFER_POSITION);

        let keynormal = KeyVertexBuffer::from(QuadBuilder::KEY_BUFFER_NORMAL);
        
        let keyuv = KeyVertexBuffer::from(QuadBuilder::KEY_BUFFER_UV);

        let key = KeyVertexBuffer::from(QuadBuilder::KEY_BUFFER_INDICES);
        
        self.use_geometry(
            entity,
            vec![
                VertexBufferDesc { bufferkey: keypos, range: None, attributes: vec![VertexAttribute { kind: EVertexDataKind::Position, format: wgpu::VertexFormat::Float32x3 }], step_mode: wgpu::VertexStepMode::Vertex },
                VertexBufferDesc { bufferkey: keynormal, range: None, attributes: vec![VertexAttribute { kind: EVertexDataKind::Normal, format: wgpu::VertexFormat::Float32x3 }], step_mode: wgpu::VertexStepMode::Vertex },
                VertexBufferDesc { bufferkey: keyuv, range: None, attributes: vec![VertexAttribute { kind: EVertexDataKind::UV, format: wgpu::VertexFormat::Float32x2 }], step_mode: wgpu::VertexStepMode::Vertex },
            ]
        );
        self.use_indices(entity, IndicesBufferDesc { format: wgpu::IndexFormat::Uint16, buffer_range: None, buffer: key });

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
        engine.regist_quad();

        Ok(())
    }
}
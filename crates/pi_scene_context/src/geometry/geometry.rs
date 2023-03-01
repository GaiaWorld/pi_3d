use std::ops::Range;

use pi_map::vecmap::VecMap;
use pi_render::renderer::{indices::{IndicesBufferDesc, AssetResBufferIndices}, vertices::{RenderVertices, RenderIndices, EVerticesBufferUsage}};

use super::vertex_buffer_useinfo::{
    AssetResVBSlot01, AssetDescVBSlot01,
    TVertexBufferUseInfo, TAssetResVertexBuffer,
    AssetResVBSlot02, AssetDescVBSlot02, 
    AssetResVBSlot03, AssetDescVBSlot03, 
    AssetResVBSlot04, AssetDescVBSlot04, 
    AssetResVBSlot05, AssetDescVBSlot05
};

pub struct RenderGeometryEable(pub bool);

pub trait RenderVerticesFrom {
    fn create<T0: TVertexBufferUseInfo, T1: TAssetResVertexBuffer>(useinfo: &T0, res: &T1) -> Self;
}
impl RenderVerticesFrom for RenderVertices {
    fn create<T0: TVertexBufferUseInfo, T1: TAssetResVertexBuffer>(useinfo: &T0, res: &T1) -> Self {
        Self {
            slot: T0::slot(),
            buffer: EVerticesBufferUsage::Other(res.buffer()),
            buffer_range: useinfo.range(),
            size_per_value: useinfo.desc().stride()
        }
    }
}

pub trait RenderIndicesFrom {
    fn create(item: (&IndicesBufferDesc, &AssetResBufferIndices)) -> Self;
}
impl RenderIndicesFrom for RenderIndices {
    fn create(item: (&IndicesBufferDesc, &AssetResBufferIndices)) -> Self {
        Self {
            buffer: EVerticesBufferUsage::Other(item.1.0.clone()),
            buffer_range: item.0.buffer_range.clone(),
            format: item.0.format,
        }
    }
}

#[derive(Debug, Clone)]
pub struct RenderGeometry {
    pub vertices: Vec<RenderVertices>,
    pub instances: Vec<RenderVertices>,
    pub indices: Option<RenderIndices>,
}
impl RenderGeometry {
    pub fn vertices(&self) -> VecMap<RenderVertices> {
        let mut result = VecMap::default();
        let mut index = 0;
        self.vertices.iter().for_each(|item| {
            result.insert(index, item.clone());
            index += 1;
        });
        self.instances.iter().for_each(|item| {
            result.insert(index, item.clone());
            index += 1;
        });

        result
    }
    pub fn instances(&self) -> Range<u32> {
        if let Some(item) = self.instances.get(0) {
            item.value_range()
        } else {
            0..1
        }
    }
}

impl From<
    (
        &AssetDescVBSlot01, &AssetResVBSlot01
        , Option<&IndicesBufferDesc>, Option<&AssetResBufferIndices>
    )
> for RenderGeometry {
    fn from(
        value: (
            &AssetDescVBSlot01, &AssetResVBSlot01
            , Option<&IndicesBufferDesc>, Option<&AssetResBufferIndices>
        )
    ) -> Self {
        let mut vertices = vec![];
        let mut instances = vec![];

        let mut vertex_count = 0;

        let render_vertices = RenderVertices::create(value.0, value.1);
        if value.0.desc().step_mode() == wgpu::VertexStepMode::Vertex { vertices.push(render_vertices) } else { instances.push(render_vertices) };

        let indices = if let (Some(desc), Some(val)) = (value.2, value.3) {
            Some(RenderIndices::create((desc, val)))
        } else { None};

        Self {
            vertices,
            instances,
            indices
        }
    }
}
impl From<
    (   &AssetDescVBSlot01, &AssetResVBSlot01
        , &AssetDescVBSlot02, &AssetResVBSlot02
        , Option<&IndicesBufferDesc>, Option<&AssetResBufferIndices>
    )
> for RenderGeometry {
    fn from(
        value: (
            &AssetDescVBSlot01, &AssetResVBSlot01
            , &AssetDescVBSlot02, &AssetResVBSlot02
            , Option<&IndicesBufferDesc>, Option<&AssetResBufferIndices>
        )
    ) -> Self {
        let mut vertices = vec![];
        let mut instances = vec![];
        
        let render_vertices = RenderVertices::create(value.0, value.1);
        if value.0.desc().step_mode() == wgpu::VertexStepMode::Vertex { vertices.push(render_vertices) } else { instances.push(render_vertices) };

        let render_vertices = RenderVertices::create(value.2, value.3);
        if value.2.desc().step_mode() == wgpu::VertexStepMode::Vertex { vertices.push(render_vertices) } else { instances.push(render_vertices) };

        let indices = if let (Some(desc), Some(val)) = (value.4, value.5) {
            Some(RenderIndices::create((desc, val)))
        } else { None};

        Self {
            vertices,
            instances,
            indices
        }
    }
}

impl From<
    (   &AssetDescVBSlot01, &AssetResVBSlot01
        , &AssetDescVBSlot02, &AssetResVBSlot02
        , &AssetDescVBSlot03, &AssetResVBSlot03
        , Option<&IndicesBufferDesc>, Option<&AssetResBufferIndices>
    )
> for RenderGeometry {
    fn from(
        value: (
            &AssetDescVBSlot01, &AssetResVBSlot01
            , &AssetDescVBSlot02, &AssetResVBSlot02
            , &AssetDescVBSlot03, &AssetResVBSlot03
            , Option<&IndicesBufferDesc>, Option<&AssetResBufferIndices>
        )
    ) -> Self {
        let mut vertices = vec![];
        let mut instances = vec![];

        let render_vertices = RenderVertices::create(value.0, value.1);
        if value.0.desc().step_mode() == wgpu::VertexStepMode::Vertex { vertices.push(render_vertices) } else { instances.push(render_vertices) };

        let render_vertices = RenderVertices::create(value.2, value.3);
        if value.2.desc().step_mode() == wgpu::VertexStepMode::Vertex { vertices.push(render_vertices) } else { instances.push(render_vertices) };

        let render_vertices = RenderVertices::create(value.4, value.5);
        if value.4.desc().step_mode() == wgpu::VertexStepMode::Vertex { vertices.push(render_vertices) } else { instances.push(render_vertices) };

        let indices = if let (Some(desc), Some(val)) = (value.6, value.7) {
            Some(RenderIndices::create((desc, val)))
        } else { None};

        Self {
            vertices,
            instances,
            indices
        }
    }
}
impl From<
    (   &AssetDescVBSlot01, &AssetResVBSlot01
        , &AssetDescVBSlot02, &AssetResVBSlot02
        , &AssetDescVBSlot03, &AssetResVBSlot03
        , &AssetDescVBSlot04, &AssetResVBSlot04
        , Option<&IndicesBufferDesc>, Option<&AssetResBufferIndices>
    )
> for RenderGeometry {
    fn from(
        value: (
            &AssetDescVBSlot01, &AssetResVBSlot01
            , &AssetDescVBSlot02, &AssetResVBSlot02
            , &AssetDescVBSlot03, &AssetResVBSlot03
            , &AssetDescVBSlot04, &AssetResVBSlot04
            , Option<&IndicesBufferDesc>, Option<&AssetResBufferIndices>
        )
    ) -> Self {
        let mut vertices = vec![];
        let mut instances = vec![];

        let render_vertices = RenderVertices::create(value.0, value.1);
        if value.0.desc().step_mode() == wgpu::VertexStepMode::Vertex { vertices.push(render_vertices) } else { instances.push(render_vertices) };

        let render_vertices = RenderVertices::create(value.2, value.3);
        if value.2.desc().step_mode() == wgpu::VertexStepMode::Vertex { vertices.push(render_vertices) } else { instances.push(render_vertices) };

        let render_vertices = RenderVertices::create(value.4, value.5);
        if value.4.desc().step_mode() == wgpu::VertexStepMode::Vertex { vertices.push(render_vertices) } else { instances.push(render_vertices) };

        let render_vertices = RenderVertices::create(value.6, value.7);
        if value.6.desc().step_mode() == wgpu::VertexStepMode::Vertex { vertices.push(render_vertices) } else { instances.push(render_vertices) };

        let indices = if let (Some(desc), Some(val)) = (value.8, value.9) {
            Some(RenderIndices::create((desc, val)))
        } else { None};

        Self {
            vertices,
            instances,
            indices
        }
    }
}
impl From<
    (   &AssetDescVBSlot01, &AssetResVBSlot01
        , &AssetDescVBSlot02, &AssetResVBSlot02
        , &AssetDescVBSlot03, &AssetResVBSlot03
        , &AssetDescVBSlot04, &AssetResVBSlot04
        , &AssetDescVBSlot05, &AssetResVBSlot05
        , Option<&IndicesBufferDesc>, Option<&AssetResBufferIndices>
    )
> for RenderGeometry {
    fn from(
        value: (
            &AssetDescVBSlot01, &AssetResVBSlot01
            , &AssetDescVBSlot02, &AssetResVBSlot02
            , &AssetDescVBSlot03, &AssetResVBSlot03
            , &AssetDescVBSlot04, &AssetResVBSlot04
            , &AssetDescVBSlot05, &AssetResVBSlot05
            , Option<&IndicesBufferDesc>, Option<&AssetResBufferIndices>
        )
    ) -> Self {
        let mut vertices = vec![];
        let mut instances = vec![];

        let render_vertices = RenderVertices::create(value.0, value.1);
        if value.0.desc().step_mode() == wgpu::VertexStepMode::Vertex { vertices.push(render_vertices) } else { instances.push(render_vertices) };

        let render_vertices = RenderVertices::create(value.2, value.3);
        if value.2.desc().step_mode() == wgpu::VertexStepMode::Vertex { vertices.push(render_vertices) } else { instances.push(render_vertices) };

        let render_vertices = RenderVertices::create(value.4, value.5);
        if value.4.desc().step_mode() == wgpu::VertexStepMode::Vertex { vertices.push(render_vertices) } else { instances.push(render_vertices) };

        let render_vertices = RenderVertices::create(value.6, value.7);
        if value.6.desc().step_mode() == wgpu::VertexStepMode::Vertex { vertices.push(render_vertices) } else { instances.push(render_vertices) };

        let render_vertices = RenderVertices::create(value.8, value.9);
        if value.8.desc().step_mode() == wgpu::VertexStepMode::Vertex { vertices.push(render_vertices) } else { instances.push(render_vertices) };

        let indices = if let (Some(desc), Some(val)) = (value.10, value.11) {
            Some(RenderIndices::create((desc, val)))
        } else { None};

        Self {
            vertices,
            instances,
            indices
        }
    }
}
impl RenderGeometry {
    pub fn create(
        mut values: Vec<(wgpu::VertexStepMode, RenderVertices)>,
        indices: (Option<&IndicesBufferDesc>, Option<&AssetResBufferIndices>),
    ) -> Self {
        let mut vertices = vec![];
        let mut instances = vec![];

        values.drain(..).for_each(|(step_mode, render_vertices)| {
            if step_mode == wgpu::VertexStepMode::Vertex { vertices.push(render_vertices) } else { instances.push(render_vertices) };
        });

        let indices = if let (Some(desc), Some(val)) = indices {
            Some(RenderIndices::create((desc, val)))
        } else { None};

        Self {
            vertices,
            instances,
            indices,
        }
    }
}
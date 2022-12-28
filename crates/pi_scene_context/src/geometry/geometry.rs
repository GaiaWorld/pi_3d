use render_data_container::{TRenderGeometry, VertexBufferUse};
use render_geometry::{vertex_data::TVertexBufferDesc, indices::{IndicesBufferDesc, AssetResBufferIndices}};

use super::vertex_buffer_useinfo::{AssetResVBSlot1, AssetDescVBSlot1, TVertexBufferUseInfo, TAssetResVertexBuffer, AssetResVBSlot2, AssetResVBSlot3, AssetResVBSlot4, AssetDescVBSlot2, AssetDescVBSlot3, AssetDescVBSlot4, AssetResVBSlot5, AssetDescVBSlot5};

pub struct RenderGeometryEable(pub bool);

pub trait RenderVerticesFrom {
    fn create<T0: TVertexBufferUseInfo, T1: TAssetResVertexBuffer>(useinfo: &T0, res: &T1) -> Self;
}
impl RenderVerticesFrom for render_data_container::RenderVertices {
    fn create<T0: TVertexBufferUseInfo, T1: TAssetResVertexBuffer>(useinfo: &T0, res: &T1) -> Self {
        Self {
            slot: T0::slot(),
            buffer: res.buffer(),
            buffer_range: useinfo.range(),
            size_per_value: useinfo.desc().stride()
        }
    }
}

pub trait RenderIndicesFrom {
    fn create(item: (&IndicesBufferDesc, &AssetResBufferIndices)) -> Self;
}
impl RenderIndicesFrom for render_data_container::RenderIndices {
    fn create(item: (&IndicesBufferDesc, &AssetResBufferIndices)) -> Self {
        Self {
            buffer: VertexBufferUse::Handle(item.1.0.clone()),
            buffer_range: item.0.buffer_range.clone(),
            format: item.0.format,
        }
    }
}

pub struct RenderGeometry {
    pub vertices: Vec<render_data_container::RenderVertices>,
    pub instances: Vec<render_data_container::RenderVertices>,
}

impl From<
    (&AssetDescVBSlot1, &AssetResVBSlot1)
> for RenderGeometry {
    fn from(value: (&AssetDescVBSlot1, &AssetResVBSlot1)) -> Self {
        let mut vertices = vec![];
        let mut instances = vec![];

        let mut vertex_count = 0;

        let render_vertices = render_data_container::RenderVertices::create(value.0, value.1);
        if value.0.desc().step_mode() == wgpu::VertexStepMode::Vertex { vertices.push(render_vertices) } else { instances.push(render_vertices) };

        Self {
            vertices,
            instances,
        }
    }
}
impl From<
    (   &AssetDescVBSlot1, &AssetResVBSlot1
        , &AssetDescVBSlot2, &AssetResVBSlot2
    )
> for RenderGeometry {
    fn from(
        value: (
            &AssetDescVBSlot1, &AssetResVBSlot1
            , &AssetDescVBSlot2, &AssetResVBSlot2
        )
    ) -> Self {
        let mut vertices = vec![];
        let mut instances = vec![];
        
        let render_vertices = render_data_container::RenderVertices::create(value.0, value.1);
        if value.0.desc().step_mode() == wgpu::VertexStepMode::Vertex { vertices.push(render_vertices) } else { instances.push(render_vertices) };

        let render_vertices = render_data_container::RenderVertices::create(value.2, value.3);
        if value.2.desc().step_mode() == wgpu::VertexStepMode::Vertex { vertices.push(render_vertices) } else { instances.push(render_vertices) };

        Self {
            vertices,
            instances,
        }
    }
}

impl From<
    (   &AssetDescVBSlot1, &AssetResVBSlot1
        , &AssetDescVBSlot2, &AssetResVBSlot2
        , &AssetDescVBSlot3, &AssetResVBSlot3
    )
> for RenderGeometry {
    fn from(
        value: (
            &AssetDescVBSlot1, &AssetResVBSlot1
            , &AssetDescVBSlot2, &AssetResVBSlot2
            , &AssetDescVBSlot3, &AssetResVBSlot3
        )
    ) -> Self {
        let mut vertices = vec![];
        let mut instances = vec![];

        let render_vertices = render_data_container::RenderVertices::create(value.0, value.1);
        if value.0.desc().step_mode() == wgpu::VertexStepMode::Vertex { vertices.push(render_vertices) } else { instances.push(render_vertices) };

        let render_vertices = render_data_container::RenderVertices::create(value.2, value.3);
        if value.2.desc().step_mode() == wgpu::VertexStepMode::Vertex { vertices.push(render_vertices) } else { instances.push(render_vertices) };

        let render_vertices = render_data_container::RenderVertices::create(value.4, value.5);
        if value.4.desc().step_mode() == wgpu::VertexStepMode::Vertex { vertices.push(render_vertices) } else { instances.push(render_vertices) };

        Self {
            vertices,
            instances,
        }
    }
}
impl From<
    (   &AssetDescVBSlot1, &AssetResVBSlot1
        , &AssetDescVBSlot2, &AssetResVBSlot2
        , &AssetDescVBSlot3, &AssetResVBSlot3
        , &AssetDescVBSlot4, &AssetResVBSlot4
    )
> for RenderGeometry {
    fn from(
        value: (
            &AssetDescVBSlot1, &AssetResVBSlot1
            , &AssetDescVBSlot2, &AssetResVBSlot2
            , &AssetDescVBSlot3, &AssetResVBSlot3
            , &AssetDescVBSlot4, &AssetResVBSlot4
        )
    ) -> Self {
        let mut vertices = vec![];
        let mut instances = vec![];

        let render_vertices = render_data_container::RenderVertices::create(value.0, value.1);
        if value.0.desc().step_mode() == wgpu::VertexStepMode::Vertex { vertices.push(render_vertices) } else { instances.push(render_vertices) };

        let render_vertices = render_data_container::RenderVertices::create(value.2, value.3);
        if value.2.desc().step_mode() == wgpu::VertexStepMode::Vertex { vertices.push(render_vertices) } else { instances.push(render_vertices) };

        let render_vertices = render_data_container::RenderVertices::create(value.4, value.5);
        if value.4.desc().step_mode() == wgpu::VertexStepMode::Vertex { vertices.push(render_vertices) } else { instances.push(render_vertices) };

        let render_vertices = render_data_container::RenderVertices::create(value.6, value.7);
        if value.6.desc().step_mode() == wgpu::VertexStepMode::Vertex { vertices.push(render_vertices) } else { instances.push(render_vertices) };

        Self {
            vertices,
            instances,
        }
    }
}
impl From<
    (   &AssetDescVBSlot1, &AssetResVBSlot1
        , &AssetDescVBSlot2, &AssetResVBSlot2
        , &AssetDescVBSlot3, &AssetResVBSlot3
        , &AssetDescVBSlot4, &AssetResVBSlot4
        , &AssetDescVBSlot5, &AssetResVBSlot5
    )
> for RenderGeometry {
    fn from(
        value: (
            &AssetDescVBSlot1, &AssetResVBSlot1
            , &AssetDescVBSlot2, &AssetResVBSlot2
            , &AssetDescVBSlot3, &AssetResVBSlot3
            , &AssetDescVBSlot4, &AssetResVBSlot4
            , &AssetDescVBSlot5, &AssetResVBSlot5
        )
    ) -> Self {
        let mut vertices = vec![];
        let mut instances = vec![];

        let render_vertices = render_data_container::RenderVertices::create(value.0, value.1);
        if value.0.desc().step_mode() == wgpu::VertexStepMode::Vertex { vertices.push(render_vertices) } else { instances.push(render_vertices) };

        let render_vertices = render_data_container::RenderVertices::create(value.2, value.3);
        if value.2.desc().step_mode() == wgpu::VertexStepMode::Vertex { vertices.push(render_vertices) } else { instances.push(render_vertices) };

        let render_vertices = render_data_container::RenderVertices::create(value.4, value.5);
        if value.4.desc().step_mode() == wgpu::VertexStepMode::Vertex { vertices.push(render_vertices) } else { instances.push(render_vertices) };

        let render_vertices = render_data_container::RenderVertices::create(value.6, value.7);
        if value.6.desc().step_mode() == wgpu::VertexStepMode::Vertex { vertices.push(render_vertices) } else { instances.push(render_vertices) };

        let render_vertices = render_data_container::RenderVertices::create(value.8, value.9);
        if value.8.desc().step_mode() == wgpu::VertexStepMode::Vertex { vertices.push(render_vertices) } else { instances.push(render_vertices) };

        Self {
            vertices,
            instances,
        }
    }
}
impl RenderGeometry {
    pub fn create(
        mut values: Vec<(wgpu::VertexStepMode, render_data_container::RenderVertices)>
    ) -> Self {
        let mut vertices = vec![];
        let mut instances = vec![];

        values.drain(..).for_each(|(step_mode, render_vertices)| {
            if step_mode == wgpu::VertexStepMode::Vertex { vertices.push(render_vertices) } else { instances.push(render_vertices) };
        });

        Self {
            vertices,
            instances,
        }
    }
}
impl TRenderGeometry for RenderGeometry {
    fn vertices(&self) -> Vec<render_data_container::RenderVertices> {
        self.vertices.clone()
    }

    fn instances(&self) -> Vec<render_data_container::RenderVertices> {
        self.instances.clone()
    }
}
use std::ops::Range;
use derive_deref::{Deref, DerefMut};
use pi_scene_shell::prelude::*;

use super::{vertex_buffer_useinfo::TVertexBufferUseInfo, EVerteicesMemory};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemSet, PartialOrd, Ord)]
pub enum StageGeometry {
    Create,
    _GeoCreate,
    VertexBufferLoaded,
    _VertexBufferLoadedApply,
    GeometryLoaded,
    Upload,
}

#[derive(Component, Default)]
pub struct RenderGeometryEable(pub bool);

pub trait RenderVerticesFrom {
    fn create<T0: TVertexBufferUseInfo>(useinfo: &T0, res: EVerticesBufferUsage) -> Self;
}
impl RenderVerticesFrom for RenderVertices {
    fn create<T0: TVertexBufferUseInfo>(useinfo: &T0, res: EVerticesBufferUsage) -> Self {
        Self {
            slot: T0::slot(),
            buffer: res,
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
            buffer: item.1.0.clone(),
            buffer_range: item.0.buffer_range.clone(),
            format: item.0.format,
        }
    }
}

#[derive(Clone, Component, Default, Deref, DerefMut)]
pub struct RenderGeometryComp(pub Option<RenderGeometry>);

#[derive(Clone)]
pub struct RenderGeometry {
    pub vertices: Vec<RenderVertices>,
    pub instances: Vec<RenderVertices>,
    pub indices: Option<RenderIndices>,
    pub instance_memory: Option<EVerteicesMemory>,
}
impl RenderGeometry {

    pub fn vertices(&self) -> SmallVecMap<RenderVertices, 3> {
        let mut result = SmallVecMap::default();
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
    pub fn isok(&self) -> bool {
        let mut flag = true;
        let range = self.vertices[0].value_range();
        flag = flag && (range.end > range.start);

        if let Some(item) = self.instances.get(0) {
            let range = item.value_range();
            flag = flag && (range.end > range.start);
        }

        // log::warn!("flag: {:?}, {:?}, {:?}", flag, range.end, range.start);

        flag
    }
    pub fn create(
        mut values: Vec<(wgpu::VertexStepMode, RenderVertices)>,
        indices: (Option<&IndicesBufferDesc>, Option<&AssetResBufferIndices>),
        instance_memory: Option<EVerteicesMemory>,
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
            instance_memory
        }
    }
    pub fn vertex_range(&self) -> Range<u32> {
        let range = self.vertices[0].value_range();
        // range.end = range.end - range.start;
        // range.start = 0;
        range
    }
}
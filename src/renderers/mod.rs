use render_data_container::{TGeometryBufferID, GeometryBufferPool, TVertexBufferKindKey, EVertexDataFormat};
use render_geometry::geometry::{Geometry, GeometryBufferDesc};
use render_material::error::EMaterialError;

use crate::{meshes::Mesh, geometry::{VDK, GBID}};

pub mod pipeline;
pub mod render_default;

pub fn render_opaque<'a>(
    renderpass: &mut wgpu::RenderPass<'a>,
    geometry: &Geometry<VDK, GBID>,
    geo_buffer_pool: &'a GBP,
    attribute_descs: &Vec<GeometryBufferDesc<VBK>>,
) -> Result<(), EMaterialError> {
    attribute_descs.iter().for_each(|desc| {
        let data = geometry.get_vertices(desc);
        match desc.format {
            EVertexDataFormat::U8 => match data {
                Some(id) => {
                    match geo_buffer_pool.get_buffer(&id) {
                        Some(buffer) => {
                            renderpass.set_vertex_buffer(desc.slot, buffer.slice(..));
                        },
                        None => {},
                    }
                },
                None => {},
            },
            EVertexDataFormat::U16 => match data {
                Some(id) => {
                    match geo_buffer_pool.get_buffer(&id) {
                        Some(buffer) => {
                            renderpass.set_vertex_buffer(desc.slot, buffer.slice(..));
                        },
                        None => {},
                    }
                },
                None => {},
            },
            EVertexDataFormat::U32 => match data {
                Some(id) => {
                    match geo_buffer_pool.get_buffer(&id) {
                        Some(buffer) => {
                            renderpass.set_vertex_buffer(desc.slot, buffer.slice(..));
                        },
                        None => {},
                    }
                },
                None => {},
            },
            EVertexDataFormat::F32 => match data {
                Some(id) => {
                    match geo_buffer_pool.get_buffer(&id) {
                        Some(buffer) => {
                            renderpass.set_vertex_buffer(desc.slot, buffer.slice(..));
                        },
                        None => {},
                    }
                },
                None => {},
            },
            EVertexDataFormat::F64 => match data {
                Some(id) => {
                    match geo_buffer_pool.get_buffer(&id) {
                        Some(buffer) => {
                            renderpass.set_vertex_buffer(desc.slot, buffer.slice(..));
                        },
                        None => {},
                    }
                },
                None => {},
            },
        }
    });

    let instance_count = geometry.get_instanced_number(geo_buffer_pool);

    match geometry.get_indices() {
        Some(indices_buffer_id) => match geo_buffer_pool.get_buffer(&indices_buffer_id) {
            Some(buffer) => {
                renderpass.set_index_buffer(buffer.slice(..), wgpu::IndexFormat::Uint16);
                renderpass.draw_indexed(0..geo_buffer_pool.get_size(&indices_buffer_id) as u32, 0, 0..instance_count as u32);
                Ok(())
            },
            None => {
                Err(EMaterialError::NotFoundIndicesBuffer)
            },
        },
        None => {
            match geometry.get_vertices_number(geo_buffer_pool) {
                Some(count) => {
                    renderpass.draw(0..count as u32, 0..instance_count as u32);
                    Ok(())
                },
                None => {
                    Ok(())
                },
            }
        },
    }
}
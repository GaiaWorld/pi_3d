use render_material::MaterialPropertypeBlock;

use crate::{materials::MBKK, geometry::GeometryMeta};


pub struct PipelineQpaque;

impl PipelineQpaque {
    pub fn render<'a>(
        renderpass: &mut wgpu::RenderPass<'a>,
        geometrys: &Vec<&GeometryMeta>,
        materials: &Vec<&MaterialPropertypeBlock<MBKK>>,
        // shaders: &Vec<>
    ) {

    }
}
use render_material::MaterialPropertypeBlock;

use crate::{materials::MBKK};


pub struct PipelineQpaque;

impl PipelineQpaque {
    pub fn render<'a>(
        renderpass: &mut wgpu::RenderPass<'a>,
        materials: &Vec<&MaterialPropertypeBlock<MBKK>>,
        // shaders: &Vec<>
    ) {

    }
}
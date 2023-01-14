use pi_render::rhi::device::RenderDevice;
use render_resource::data_texture2d::DataTexture2D;


pub struct SkinTexture {
    pub tex: DataTexture2D
}

impl SkinTexture {
    pub fn new(device: &RenderDevice, bone_count: u32, frames: u32) -> Self {
        let tex = DataTexture2D::new_rgba_f32(device, (bone_count + 1) * 4, frames);

        Self {
            tex,
        }
    }
    pub fn update_row(&mut self, row_index: u32, data: &[u8]) {
        self.tex.update_row(row_index, data)
    }
}
use pi_render::rhi::device::RenderDevice;
use render_resource::data_texture2d::DataTexture2D;



pub struct SkinFramesTexture {
    pub tex: DataTexture2D
}

impl SkinFramesTexture {
    pub fn new(device: &RenderDevice, bone_count: u32, frames: u32, data: Option<&[u8]>) -> Self {
        let mut tex = DataTexture2D::new_rgba_f32(device, (bone_count + 1) * 4, frames);

        if let Some(data) = data {
            tex.update_row(0, data);
        }

        Self {
            tex,
        }
    }
}
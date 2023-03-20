use super::render_blend::ModelBlend;


pub struct RenderTargetState {
    // pub state: wgpu::ColorTargetState,
}
impl RenderTargetState {
    pub fn color_target(
        format: wgpu::TextureFormat,
        blend: &ModelBlend,
    ) -> [Option<wgpu::ColorTargetState>;1] {
        match blend.enable {
            true => {
                [
                    Some(wgpu::ColorTargetState {
                        format,
                        blend: Some(
                            wgpu::BlendState {
                                color: wgpu::BlendComponent {
                                    src_factor: blend.src_color,
                                    dst_factor: blend.dst_color,
                                    operation: blend.opt_color,
                                },
                                alpha: wgpu::BlendComponent {
                                    src_factor: blend.src_alpha,
                                    dst_factor: blend.dst_alpha,
                                    operation: blend.opt_alpha,
                                },
                            }
                        ),
                        write_mask: wgpu::ColorWrites::ALL,
                    })
                ]
            },
            false => {
                [
                    Some(wgpu::ColorTargetState {
                        format,
                        blend: None,
                        write_mask: wgpu::ColorWrites::ALL,
                    })
                ]
            },
        }
    }
}
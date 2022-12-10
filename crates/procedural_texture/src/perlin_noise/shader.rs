use pi_atom::Atom;
use pi_render::rhi::{device::RenderDevice};
use render_shader::{shader::{KeyPreShader, ResPreShaderMeta, PreShaderMeta}, block_code::{BlockCode, BlockCodeAtom}, varying_code::{Varying, Varyings}, unifrom_code::MaterialValueBindDesc};

pub struct PerlinNoiseShader {
    pub vs_module: wgpu::ShaderModule,
    pub fs_module: wgpu::ShaderModule,
}

impl PerlinNoiseShader {
    pub const KEY: &str     = "PerlinNoiseShader";

    pub fn res() -> PreShaderMeta {
        PreShaderMeta::new(
            MaterialValueBindDesc {
                set: 1,
                bind: 1,
                stage: wgpu::ShaderStages::VERTEX_FRAGMENT,
                mat4_list: vec![],
                mat2_list: vec![],
                vec4_list: vec![],
                vec2_list: vec![],
                float_list: vec![Atom::from("size"), Atom::from("width"), Atom::from("height"), Atom::from("placeholder"), ],
                int_list: vec![],
                uint_list: vec![],
            },
            None,
            Varyings(
                vec![
                ]
            ),
            BlockCodeAtom { 
                define: Atom::from(""), 
                running: Atom::from(include_str!("../assets/skybox.vert"))
            },
            BlockCodeAtom { 
                define: Atom::from(include_str!("./perlin_noise_define.frag")),
                running: Atom::from(include_str!("./perlin_noise.frag"))
            },
        )
    }
}
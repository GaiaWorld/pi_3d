use pi_atom::Atom;
use pi_render::rhi::{device::RenderDevice};
use pi_scene_context::materials::shader_effect::{ShaderEffectMeta, UniformPropertyFloat};
use render_shader::{shader::{}, block_code::{BlockCode, BlockCodeAtom}, varying_code::{Varying, Varyings}, unifrom_code::MaterialValueBindDesc};

pub struct PerlinNoiseShader {
    pub vs_module: wgpu::ShaderModule,
    pub fs_module: wgpu::ShaderModule,
}

impl PerlinNoiseShader {
    pub const KEY: &'static str     = "PerlinNoiseShader";

    pub fn meta() -> ShaderEffectMeta {
        ShaderEffectMeta::new(
            MaterialValueBindDesc {
                stage: wgpu::ShaderStages::VERTEX_FRAGMENT,
                mat4_list: vec![],
                mat2_list: vec![],
                vec4_list: vec![],
                vec2_list: vec![],
                float_list: vec![
                    UniformPropertyFloat(Atom::from("size"), 1.),
                    UniformPropertyFloat(Atom::from("width"), 1.),
                    UniformPropertyFloat(Atom::from("height"), 1.),
                ],
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
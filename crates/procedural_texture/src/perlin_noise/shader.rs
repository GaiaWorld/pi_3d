use pi_atom::Atom;
use pi_render::rhi::{device::RenderDevice};
use pi_scene_context::materials::shader_effect::{ShaderEffectMeta};
use pi_render::{render_3d::shader::{uniform_value::{MaterialValueBindDesc, UniformPropertyVec4, UniformPropertyFloat}, uniform_texture::UniformTexture2DDesc, UniformPropertyName, varying_code::{Varyings, Varying}, block_code::BlockCodeAtom, shader_defines::ShaderDefinesSet}, renderer::{buildin_data::EDefaultTexture, shader_stage::EShaderStage}};

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
            vec![],
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
            ShaderDefinesSet::default()
        )
    }
}
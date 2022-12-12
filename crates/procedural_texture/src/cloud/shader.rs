use pi_atom::Atom;
use pi_render::rhi::{device::RenderDevice};
use pi_scene_context::materials::material_meta::{ShaderEffectMeta, UniformPropertyVec4, UniformPropertyFloat};
use pi_scene_math::Vector4;
use render_shader::{shader::{KeyShaderEffect}, block_code::{BlockCode, BlockCodeAtom}, varying_code::{Varying, Varyings}, unifrom_code::MaterialValueBindDesc};

pub struct CloudShader {
    pub vs_module: wgpu::ShaderModule,
    pub fs_module: wgpu::ShaderModule,
}

impl CloudShader {
    pub const KEY: &str     = "CloudShader";

    pub fn meta() -> ShaderEffectMeta {
        ShaderEffectMeta::new(
            MaterialValueBindDesc {
                set: 1,
                bind: 1,
                stage: wgpu::ShaderStages::VERTEX_FRAGMENT,
                mat4_list: vec![],
                mat2_list: vec![],
                vec4_list: vec![
                    UniformPropertyVec4(Atom::from("skyColor"), Vector4::new(0.15, 0.68, 1.0, 1.0)),
                    UniformPropertyVec4(Atom::from("cloudColor"), Vector4::new(1., 1., 1., 1.)),
                ],
                vec2_list: vec![],
                float_list: vec![
                    UniformPropertyFloat(Atom::from("amplitude"), 1.),
                    UniformPropertyFloat(Atom::from("numOctaves"), 4.),
                    UniformPropertyFloat(Atom::from("width"), 800.),
                    UniformPropertyFloat(Atom::from("height"), 600.),
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
                define: Atom::from(include_str!("./cloud_define.frag")),
                running: Atom::from(include_str!("./cloud.frag"))
            },
        )
    }
}
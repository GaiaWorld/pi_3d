use pi_atom::Atom;
use pi_render::rhi::{device::RenderDevice};
use pi_scene_context::materials::material_meta::{ShaderEffectMeta, UniformPropertyVec4};
use pi_scene_math::Vector4;
use render_shader::{shader::{}, block_code::{BlockCode, BlockCodeAtom}, varying_code::{Varying, Varyings}, unifrom_code::MaterialValueBindDesc};

pub struct SkyboxShader {
    pub vs_module: wgpu::ShaderModule,
    pub fs_module: wgpu::ShaderModule,
}

impl SkyboxShader {
    pub const KEY: &str     = "SkyboxShader";

    pub fn meta() -> ShaderEffectMeta {
        ShaderEffectMeta::new(
            MaterialValueBindDesc {
                set: 1,
                bind: 1,
                stage: wgpu::ShaderStages::VERTEX_FRAGMENT,
                mat4_list: vec![],
                mat2_list: vec![],
                vec4_list: vec![
                    UniformPropertyVec4(Atom::from("emissive"), Vector4::new(1., 1., 1., 0.5)),
                ],
                vec2_list: vec![],
                float_list: vec![],
                int_list: vec![],
                uint_list: vec![],
            },
            None,
            Varyings(
                vec![
                    Varying { format: Atom::from("vec3"), name: Atom::from("v_normal") },
                    Varying { format: Atom::from("float"), name: Atom::from("v_dist") },
                ]
            ),
            BlockCodeAtom { 
                define: Atom::from(include_str!("./assets/skybox_define.vert")), 
                running: Atom::from(include_str!("./assets/skybox.vert"))
            },
            BlockCodeAtom { 
                define: Atom::from(include_str!("./assets/skybox_define.frag")),
                running: Atom::from(include_str!("./assets/skybox.frag"))
            },
        )
    }
}
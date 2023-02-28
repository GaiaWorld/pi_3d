use pi_atom::Atom;
use pi_render::{rhi::{device::RenderDevice}, render_3d::shader::{uniform_value::{MaterialValueBindDesc, UniformPropertyVec4}, varying_code::{Varyings, Varying}, block_code::BlockCodeAtom, shader_defines::ShaderDefinesSet}};
use pi_scene_context::materials::shader_effect::{ShaderEffectMeta};

pub struct SkyboxShader {
    pub vs_module: wgpu::ShaderModule,
    pub fs_module: wgpu::ShaderModule,
}

impl SkyboxShader {
    pub const KEY: &'static str     = "SkyboxShader";

    pub fn meta() -> ShaderEffectMeta {
        ShaderEffectMeta::new(
            MaterialValueBindDesc {
                stage: wgpu::ShaderStages::VERTEX_FRAGMENT,
                mat4_list: vec![],
                mat2_list: vec![],
                vec4_list: vec![
                    UniformPropertyVec4(Atom::from("emissive"), [1., 1., 1., 0.5]),
                ],
                vec2_list: vec![],
                float_list: vec![],
                int_list: vec![],
                uint_list: vec![],
            },
            vec![],
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
            ShaderDefinesSet::default()
        )
    }
}
use pi_atom::Atom;
use pi_render::rhi::{device::RenderDevice};
use render_shader::{shader::{KeyPreShader, ResPreShaderMeta, PreShaderMeta}, block_code::{BlockCode, BlockCodeAtom}, varying_code::{Varying, Varyings}, unifrom_code::{MaterialValueBindDesc, MaterialTextureBindDesc, UniformTextureDesc, UniformPropertyName}};

pub struct UnlitShader {
    pub vs_module: wgpu::ShaderModule,
    pub fs_module: wgpu::ShaderModule,
}

impl UnlitShader {
    pub const KEY: &str     = "UnlitShader";

    pub fn res() -> PreShaderMeta {
        PreShaderMeta::new(
            MaterialValueBindDesc {
                set: 1,
                bind: 1,
                stage: wgpu::ShaderStages::VERTEX_FRAGMENT,
                mat4_list: vec![],
                mat2_list: vec![],
                vec4_list: vec![Atom::from("emissive"), Atom::from("emissive_scaleoffset")],
                vec2_list: vec![],
                float_list: vec![],
                int_list: vec![],
                uint_list: vec![],
            },
            Some(MaterialTextureBindDesc {
                set: 2,
                list: vec![
                    UniformTextureDesc::new2d(UniformPropertyName::from("_MainTex"), wgpu::ShaderStages::FRAGMENT)
                ]
            }),
            Varyings(
                vec![
                    Varying { format: Atom::from("vec3"), name: Atom::from("v_normal") },
                    Varying { format: Atom::from("vec3"), name: Atom::from("v_pos") },
                    Varying { format: Atom::from("vec2"), name: Atom::from("v_uv") },
                ]
            ),
            BlockCodeAtom { 
                define: Atom::from(include_str!("./unlit_define.vert")), 
                running: Atom::from(include_str!("./unlit.vert"))
            },
            BlockCodeAtom { 
                define: Atom::from(include_str!("./unlit_define.frag")),
                running: Atom::from(include_str!("./unlit.frag"))
            },
        )
    }
}
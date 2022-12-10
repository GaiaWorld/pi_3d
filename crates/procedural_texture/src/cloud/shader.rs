use pi_atom::Atom;
use pi_render::rhi::{device::RenderDevice};
use render_shader::{shader::{KeyPreShader, ResPreShaderMeta, PreShaderMeta}, block_code::{BlockCode, BlockCodeAtom}, varying_code::{Varying, Varyings}, unifrom_code::MaterialValueBindDesc};

pub struct CloudShader {
    pub vs_module: wgpu::ShaderModule,
    pub fs_module: wgpu::ShaderModule,
}

impl CloudShader {
    pub const KEY: &str     = "CloudShader";

    pub fn res() -> PreShaderMeta {
        PreShaderMeta::new(
            MaterialValueBindDesc {
                set: 1,
                bind: 1,
                stage: wgpu::ShaderStages::VERTEX_FRAGMENT,
                mat4_list: vec![],
                mat2_list: vec![],
                vec4_list: vec![Atom::from("skyColor"), Atom::from("cloudColor"), ],
                vec2_list: vec![],
                float_list: vec![Atom::from("amplitude"), Atom::from("numOctaves"), Atom::from("width"), Atom::from("height"), ],
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
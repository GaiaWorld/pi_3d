
use pi_atom::Atom;
use pi_render::rhi::{device::RenderDevice};
use pi_scene_context::materials::material_meta::{ShaderEffectMeta, UniformPropertyFloat, UniformPropertyVec4};
use pi_scene_math::Vector4;
use render_shader::{shader::{}, block_code::{BlockCode, BlockCodeAtom}, varying_code::{Varying, Varyings}, unifrom_code::{MaterialValueBindDesc, UniformTextureDesc, UniformPropertyName, MaterialTextureBindDesc}};

pub struct BRDFShader {
    pub vs_module: wgpu::ShaderModule,
    pub fs_module: wgpu::ShaderModule,
}

impl BRDFShader {
    pub const KEY: &'static str     = "BrdfShader";

    pub fn meta() -> ShaderEffectMeta {
        ShaderEffectMeta::new(
            MaterialValueBindDesc {
                set: 1,
                bind: 1,
                stage: wgpu::ShaderStages::VERTEX_FRAGMENT,
                mat4_list: vec![],
                mat2_list: vec![],
                vec4_list: vec![UniformPropertyVec4(Atom::from("skyColor"), Vector4::new(0.15, 0.68, 1.0, 1.0)),],
                vec2_list: vec![],
                float_list: vec![],
                int_list: vec![],
                uint_list: vec![],
            },
            Some(MaterialTextureBindDesc {
                set: 2,
                list: vec![UniformTextureDesc::new2d(
                    UniformPropertyName::from("_MainTex"),
                    wgpu::ShaderStages::FRAGMENT,
                )],
            }),
            Varyings(
                vec![
                    Varying{ format: Atom::from("vec2"), name: Atom::from("v_UV") }, 
                ]
            ),
            BlockCodeAtom { 
                define: Atom::from(""), 
                running: Atom::from(include_str!("./brdf.vert"))
            },
            BlockCodeAtom { 
                define: Atom::from(include_str!("./brdf_define.frag")),
                running: Atom::from(include_str!("./brdf.frag"))
            },
        )
    }
}
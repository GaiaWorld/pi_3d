use pi_atom::Atom;
use pi_scene_shell::prelude::*;
use pi_node_materials::prelude::*;

pub struct UnlitShader {
    pub vs_module: wgpu::ShaderModule,
    pub fs_module: wgpu::ShaderModule,
}

impl UnlitShader {
    pub const KEY: &'static str = "UnlitShader";

    pub fn meta() -> ShaderEffectMeta {

        let mut nodemat = NodeMaterialBuilder::new();
        nodemat.fs_define = String::from(include_str!("./unlit_define.frag"));

        nodemat.vs = String::from(include_str!("./unlit.vert"));
        nodemat.fs = String::from(include_str!("./unlit.frag"));

        nodemat.varyings = Varyings(
            vec![
                Varying { 
                    format: Atom::from(S_VEC3),
                    name: Atom::from(S_V_NORMAL),
                },
                Varying { 
                    format: Atom::from(S_VEC3),
                    name: Atom::from(S_V_POS),
                },
                Varying {
                    format: Atom::from(S_VEC2),
                    name: Atom::from(S_V_UV),
                },
                Varying { 
                    format: Atom::from(S_VEC4),
                    name: Atom::from(S_V_COLOR),
                },
            ]
        );

        nodemat.apply::<BlockUVAtlas>();
        nodemat.apply::<BlockUVOffsetSpeed>();
        nodemat.apply::<BlockMainTexture>();
        nodemat.apply::<BlockMainTextureUVOffsetSpeed>();

        nodemat.meta()

        // ShaderEffectMeta::new(
        //     MaterialValueBindDesc {
        //         stage: wgpu::ShaderStages::VERTEX_FRAGMENT,
        //         mat4_list: vec![],
        //         mat2_list: vec![],
        //         vec4_list: vec![
        //             UniformPropertyVec4(Atom::from("emissive"), [1., 1., 1., 1.]),
        //             UniformPropertyVec4(
        //                 Atom::from("emissive_scaleoffset"),
        //                 [1., 1., 0., 0.],
        //             ),
        //         ],
        //         vec2_list: vec![],
        //         float_list: vec![],
        //         int_list: vec![],
        //         uint_list: vec![],
        //     },
        //     vec![
        //         UniformTexture2DDesc::new(
        //             UniformPropertyName::from("_MainTex"),
        //             wgpu::TextureSampleType::Float { filterable: true },
        //             false,
        //             EShaderStage::FRAGMENT,
        //             EDefaultTexture::White
        //         )
        //     ],
        //     Varyings(vec![
        //         Varying {
        //             format: Atom::from(pi_scene_shell::prelude::S_VEC3),
        //             name: Atom::from(pi_scene_shell::prelude::S_V_NORMAL),
        //         },
        //         Varying {
        //             format: Atom::from(pi_scene_shell::prelude::S_VEC3),
        //             name: Atom::from(S_V_POS),
        //         },
        //         Varying {
        //             format: Atom::from(S_VEC2),
        //             name: Atom::from(pi_scene_shell::prelude::S_V_UV),
        //         },
        //         Varying {
        //             format: Atom::from(S_VEC4),
        //             name: Atom::from(pi_scene_shell::prelude::S_V_COLOR),
        //         },
        //     ]),
        //     BlockCodeAtom {
        //         define: Atom::from(include_str!("./unlit_define.vert")),
        //         running: Atom::from(include_str!("./unlit.vert")),
        //     },
        //     BlockCodeAtom {
        //         define: Atom::from(include_str!("./unlit_define.frag")),
        //         running: Atom::from(include_str!("./unlit.frag")),
        //     },
        //     ShaderDefinesSet::default()
        // )
    }
}

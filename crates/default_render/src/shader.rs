use pi_atom::Atom;
use pi_engine_shell::prelude::*;
use pi_node_materials::prelude::*;

pub struct DefaultShader;
impl DefaultShader {
    pub const KEY: &'static str = "Default";
    pub fn res() -> ShaderEffectMeta {
        let mut nodemat = NodeMaterialBuilder::new();
        nodemat.values.int_list.push(UniformPropertyInt(Atom::from("debug_normal"), 0));
        nodemat.fs_define = String::from(include_str!("./default_define.frag"));

        nodemat.vs = String::from(include_str!("./default.vert"));
        nodemat.fs = String::from(include_str!("./default.frag"));

        nodemat.varyings = Varyings(
            vec![
                Varying { 
                    format: Atom::from("vec3"),
                    name: Atom::from("v_normal"),
                },
                Varying { 
                    format: Atom::from("vec3"),
                    name: Atom::from("v_pos"),
                },
                Varying { 
                    format: Atom::from("vec4"),
                    name: Atom::from("v_color"),
                },
            ]
        );

        nodemat.meta()

    //     ShaderEffectMeta::new(
    //         MaterialValueBindDesc {
    //             stage: wgpu::ShaderStages::VERTEX_FRAGMENT,
    //             mat4_list: vec![],
    //             mat2_list: vec![],
    //             vec4_list: vec![],
    //             vec2_list: vec![],
    //             float_list: vec![],
    //             int_list: vec![UniformPropertyInt(Atom::from("debug_normal"), 0)],
    //             uint_list: vec![],
    //         },
    //         vec![],
    //         Varyings(
    //             vec![
    //                 Varying { 
    //                     format: Atom::from("vec3"),
    //                     name: Atom::from("v_normal"),
    //                 },
    //                 Varying { 
    //                     format: Atom::from("vec3"),
    //                     name: Atom::from("v_pos"),
    //                 },
    //                 Varying { 
    //                     format: Atom::from("vec4"),
    //                     name: Atom::from("v_color"),
    //                 },
    //             ]
    //         ),
    //         BlockCodeAtom { 
    //             define: Atom::from(""), 
    //             running: Atom::from(include_str!("./default.vert"))
    //         },
    //         BlockCodeAtom { 
    //             define: Atom::from(include_str!("./default_define.frag")), 
    //             running: Atom::from(include_str!("./default.frag"))
    //         },
    //         ShaderDefinesSet::default()
    //     )
    }
}

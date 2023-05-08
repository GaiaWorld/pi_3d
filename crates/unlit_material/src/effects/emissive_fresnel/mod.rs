use pi_atom::Atom;
use pi_engine_shell::prelude::*;
use pi_node_materials::prelude::*;

pub struct EmissiveFresnelShader;

impl EmissiveFresnelShader {
    pub const KEY: &'static str = "EmissiveFresnelShader";

    pub fn meta() -> ShaderEffectMeta {

        let mut nodemat = NodeMaterialBuilder::new();
        nodemat.fs_define = String::from("\r\nlayout(location = 0) out vec4 gl_FragColor; \r\n");

        nodemat.vs = String::from(include_str!("../base.vert"));
        nodemat.fs = String::from(include_str!("./emissive_fresnel.frag"));

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
                    format: Atom::from("vec2"),
                    name: Atom::from("v_uv"),
                },
                Varying { 
                    format: Atom::from("vec4"),
                    name: Atom::from("v_color"),
                },
            ]
        );

        nodemat.apply::<BlockFloat>();
        nodemat.apply::<BlockFresnel>();
        nodemat.apply::<BlockViewDirection>();
        nodemat.apply::<BlockEmissiveBase>();
        nodemat.apply::<BlockEmissiveFresnel>();

        nodemat.meta()
    }
}

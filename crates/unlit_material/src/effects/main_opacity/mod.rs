use pi_atom::Atom;
use pi_engine_shell::prelude::*;
use pi_node_materials::prelude::*;

pub struct MainOpacityShader;

impl MainOpacityShader {
    pub const KEY: &'static str = "MainOpacityShader";

    pub fn meta() -> ShaderEffectMeta {

        let mut nodemat = NodeMaterialBuilder::new();
        nodemat.fs_define = String::from("\r\nlayout(location = 0) out vec4 gl_FragColor; \r\n");

        nodemat.vs = String::from(include_str!("../base.vert"));
        nodemat.fs = String::from(include_str!("./main_opacity.frag"));

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

        nodemat.apply::<BlockColorGray>();
        nodemat.apply::<BlockTextureChannel>();
        nodemat.apply::<BlockUVOffsetSpeed>();
        nodemat.apply::<BlockMainTexture>();
        nodemat.apply::<BlockMainTextureUVOffsetSpeed>();
        nodemat.apply::<BlockOpacity>();
        nodemat.apply::<BlockOpacityTexture>();
        nodemat.apply::<BlockOpacityTextureUVOffsetSpeed>();
        nodemat.apply::<BlockEmissiveBase>();

        nodemat.meta()
    }
}

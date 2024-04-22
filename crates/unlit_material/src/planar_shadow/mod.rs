use pi_atom::Atom;
use pi_node_materials::prelude::NodeMaterialBuilder;
use pi_scene_context::pass::{ShaderEffectMeta, UniformPropertyVec4, Varying, Varyings};

pub struct PlanarShadow;
impl PlanarShadow {
    pub const KEY: &'static str = "PlanarShadow";

    pub fn meta() -> ShaderEffectMeta {

        let mut nodemat = NodeMaterialBuilder::new();
        nodemat.fs_define = String::from("\r\nlayout(location = 0) out vec4 gl_FragColor; \r\n");

        nodemat.vs = String::from(include_str!("./planar_shadow.vert"));
        nodemat.fs = String::from(include_str!("./planar_shadow.frag"));

        nodemat.varyings = Varyings(
            vec![
                Varying { 
                    format: Atom::from("vec4"),
                    name: Atom::from("v_color"),
                },
            ]
        );

        nodemat.values.vec4_list.push(UniformPropertyVec4(Atom::from("uLightDir"), [1., -1., 1., 0.], false));
        nodemat.values.vec4_list.push(UniformPropertyVec4(Atom::from("uShadowColor"), [0.2, 0.2, 0.2, 1.0], false));

        nodemat.meta()
    }
}

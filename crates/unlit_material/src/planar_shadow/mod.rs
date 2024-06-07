use pi_atom::Atom;
use pi_node_materials::prelude::NodeMaterialBuilder;
use pi_scene_context::pass::{ShaderEffectMeta, UniformPropertyVec4, Varying, Varyings};
use pi_scene_shell::prelude::*;

pub struct PlanarShadow;
impl PlanarShadow {
    pub const KEY: &'static str = "PlanarShadow";

    pub fn meta() -> ShaderEffectMeta {

        let mut nodemat = NodeMaterialBuilder::new();
        nodemat.fs_define = String::from(S_BREAK) + "layout(location = 0) out vec4 gl_FragColor;" + S_BREAK;

        nodemat.vs = String::from(include_str!("./planar_shadow.vert"));
        nodemat.fs = String::from(include_str!("./planar_shadow.frag"));

        nodemat.varyings = Varyings(
            vec![
                Varying { 
                    format: Atom::from(S_VEC4),
                    name: Atom::from(S_V_COLOR),
                },
            ]
        );

        let light = Vector3::new(2., -1., 2.).normalize();
        let [row1, row2, row3, _] = Self::rows(&Vector3::new(0., 1., 0.), 0., &light);
        nodemat.values.vec4_list.push(UniformPropertyVec4(Atom::from("uPlanarRow1"), row1, false));
        nodemat.values.vec4_list.push(UniformPropertyVec4(Atom::from("uPlanarRow2"), row2, false));
        nodemat.values.vec4_list.push(UniformPropertyVec4(Atom::from("uPlanarRow3"), row3, false));
        nodemat.values.vec4_list.push(UniformPropertyVec4(Atom::from("uPlanarDir"), [light.x, light.y, light.z, 0.], false));
        nodemat.values.vec4_list.push(UniformPropertyVec4(Atom::from("uShadowColor"), [0.2, 0.2, 0.2, 0.5], false));

        nodemat.meta()
    }
    pub fn matrix(plane_normal: &Vector3, plane_d: Number, light: &Vector3) -> [Number; 16] {
        let mut nl = plane_normal.dot(light);
        if nl.abs() < 0.0000001 {
            nl = 0.;
        } else {
            nl = 1. / nl;
        }
        let n = plane_normal;
        let l = light;
        let d = plane_d;
        [
            1. -n.x * l.x * nl,     -n.y * l.x * nl,    -n.z * l.x * nl,    -d * l.x * nl,
               -n.x * l.y * nl,  1. -n.y * l.y * nl,    -n.z * l.y * nl,    -d * l.y * nl,
               -n.x * l.z * nl,     -n.y * l.z * nl, 1. -n.z * l.z * nl,    -d * l.z * nl,
                       0.,             0.,            0.,          1.
        ]
    }
    pub fn rows(plane_normal: &Vector3, plane_d: Number, light: &Vector3) -> [[Number; 4];4] {
        let mut nl = plane_normal.dot(light);
        if nl.abs() < 0.0000001 {
            nl = 0.;
        } else {
            nl = 1. / nl;
        }
        let n = plane_normal;
        let l = light;
        let d = plane_d;
        // [
        // [   1. -n.x * l.x * nl,     -n.y * l.x * nl,    -n.z * l.x * nl,    -d * l.x * nl,],
        // [      -n.x * l.y * nl,  1. -n.y * l.y * nl,    -n.z * l.y * nl,    -d * l.y * nl,],
        // [      -n.x * l.z * nl,     -n.y * l.z * nl, 1. -n.z * l.z * nl,    -d * l.z * nl,],
        // [              0.,             0.,            0.,          1.]
        // ]

        [
        [   1. -n.x * l.x * nl,     -n.x * l.y * nl,    -n.x * l.z * nl,    0.,],
        [      -n.y * l.x * nl,  1. -n.y * l.y * nl,    -n.z * l.y * nl,    0.,],
        [      -n.z * l.x * nl,     -n.z * l.y * nl, 1. -n.z * l.z * nl,    0.,],
        [        -d * l.x * nl,       -d * l.y * nl,      -d * l.z * nl,    1.,]
        ]
    }
}

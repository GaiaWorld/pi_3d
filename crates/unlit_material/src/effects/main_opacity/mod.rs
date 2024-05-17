use pi_atom::Atom;
use pi_scene_shell::prelude::*;
use pi_node_materials::prelude::*;

pub struct MainOpacityShader;

impl MainOpacityShader {
    pub const KEY: &'static str = "MainOpacityShader";

    pub fn meta() -> ShaderEffectMeta {

        let mut nodemat = NodeMaterialBuilder::new();
        nodemat.fs_define = String::from(S_BREAK) + "layout(location = 0) out vec4 gl_FragColor;" + S_BREAK;

        nodemat.vs = String::from("
    mat4 finalWorld = PI_ObjectToWorld;

    vec4 position =  vec4(A_POSITION, 1.);
    vec4 worldPos =  finalWorld * position;
    // vec4 worldPos =  position;

    gl_Position = PI_MATRIX_VP * worldPos;
     
    // gl_Position = position;

    v_pos = worldPos.xyz;

    mat3 normalWorld = mat3(finalWorld);
    v_normal = A_NORMAL;
    
    v_uv = A_UV;
    v_uv2 = A_UV;
    v_uv3 = A_UV;
    v_color = A_COLOR4;
        ");
        nodemat.fs = String::from(include_str!("./main_opacity.frag"));

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
                    format: Atom::from(S_VEC2),
                    name: Atom::from("v_uv2"),
                },
                Varying {
                    format: Atom::from(S_VEC2),
                    name: Atom::from("v_uv3"),
                },
                Varying { 
                    format: Atom::from(S_VEC4),
                    name: Atom::from(S_V_COLOR),
                },
            ]
        );

        nodemat.apply::<BlockColorGray>();
        nodemat.apply::<BlockTextureChannel>();
        nodemat.apply::<BlockUVOffsetSpeed>();
        nodemat.apply::<BlockCutoff>();
        nodemat.apply::<BlockMainTexture>();
        nodemat.apply::<BlockMainTextureUVOffsetSpeed>();
        nodemat.apply::<BlockOpacity>();
        nodemat.apply::<BlockOpacityTexture>();
        nodemat.apply::<BlockOpacityTextureUVOffsetSpeed>();

        nodemat.meta()
    }
}

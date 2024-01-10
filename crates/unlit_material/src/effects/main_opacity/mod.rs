use pi_atom::Atom;
use pi_scene_shell::prelude::*;
use pi_node_materials::prelude::*;

pub struct MainOpacityShader;

impl MainOpacityShader {
    pub const KEY: &'static str = "MainOpacityShader";

    pub fn meta() -> ShaderEffectMeta {

        let mut nodemat = NodeMaterialBuilder::new();
        nodemat.fs_define = String::from("\r\nlayout(location = 0) out vec4 gl_FragColor; \r\n");

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
                    format: Atom::from("vec2"),
                    name: Atom::from("v_uv2"),
                },
                Varying {
                    format: Atom::from("vec2"),
                    name: Atom::from("v_uv3"),
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
        nodemat.apply::<BlockCutoff>();
        nodemat.apply::<BlockMainTexture>();
        nodemat.apply::<BlockMainTextureUVOffsetSpeed>();
        nodemat.apply::<BlockOpacity>();
        nodemat.apply::<BlockOpacityTexture>();
        nodemat.apply::<BlockOpacityTextureUVOffsetSpeed>();

        nodemat.meta()
    }
}

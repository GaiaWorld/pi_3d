use pi_atom::Atom;
use pi_engine_shell::prelude::*;
use pi_node_materials::{prelude::*, NodeMaterialBlocks};

pub struct MainOpacityFresnelShader;

impl MainOpacityFresnelShader {
    pub const KEY: &'static str = "MainOpacityFresnelShader";

    pub fn create(infos: &NodeMaterialBlocks) -> ShaderEffectMeta {
        let mut nodemat = NodeMaterialBuilder::new();
        nodemat.fs_define = String::from("\r\nlayout(location = 0) out vec4 gl_FragColor; \r\n");

        nodemat.vs = String::from(include_str!("../base.vert"));
        nodemat.fs = String::from(include_str!("./main_opacity_fresnel.frag"));

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

        nodemat.include(BlockFloat::KEY, infos);
        nodemat.include(BlockColorGray::KEY, infos);
        nodemat.include(BlockTextureChannel::KEY, infos);
        nodemat.include(BlockFresnel::KEY, infos);
        nodemat.include(BlockViewDirection::KEY, infos);
        nodemat.include(BlockUVOffsetSpeed::KEY, infos);
        nodemat.include(BlockMainTexture::KEY, infos);
        nodemat.include(BlockMainTextureUVOffsetSpeed::KEY, infos);
        nodemat.include(BlockOpacity::KEY, infos);
        nodemat.include(BlockOpacityTexture::KEY, infos);
        nodemat.include(BlockOpacityTextureUVOffsetSpeed::KEY, infos);
        nodemat.include(BlockOpacityFresnel::KEY, infos);
        nodemat.include(BlockEmissiveTexture::KEY, infos);
        nodemat.include(BlockEmissiveFresnel::KEY, infos);

        // log::warn!("MainOpacityFresnelShader Create");
        
        nodemat.meta()
    }
    pub fn meta() -> ShaderEffectMeta {

        let mut nodemat = NodeMaterialBuilder::new();
        nodemat.fs_define = String::from("\r\nlayout(location = 0) out vec4 gl_FragColor; \r\n");

        nodemat.vs = String::from(include_str!("../base.vert"));
        nodemat.fs = String::from(include_str!("./main_opacity_fresnel.frag"));

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
        nodemat.apply::<BlockColorGray>();
        nodemat.apply::<BlockTextureChannel>();
        nodemat.apply::<BlockFresnel>();
        nodemat.apply::<BlockViewDirection>();
        nodemat.apply::<BlockUVOffsetSpeed>();
        nodemat.apply::<BlockCutoff>();
        nodemat.apply::<BlockMainTexture>();
        nodemat.apply::<BlockMainTextureUVOffsetSpeed>();
        nodemat.apply::<BlockOpacityTexture>();
        nodemat.apply::<BlockOpacityTextureUVOffsetSpeed>();
        nodemat.apply::<BlockOpacityFresnel>();
        nodemat.apply::<BlockEmissiveTexture>();
        nodemat.apply::<BlockEmissiveFresnel>();

        nodemat.meta()
    }
}

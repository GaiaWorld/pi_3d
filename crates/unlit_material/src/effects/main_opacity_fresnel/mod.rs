use pi_atom::Atom;
use pi_scene_shell::prelude::*;
use pi_node_materials::{prelude::*, NodeMaterialBlocks};

pub struct MainOpacityFresnelShader;

impl MainOpacityFresnelShader {
    pub const KEY: &'static str = "MainOpacityFresnelShader";

    pub fn create(infos: &NodeMaterialBlocks) -> ShaderEffectMeta {
        let mut nodemat = NodeMaterialBuilder::new();
        nodemat.fs_define = String::from(S_BREAK) + "layout(location = 0) out vec4 gl_FragColor;" + S_BREAK;

        nodemat.vs = String::from(include_str!("../base.vert"));
        nodemat.fs = String::from(include_str!("./main_opacity_fresnel.frag"));

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

        nodemat.include(&Atom::from(BlockFloat::KEY), infos);
        nodemat.include(&Atom::from(BlockColorGray::KEY), infos);
        nodemat.include(&Atom::from(BlockTextureChannel::KEY), infos);
        nodemat.include(&Atom::from(BlockUVAtlas::KEY), infos);
        nodemat.include(&Atom::from(BlockFresnel::KEY), infos);
        nodemat.include(&Atom::from(BlockViewDirection::KEY), infos);
        nodemat.include(&Atom::from(BlockUVOffsetSpeed::KEY), infos);
        nodemat.include(&Atom::from(BlockMainTexture::KEY), infos);
        nodemat.include(&Atom::from(BlockMainTextureUVOffsetSpeed::KEY), infos);
        nodemat.include(&Atom::from(BlockOpacity::KEY), infos);
        nodemat.include(&Atom::from(BlockOpacityTexture::KEY), infos);
        nodemat.include(&Atom::from(BlockOpacityTextureUVOffsetSpeed::KEY), infos);
        nodemat.include(&Atom::from(BlockOpacityFresnel::KEY), infos);
        nodemat.include(&Atom::from(BlockEmissiveTexture::KEY), infos);
        nodemat.include(&Atom::from(BlockEmissiveFresnel::KEY), infos);

        // log::warn!("MainOpacityFresnelShader Create");
        
        nodemat.meta()
    }
    pub fn meta() -> ShaderEffectMeta {

        let mut nodemat = NodeMaterialBuilder::new();
        nodemat.fs_define = String::from(S_BREAK) + "layout(location = 0) out vec4 gl_FragColor;" + S_BREAK;

        nodemat.vs = String::from(include_str!("../base.vert"));
        nodemat.fs = String::from(include_str!("./main_opacity_fresnel.frag"));

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

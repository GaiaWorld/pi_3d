

use pi_scene_shell::prelude::*;

use pi_node_materials::prelude::*;
use pi_scene_context::prelude::*;

mod command;
mod interface;
mod effects;
mod planar_shadow;

pub use command::*;
pub use interface::*;
pub use effects::*;
pub use planar_shadow::*;

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

        nodemat.values.vec4_list.push(UniformPropertyVec4(Atom::from("uMainAtlas"), [11., 11., 0., 0.], true));

        nodemat.apply::<BlockUVAtlas>();
        nodemat.apply::<BlockUVOffsetSpeed>();
        nodemat.apply::<BlockMainTexture>();
        nodemat.apply::<BlockMainTextureUVOffsetSpeed>();

        nodemat.meta()
    }
}


pub struct PluginUnlitMaterial;
impl Plugin for PluginUnlitMaterial {
    fn build(&self, app: &mut App) {

        let asset_mgr = app.world.get_resource::<ShareAssetMgr<ShaderEffectMeta>>().unwrap().clone();
        ActionMaterial::regist_material_meta(&asset_mgr, KeyShaderMeta::from(UnlitShader::KEY), UnlitShader::meta());
        // app.add_startup_system(setup);
    }
}
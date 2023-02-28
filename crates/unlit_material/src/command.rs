
use std::{mem::replace, sync::Arc};

use pi_ecs::{prelude::{ResMut, Query, Commands}, query::{With}};
use pi_ecs_macros::setup;
use pi_engine_shell::run_stage::TSystemStageInfo;
use pi_render::render_3d::shader::uniform_texture::UniformTextureWithSamplerParam;
use pi_scene_math::Number;

use pi_scene_context::{
    object::{ObjectID, GameObject},
    materials::{uniforms::{texture::TextureSlot01, uniform::BindEffectValues}, shader_effect::AssetResShaderEffectMeta},
};

#[derive(Debug, Clone)]
pub enum EUnlitMaterialCommand {
    EmissiveColor(ObjectID, (Number, Number, Number)),
    EmissiveIntensity(ObjectID, Number),
    EmissiveTexture(ObjectID, UniformTextureWithSamplerParam),
}
#[derive(Default)]
pub struct SingleUnlitMaterialCommandList {
    pub list: Vec<EUnlitMaterialCommand>,
}
pub struct SysUnlitMaterialCommand;
impl TSystemStageInfo for SysUnlitMaterialCommand {}
#[setup]
impl SysUnlitMaterialCommand {
    #[system]
    pub fn cmd(
        mut cmds: ResMut<SingleUnlitMaterialCommandList>,
        mut material_vec4: Query<
            GameObject,
            &mut BindEffectValues,
            With<AssetResShaderEffectMeta>
        >,
        mut tex01_cmd: Commands<GameObject, TextureSlot01>,
    ) {
        let mut list = replace(&mut cmds.list, vec![]);

        list.drain(..).for_each(|cmd| {
            match cmd {
                EUnlitMaterialCommand::EmissiveColor(entity, color) => {
                    match material_vec4.get_mut(entity) {
                        Some(mut valueuniform) => {
                            let a = valueuniform.vec4_.value(0)[3];
                            valueuniform.vec4(0, &[color.0, color.1, color.2, a]);
                        },
                        None => {
                            cmds.list.push(cmd.clone());
                        },
                    }
                },
                EUnlitMaterialCommand::EmissiveIntensity(entity, intensity) => {
                    match material_vec4.get_mut(entity) {
                        Some(mut valueuniform) => {
                            
                            let t = valueuniform.vec4_.value(0);
                            let r = t[0];
                            let g = t[1];
                            let b = t[2];
                            valueuniform.vec4(0, &[r, g, b, intensity]);
                        },
                        None => {
                            cmds.list.push(cmd.clone());
                        },
                    }
                },
                EUnlitMaterialCommand::EmissiveTexture(entity, imagepath) => {
                    tex01_cmd.insert(entity, TextureSlot01(Arc::new(imagepath)));
                },
            }
        });
    }
}
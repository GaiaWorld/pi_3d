
use std::mem::replace;

use pi_ecs::{prelude::{ResMut, Query, EntityDelete, Res}, query::{Write, With}, storage::Local};
use pi_ecs_macros::setup;
use pi_render::rhi::device::RenderDevice;
use pi_scene_math::Number;

use pi_scene_context::{object::{ObjectID, GameObject}, materials::{material::MaterialID, bind_group::{RenderBindGroupPool, RenderBindGroupKey}, uniforms::{vec4::Vec4Uniform, texture::TextureSlot1, texture_uniform::MaterialTextureBindGroupID}, material_meta::AssetResMaterailMeta}, resources::RenderDynUniformBuffer};
use render_resource::ImageAssetKey;

#[derive(Debug, Clone)]
pub enum EUnlitMaterialCommand {
    EmissiveColor(ObjectID, (Number, Number, Number)),
    EmissiveIntensity(ObjectID, Number),
    EmissiveTexture(ObjectID, ImageAssetKey),
}
#[derive(Default)]
pub struct SingleUnlitMaterialCommandList {
    pub list: Vec<EUnlitMaterialCommand>,
}
pub struct SysUnlitMaterialCommand;
#[setup]
impl SysUnlitMaterialCommand {
    #[system]
    pub fn cmd(
        mut cmds: ResMut<SingleUnlitMaterialCommandList>,
        mut materials: Query<
            GameObject,
            (Write<Vec4Uniform>, Write<TextureSlot1>),
            (With<AssetResMaterailMeta>, With<MaterialTextureBindGroupID>)
        >,
    ) {
        let mut list = replace(&mut cmds.list, vec![]);

        list.drain(..).for_each(|cmd| {
            match cmd {
                EUnlitMaterialCommand::EmissiveColor(entity, color) => {
                    match materials.get_mut(entity) {
                        Some((mut valueuniform, texuniform)) => {
                            if let Some(prop) = valueuniform.get_mut() {
                                let a = prop.value(0)[3];
                                prop.set(0, &[color.0, color.1, color.2, a]);
                                valueuniform.notify_modify();
                            } else {
                                cmds.list.push(cmd.clone());
                            }
                        },
                        None => {
                            cmds.list.push(cmd.clone());
                        },
                    }
                },
                EUnlitMaterialCommand::EmissiveIntensity(entity, intensity) => {
                    match materials.get_mut(entity) {
                        Some((mut valueuniform, texuniform)) => {
                            if let Some(prop) = valueuniform.get_mut() {
                                let t = prop.value(0);
                                let r = t[0];
                                let g = t[1];
                                let b = t[2];
                                prop.set(0, &[r, g, b, intensity]);
                                valueuniform.notify_modify();
                            } else {
                                cmds.list.push(cmd.clone());
                            }
                        },
                        None => {
                            cmds.list.push(cmd.clone());
                        },
                    }
                },
                EUnlitMaterialCommand::EmissiveTexture(entity, imagepath) => {
                    match materials.get_mut(entity) {
                        Some((mut valueuniform, mut texuniform)) => {
                            texuniform.write(TextureSlot1(imagepath.clone()));
                        },
                        None => {
                            cmds.list.push(EUnlitMaterialCommand::EmissiveTexture(entity, imagepath));
                        },
                    }
                },
            }
        });
    }
}
use pi_scene_shell::prelude::*;

use crate::materials::prelude::*;

use super::pass_object::*;

pub fn sys_modify_pass_effect_by_pass(
    materials: Query<
    (&AssetKeyShaderEffect, &AssetResShaderEffectMeta, &BindEffect, &MaterialRefs, &EffectTextureSamplersComp),
    >,
    mut passes: Query<(Entity, &mut PassEffectReady, &mut PassBindEffectValue, &mut PassBindEffectTextures, &PassMaterialID), Changed<PassMaterialID>>,
) {
    passes.iter_mut().for_each(|(idpass, mut passready, mut passbind, mut passtextures, idmat)| {
        
        // log::error!("Material {:?}", idmat.0);
        if let Ok((effect_key, meta, bind, _list, textures)) = materials.get(idmat.0) {
            let meta = meta.as_ref().unwrap();

            let (bindvalue, bindtextures, effect) = match (0 < meta.textures.len(), &textures.0) {
                (true, Some(textures)) => {
                    if textures.textures.len() == meta.textures.len() {
                        (bind.0.as_ref(), Some(textures.clone()), Some((effect_key.0.clone(), meta.clone())))
                    } else {
                        // log::error!("sys_modify_pass_effect_by_pass : 2");
                        (None, None, None)
                    }
                },
                (false, _) => (bind.0.as_ref(), None, Some((effect_key.0.clone(), meta.clone()))),
                _ => { 
                    // log::error!("sys_modify_pass_effect_by_pass : 3");
                    (None, None, None)
                }
            };

            // log::error!("Effect None : 3 {:?}", (effect_key, effect.is_some(), idmat.0, idpass));
            passready.0 = effect.clone();
            passtextures.0 = bindtextures.clone();
            match bindvalue {
                Some(bindvalue) => passbind.0 = Some(bindvalue.bind()),
                None =>  passbind.0 = None ,
            }
        } else {
            // log::error!("Material NotFound ");
        }
    });
}

pub fn sys_modify_pass_effect_by_material(
    materials: Query<
        (Entity, &AssetKeyShaderEffect, &AssetResShaderEffectMeta, &BindEffect, &MaterialRefs, &EffectTextureSamplersComp),
        Or<(Changed<PassTag>, Changed<DirtyMaterialRefs>, Changed<BindEffectReset>, Changed<EffectTextureSamplersComp>)>
    >,
    mut passes: Query<(&mut PassEffectReady, &mut PassBindEffectValue, &mut PassBindEffectTextures)>,
) {
    // log::error!("MaterialBind : ");
    materials.iter().for_each(|(idmat, effect_key, meta, bind, list, textures)| {
        let meta = meta.0.as_ref().unwrap();
        let (bindvalue, bindtextures, effect) = match (0 < meta.textures.len(), &textures.0) {
            (true, Some(textures)) => {
                if textures.textures.len() == meta.textures.len() {
                    (bind.0.as_ref(), Some(textures.clone()), Some((effect_key.0.clone(), meta.clone())))
                } else {
                    (None, None, None)
                }
            },
            (false, _) => (bind.0.as_ref(), None, Some((effect_key.0.clone(), meta.clone()))),
            _ => {
                (None, None, None)
            }
        };

        // log::error!("MaterialBind : 1  - {:?}", (bindvalue.is_some(), bindtextures.is_some(), effect.is_some()));
        list.iter().for_each(|target| {
            // log::error!("MaterialBind : 2");
            if let Ok((mut passready, mut passbind, mut passtextures)) = passes.get_mut(*target) {
                // log::error!("Effect None : 3 {:?}", (effect_key, effect.is_some(), idmat, target));
                if let (Some(old), Some(new)) = (&passready.0, &effect) {
                    if old.0 != new.0 {
                        passready.0 = effect.clone();
                    } else {
                        // log::error!("Effect No Modify");
                    }
                } else {
                    passready.0 = effect.clone();
                }

                // log::error!("MaterialBind : PassBindEffectTextures");
                passtextures.0 = bindtextures.clone();

                match bindvalue {
                    Some(bindvalue) => {
                        passbind.0 = Some(bindvalue.bind())
                    },
                    None =>  passbind.0 = None ,
                }
                
            }
        });
    });
}
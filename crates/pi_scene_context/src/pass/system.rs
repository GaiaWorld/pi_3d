use pi_engine_shell::prelude::*;

use crate::materials::prelude::*;

use super::pass_object::*;

pub fn sys_modify_pass_effect_by_pass(
    materials: Query<
    (&AssetKeyShaderEffect, &AssetResShaderEffectMeta, &BindEffect, &MaterialRefs, &EffectTextureSamplersComp),
    >,
    mut passes: Query<(&mut PassEffectReady, &mut PassBindEffectValue, &mut PassBindEffectTextures, &PassMaterialID), Changed<PassMaterialID>>,
) {
    passes.iter_mut().for_each(|(mut passready, mut passbind, mut passtextures, idmat)| {
        
        if let Ok((effect_key, meta, bind, list, textures)) = materials.get(idmat.0) {

            let (bindvalue, bindtextures, effect) = match (0 < meta.textures.len(), &textures.0) {
                (true, Some(textures)) => {
                    if textures.textures.len() == meta.textures.len() {
                        (bind.0.as_ref(), Some(textures.clone()), Some((effect_key.0.clone(), meta.0.clone())))
                    } else {
                        (None, None, None)
                    }
                },
                (false, _) => (bind.0.as_ref(), None, Some((effect_key.0.clone(), meta.0.clone()))),
                _ => { (None, None, None) }
            };

            // log::error!("MaterialBind : 3 {:?}", effect.is_some());
            passready.0 = effect.clone();
            passtextures.0 = bindtextures.clone();
            match bindvalue {
                Some(bindvalue) => passbind.0 = Some(bindvalue.bind.clone()),
                None =>  passbind.0 = None ,
            }
        }
    });
}

pub fn sys_modify_pass_effect_by_material(
    materials: Query<
        (&AssetKeyShaderEffect, &AssetResShaderEffectMeta, &BindEffect, &MaterialRefs, &EffectTextureSamplersComp),
        Or<(Changed<PassTag>, Changed<DirtyMaterialRefs>, Changed<BindEffectReset>, Changed<EffectTextureSamplersComp>)>
    >,
    mut passes: Query<(&mut PassEffectReady, &mut PassBindEffectValue, &mut PassBindEffectTextures)>,
) {
    // log::error!("MaterialBind : ");
    materials.iter().for_each(|(effect_key, meta, bind, list, textures)| {
        let (bindvalue, bindtextures, effect) = match (0 < meta.textures.len(), &textures.0) {
            (true, Some(textures)) => {
                if textures.textures.len() == meta.textures.len() {
                    (bind.0.as_ref(), Some(textures.clone()), Some((effect_key.0.clone(), meta.0.clone())))
                } else {
                    // log::error!("MaterialBind : Error 3");
                    (None, None, None)
                }
            },
            (false, _) => (bind.0.as_ref(), None, Some((effect_key.0.clone(), meta.0.clone()))),
            _ => {
                // log::error!("MaterialBind : Error 2");
                (None, None, None)
            }
        };

        // log::error!("MaterialBind : 1  - {:?}", (bindvalue.is_some(), bindtextures.is_some(), effect.is_some()));
        list.iter().for_each(|target| {
            // log::error!("MaterialBind : 2");
            if let Ok((mut passready, mut passbind, mut passtextures)) = passes.get_mut(*target) {
                // log::error!("MaterialBind : 3 {:?}", effect.is_some());
                if let (Some(old), Some(new)) = (&passready.0, &effect) {
                    if old.0 != new.0 { passready.0 = effect.clone(); }
                } else {
                    passready.0 = effect.clone();
                }

                // log::error!("MaterialBind : PassBindEffectTextures");
                passtextures.0 = bindtextures.clone();

                match bindvalue {
                    Some(bindvalue) => {
                        passbind.0 = Some(bindvalue.bind.clone())
                    },
                    None =>  passbind.0 = None ,
                }
                
            }
        });
    });
}
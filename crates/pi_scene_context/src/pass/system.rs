use pi_scene_shell::prelude::*;

use crate::materials::prelude::*;

use super::pass_object::*;

pub fn sys_modify_pass_effect_by_material(
    passaddeds: ComponentAdded<PassMaterialID>,
    passchanges: ComponentChanged<PassMaterialID>,
    changes: ComponentChanged<PassTag>,
    changes2: ComponentChanged<DirtyMaterialRefs>,
    changes3: ComponentChanged<BindEffectReset>,
    changes4: ComponentChanged<EffectTextureSamplersComp>,
    materials: Query<&MaterialRefs>,
    mut passes: Query<(Entity, &mut PassBindGroupsDirty)>,
) {
    passaddeds.iter().chain(passchanges.iter()).for_each(|entity| {
        if let Ok((idpass, mut dirty)) = passes.get_mut(*entity) {
            *dirty = PassBindGroupsDirty;
        }
    });
    changes.iter().for_each(|entity| {
        // log::error!("sys_modify_pass_effect_by_material");
        if let Ok(list) = materials.get(*entity) {
            list.iter().for_each(|target| {
                if let Ok((mut passready, mut dirty)) = passes.get_mut(*target) {
                    *dirty = PassBindGroupsDirty;
                }
            });
        }
    });
    changes2.iter().for_each(|entity| {
        // log::error!("sys_modify_pass_effect_by_material");
        if let Ok(list) = materials.get(*entity) {
            list.iter().for_each(|target| {
                if let Ok((mut passready, mut dirty)) = passes.get_mut(*target) {
                    *dirty = PassBindGroupsDirty;
                }
            });
        }
    });
    changes3.iter().for_each(|entity| {
        // log::error!("sys_modify_pass_effect_by_material");
        if let Ok(list) = materials.get(*entity) {
            list.iter().for_each(|target| {
                if let Ok((mut passready, mut dirty)) = passes.get_mut(*target) {
                    *dirty = PassBindGroupsDirty;
                }
            });
        }
    });
    changes4.iter().for_each(|entity| {
        // log::error!("sys_modify_pass_effect_by_material");
        if let Ok(list) = materials.get(*entity) {
            list.iter().for_each(|target| {
                if let Ok((mut passready, mut dirty)) = passes.get_mut(*target) {
                    *dirty = PassBindGroupsDirty;
                }
            });
        }
    });
    // materials.iter().for_each(|(idmat, effect_key, meta, bind, list, textures)| {
    //     // log::error!("sys_modify_pass_effect_by_material");
    //     list.iter().for_each(|target| {
    //         if let Ok((mut passready, mut dirty)) = passes.get_mut(*target) {
    //             *dirty = PassBindGroupsDirty;
    //         }
    //     });
    // });
    // // log::error!("MaterialBind : ");
    // materials.iter().for_each(|(idmat, effect_key, meta, bind, list, textures)| {
    //     let (bindvalue, bindtextures, effect) = _pass_effect_ready(
    //         effect_key, textures, meta, bind
    //     );

    //     // log::error!("MaterialBind : 1  - {:?}", (bindvalue.is_some(), bindtextures.is_some(), effect.is_some()));
    //     list.iter().for_each(|target| {
    //         // log::error!("MaterialBind : 2");
    //         if let Ok((mut passready, mut passbind, mut set2)) = passes.get_mut(*target) {
    //             // log::error!("Effect None : 3 {:?}", (effect_key, effect.is_some(), idmat, target));
    //             if let (Some(old), Some(new)) = (&passready.0, &effect) {
    //                 if old.0 != new.0 {
    //                     passready.0 = effect.clone();
    //                 } else {
    //                     // log::error!("Effect No Modify");
    //                 }
    //             } else {
    //                 passready.0 = effect.clone();
    //             }

    //             // log::error!("MaterialBind : PassBindEffectTextures");
    //             // passtextures.0 = bindtextures.clone();
    //             if let (Some(effect_texture_samplers), Some((_key_meta, meta))) = (&bindtextures, &passready.0) {
    //                 let result = _set2_modify(
    //                     _key_meta,
    //                     meta,
    //                     effect_texture_samplers,
    //                     &device,
    //                     &asset_mgr_bindgroup_layout,
    //                     &asset_mgr_bindgroup,
    //                 );
    //                 *set2 = PassBindGroupTextureSamplers(result);
    //             } else {
    //                 *set2 = PassBindGroupTextureSamplers(None);
    //             }

    //             match bindvalue {
    //                 Some(bindvalue) => {
    //                     passbind.0 = Some(bindvalue.bind())
    //                 },
    //                 None =>  passbind.0 = None ,
    //             }
                
    //         }
    //     });
    // });
}

pub fn _pass_effect_ready<'a>(
    effect_key: &'a AssetKeyShaderEffect,
    textures: &'a EffectTextureSamplersComp,
    meta: &'a AssetResShaderEffectMeta,
    bind: &'a BindEffect,
) -> (Option<&'a BindEffectValues>, Option<&'a EffectTextureSamplers>, Option<(Atom, Handle<ShaderEffectMeta>)>) {
    let meta = meta.0.as_ref().unwrap();
    match (0 < meta.textures.len(), &textures.0) {
        (true, Some(textures)) => {
            if textures.textures.len() == meta.textures.len() {
                (bind.0.as_ref(), Some(textures), Some((effect_key.0.clone(), meta.clone())))
            } else {
                (None, None, None)
            }
        },
        (false, _) => (bind.0.as_ref(), None, Some((effect_key.0.clone(), meta.clone()))),
        _ => {
            (None, None, None)
        }
    }
}
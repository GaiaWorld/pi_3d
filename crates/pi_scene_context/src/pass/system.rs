use pi_scene_shell::prelude::*;

use crate::materials::prelude::*;

use super::pass_object::*;

pub fn sys_modify_pass_effect_by_pass(
    // materials: Query<
    // (&AssetKeyShaderEffect, &AssetResShaderEffectMeta, &BindEffect, &MaterialRefs, &EffectTextureSamplersComp),
    // >,
    mut passes: Query<(Entity, &mut PassBindGroupsDirty), Changed<PassMaterialID>>,
    // mut passes: Query<(Entity, &mut PassEffectReady, &mut PassBindEffectValue, &mut PassBindGroupTextureSamplers, &PassMaterialID), Changed<PassMaterialID>>,
    // device: Res<PiRenderDevice>,
    // asset_mgr_bindgroup_layout: Res<ShareAssetMgr<BindGroupLayout>>,
    // asset_mgr_bindgroup: Res<ShareAssetMgr<BindGroup>>,
) {
    passes.iter_mut().for_each(|(idpass, mut dirty)| {
        *dirty = PassBindGroupsDirty;
    });
    // passes.iter_mut().for_each(|(idpass, mut passready, mut passbind, mut set2, idmat)| {
        
    //     // log::error!("Material {:?}", idmat.0);
    //     if let Ok((effect_key, meta, bind, _list, textures)) = materials.get(idmat.0) {
    //         let (bindvalue, bindtextures, effect) = _pass_effect_ready(
    //             effect_key, textures, meta, bind
    //         );
    //         // log::error!("Effect None : 3 {:?}", (effect_key, effect.is_some(), idmat.0, idpass));
    //         passready.0 = effect.clone();
    //         // passtextures.0 = bindtextures.clone();
    //         if let (Some(effect_texture_samplers), Some((_key_meta, meta))) = (&bindtextures, &passready.0) {
    //             let result = _set2_modify(
    //                 _key_meta,
    //                 meta,
    //                 effect_texture_samplers,
    //                 &device,
    //                 &asset_mgr_bindgroup_layout,
    //                 &asset_mgr_bindgroup,
    //             );
    //             *set2 = PassBindGroupTextureSamplers(result);
    //         } else {
    //             *set2 = PassBindGroupTextureSamplers(None);
    //         }
    //         match bindvalue {
    //             Some(bindvalue) => passbind.0 = Some(bindvalue.bind()),
    //             None =>  passbind.0 = None ,
    //         }
    //     } else {
    //         // log::error!("Material NotFound ");
    //     }
    // });
}

pub fn sys_modify_pass_effect_by_material(
    materials: Query<
        (Entity, &AssetKeyShaderEffect, &AssetResShaderEffectMeta, &BindEffect, &MaterialRefs, &EffectTextureSamplersComp),
        Or<(Changed<PassTag>, Changed<DirtyMaterialRefs>, Changed<BindEffectReset>, Changed<EffectTextureSamplersComp>)>
    >,
    mut passes: Query<(Entity, &mut PassBindGroupsDirty)>,
    // mut passes: Query<(&mut PassEffectReady, &mut PassBindEffectValue, &mut PassBindGroupTextureSamplers)>,
    // device: Res<PiRenderDevice>,
    // asset_mgr_bindgroup_layout: Res<ShareAssetMgr<BindGroupLayout>>,
    // asset_mgr_bindgroup: Res<ShareAssetMgr<BindGroup>>,
) {
    materials.iter().for_each(|(idmat, effect_key, meta, bind, list, textures)| {
        // log::error!("sys_modify_pass_effect_by_material");
        list.iter().for_each(|target| {
            if let Ok((mut passready, mut dirty)) = passes.get_mut(*target) {
                *dirty = PassBindGroupsDirty;
            }
        });
    });
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
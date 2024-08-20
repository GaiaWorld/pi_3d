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
    mut passes: Query<&mut PassBindGroupsDirty>,
) {
    passaddeds.iter().chain(passchanges.iter()).for_each(|entity| {
        if let Ok(mut dirty) = passes.get_mut(*entity) {
            *dirty = PassBindGroupsDirty;
        }
    });
    changes.iter().for_each(|entity| {
        // log::error!("sys_modify_pass_effect_by_material");
        if let Ok(list) = materials.get(*entity) {
            list.iter().for_each(|target| {
                if let Ok(mut dirty) = passes.get_mut(*target) {
                    *dirty = PassBindGroupsDirty;
                }
            });
        }
    });
    changes2.iter().for_each(|entity| {
        // log::error!("sys_modify_pass_effect_by_material");
        if let Ok(list) = materials.get(*entity) {
            list.iter().for_each(|target| {
                if let Ok(mut dirty) = passes.get_mut(*target) {
                    *dirty = PassBindGroupsDirty;
                }
            });
        }
    });
    changes3.iter().for_each(|entity| {
        // log::error!("sys_modify_pass_effect_by_material");
        if let Ok(list) = materials.get(*entity) {
            list.iter().for_each(|target| {
                if let Ok(mut dirty) = passes.get_mut(*target) {
                    *dirty = PassBindGroupsDirty;
                }
            });
        }
    });
    changes4.iter().for_each(|entity| {
        // log::error!("sys_modify_pass_effect_by_material");
        if let Ok(list) = materials.get(*entity) {
            list.iter().for_each(|target| {
                if let Ok(mut dirty) = passes.get_mut(*target) {
                    *dirty = PassBindGroupsDirty;
                }
            });
        }
    });
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
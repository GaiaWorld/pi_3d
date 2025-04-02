use pi_scene_shell::prelude::*;

use crate::materials::prelude::*;

use super::pass_object::*;

pub fn sys_modify_pass_effect_by_material(
    passaddeds: ComponentAdded<PassMaterialID>,
    passchanges: ComponentChanged<PassMaterialID>,
    changes: ComponentChanged<PassTag>,
    changes2: ComponentChanged<DirtyMaterialRefs>,
    changes3: ComponentChanged<BindEffect>,
    changes4: ComponentChanged<EffectTextureSamplersComp>,
    materials: Query<(&MaterialRefs, &DirtyMaterialRefs)>,
    mut passes: Query<&mut PassBindGroupsDirty>,
    // mut performance: ResMut<Performance>,
    entitysets: Res<EntityFilterForComponentChanged>,
) {

    let mut entities = entitysets.pop();
    // performance.systems.push(String::from("sys_modify_pass_effect_by_material"));
    passaddeds.iter().chain(passchanges.iter()).for_each(|entity| {
        if !entities.insert(&entity) { return; }
        if let Ok(mut dirty) = passes.get_mut(*entity) {
            *dirty = PassBindGroupsDirty;
        }
    });
    changes.iter().for_each(|entity| {
        // log::error!("sys_modify_pass_effect_by_material");
        if let Ok((list, _dirty)) = materials.get(*entity) {
            list.iter().for_each(|target| {
                let target = if let Some(target) = target { target } else { return; };
                if !entities.insert(target) { return; }
                if let Ok(mut dirty) = passes.get_mut(*target) {
                    *dirty = PassBindGroupsDirty;
                }
            });
        }
    });
    changes2.iter().for_each(|entity| {
        // log::error!("sys_modify_pass_effect_by_material");
        if let Ok((_list, dirty)) = materials.get(*entity) {
            while let Some(target) = dirty.0.pop() {
                if !entities.insert(&target) { return; }
                if let Ok(mut dirty) = passes.get_mut(target) {
                    *dirty = PassBindGroupsDirty;
                }
            }
        }
    });
    changes3.iter().for_each(|entity| {
        // log::error!("sys_modify_pass_effect_by_material");
        if let Ok((list, _dirty)) = materials.get(*entity) {
            list.iter().for_each(|target| {
                let target = if let Some(target) = target { target } else { return; };
                if !entities.insert(target) { return; }
                if let Ok(mut dirty) = passes.get_mut(*target) {
                    *dirty = PassBindGroupsDirty;
                }
            });
        }
    });
    changes4.iter().for_each(|entity| {
        // log::error!("sys_modify_pass_effect_by_material");
        if let Ok((list, dirty)) = materials.get(*entity) {
            list.iter().for_each(|target| {
                let target = if let Some(target) = target { target } else { return; };
                if !entities.insert(target) { return; }
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
    texturekeys: &'a TextureKeyList,
    meta: &'a AssetResShaderEffectMeta,
    bind: &'a BindEffect,
) -> (Option<&'a BindEffectValues>, Option<&'a EffectTextureSamplers>, Option<(Atom, Handle<ShaderEffectMeta>)>) {
    let meta = meta.0.as_ref().unwrap();
    match (0 < meta.textures.len(), &textures.0) {
        (true, Some(textures)) => {
            if textures.textures.len() == meta.textures.len() {
                let len = textures.textures.len();
                if let Some(bindval) = bind.0.as_ref() {
                    for texidx in 0..len {
                        let tex = &textures.textures[texidx];
                        let desc = &texturekeys.0[texidx];
                        bindval.update_texture(texidx, &tex.tilloff(), desc.wrapu, desc.wrapv, desc.wrapw, tex.coord());
                    }
                }
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
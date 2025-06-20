use pi_scene_shell::prelude::*;

use crate::materials::prelude::*;

use super::pass_object::*;

pub fn sys_modify_pass_effect_by_material(
    passaddeds: ComponentAdded<PassMaterialID>,
    passchanges: ComponentChanged<PassMaterialID>,
    changes: ComponentChanged<PassTag>,
    changes2: ComponentChanged<DirtyMaterialRefs>,
    changes3: ComponentAdded<BindEffect>,
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
        // log::error!("PassBindGroupsDirty: PassMaterialID");
        if let Ok(mut dirty) = passes.get_mut(*entity) {
            *dirty = PassBindGroupsDirty;
        }
    });
    changes.iter().for_each(|entity| {
        // log::error!("PassBindGroupsDirty: PassTag");
        if let Ok((list, _dirty)) = materials.get(*entity) {
            list.iter().for_each(|target| {
                if !entities.insert(target) { return; }
                if let Ok(mut dirty) = passes.get_mut(*target) {
                    *dirty = PassBindGroupsDirty;
                }
            });
        }
    });
    changes2.iter().for_each(|entity| {
        // log::error!("PassBindGroupsDirty: DirtyMaterialRefs");
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
        // log::error!("PassBindGroupsDirty: BindEffect");
        if let Ok((list, _dirty)) = materials.get(*entity) {
            list.iter().for_each(|target| {
                if !entities.insert(target) { return; }
                if let Ok(mut dirty) = passes.get_mut(*target) {
                    *dirty = PassBindGroupsDirty;
                }
            });
        }
    });
    changes4.iter().for_each(|entity| {
        if let Ok((list, dirtys)) = materials.get(*entity) {
            list.iter().for_each(|target| {
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
    meta: &'a AssetResShaderEffectMeta,
) -> (Option<&'a EffectTextureSamplers>, Option<(Atom, Handle<ShaderEffectMeta>)>) {
    let meta = meta.0.as_ref().unwrap();
    match (0 < meta.textures.len(), &textures.0) {
        (true, Some(textures)) => {
            if textures.textures.len() == meta.textures.len() {
                (Some(textures), Some((effect_key.0.clone(), meta.clone())))
            } else {
                // log::error!("textures not ready");
                (None, None)
            }
        },
        (false, _) => ( None, Some((effect_key.0.clone(), meta.clone()))),
        _ => {
            // log::error!("texturesamplers not ready");
            (None, None)
        }
    }
}
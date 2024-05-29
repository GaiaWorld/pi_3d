
use pi_scene_shell::{add_component, prelude::{pi_world::editor::EntityEditor, *}};
use pi_scene_math::{Vector3, coordiante_system::CoordinateSytem3, vector::TToolVector3};

use crate::{
    scene::prelude::*,
    transforms::prelude::*,
    layer_mask::prelude::*,
    flags::*,
};

use super::{spot::SpotLightAngle, hemisphere::HemiGrounds, base::*};

pub fn sys_light_index_create(
    mut editor: EntityEditor,
    items: Query<(Entity, &SceneID, Option<&DirectLight>, Option<&PointLight>, Option<&SpotLight>, Option<&HemisphericLight>), (Changed<DirectLight>, Changed<PointLight>, Changed<SpotLight>, Changed<HemisphericLight>)>,
    mut scenes: Query<(&mut SceneDirectLightsQueue, &mut ScenePointLightsQueue, &mut SceneSpotLightsQueue, &mut SceneHemiLightsQueue, &mut SceneLightingInfosDirty)>,
) {

    items.iter().for_each(|(entity, idscene, direct, point, spot, hemi)| {
        log::warn!("sys_light_index_create 2");
        if let Ok((mut queuedirect, mut queuepoint, mut queuespot, mut queuehemi, mut dirty)) = scenes.get_mut(idscene.0) {

            if direct.is_some() {
                // editor.alter(entity, (queuedirect.0.add(entity),));
                let _ = add_component(&mut editor, entity, queuedirect.0.add(entity));
                *dirty = SceneLightingInfosDirty;
            }
            if point.is_some() {
                // log::warn!("Add Point !!");
                // commands.entity(entity).insert(queuepoint.0.add(entity));
                // editor.alter(entity, (queuedirect.0.add(entity),));
                let _ = add_component(&mut editor, entity, queuepoint.0.add(entity));
                *dirty = SceneLightingInfosDirty;
            }
            if spot.is_some() {
                // commands.entity(entity).insert(queuespot.0.add(entity));
                // editor.alter(entity, (queuedirect.0.add(entity),));
                let _ = add_component(&mut editor, entity, queuespot.0.add(entity));
                *dirty = SceneLightingInfosDirty;
            }
            if hemi.is_some() {
                // commands.entity(entity).insert(queuehemi.0.add(entity));
                // editor.alter(entity, (queuedirect.0.add(entity),));
                let _ = add_component(&mut editor, entity, queuehemi.0.add(entity));
                *dirty = SceneLightingInfosDirty;
            }
        }
    });
}

pub fn sys_direct_light_update(
    items: Query<
        (&DirectLight, &SceneID, &SceneItemIndex, &LightDirection, &LightParam, &LayerMask, &GlobalEnable, &GlobalMatrix),
        (Changed<LightParam>, Changed<LightDirection>, Changed<LayerMask>, Changed<GlobalEnable>, Changed<GlobalMatrix>)
    >,
    scenes: Query<&SceneLightingInfos>,
) {
    items.iter().for_each(|(_, idscene, lidx, direction, param, layer, enabled, wm)| {
        if let Ok(info) = scenes.get(idscene.0) {
            let mut gdirection = Vector3::zeros();
            CoordinateSytem3::transform_normal(&direction.0, &wm.matrix, &mut gdirection);
            let r = param.color.x * param.strength; let g = param.color.y * param.strength; let b = param.color.z * param.strength;
            info.0.as_ref().unwrap().direct_light_data(lidx.val(), enabled.0, layer.0 as f32, gdirection.x, gdirection.y, gdirection.z, r, g, b)
        }
    });
}

pub fn sys_point_light_update(
    items: Query<
        (&PointLight, &SceneID, &SceneItemIndex, &LightParam, &GlobalMatrix, &LayerMask, &GlobalEnable),
        (Changed<LightParam>, Changed<LayerMask>, Changed<GlobalMatrix>, Changed<GlobalEnable>)
    >,
    scenes: Query<&SceneLightingInfos>,
) {
    items.iter().for_each(|(_, idscene, lidx, param, transform, layer, enabled)| {
        if let Ok(info) = scenes.get(idscene.0) {
            let pos = transform.position();
            let r = param.color.x * param.strength; let g = param.color.y * param.strength; let b = param.color.z * param.strength;
            info.0.as_ref().unwrap().point_light_data(lidx.val(), enabled.0, layer.0 as f32, pos.x, pos.y, pos.z, r, g, b, param.radius, 1.0 / (param.radius * param.radius))
        }
    });
}

pub fn sys_spot_light_update(
    items: Query<
        (&SpotLight, &SceneID, &SceneItemIndex, &LightDirection, &LightParam, &SpotLightAngle, &GlobalMatrix, &LayerMask, &GlobalEnable),
        (Changed<LightDirection>, Changed<LightParam>, Changed<SpotLightAngle>, Changed<LayerMask>, Changed<GlobalMatrix>, Changed<GlobalEnable>)
    >,
    scenes: Query<&SceneLightingInfos>,
) {
    items.iter().for_each(|(_, idscene, lidx, d, param, angle, transform, layer, enabled)| {
        if let Ok(info) = scenes.get(idscene.0) {
            let pos = transform.position();
            let r = param.color.x * param.strength; let g = param.color.y * param.strength; let b = param.color.z * param.strength;
            info.0.as_ref().unwrap().spot_light_data(lidx.val(), enabled.0, layer.0 as f32, pos.x, pos.y, pos.z, r, g, b, param.radius, 1.0 / (param.radius * param.radius), angle.in_value, angle.out_value, d.0.x, d.0.y, d.0.z);
        }
    });
}

pub fn sys_hemi_light_update(
    items: Query<
        (&HemiGrounds, &SceneID, &SceneItemIndex, &LightParam, &GlobalMatrix, &LayerMask, &GlobalEnable),
        (Changed<LightParam>, Changed<LayerMask>, Changed<GlobalMatrix>, Changed<GlobalEnable>)
    >,
    scenes: Query<&SceneLightingInfos>,
) {
    items.iter().for_each(|(_hemi, idscene, lidx, color, transform, layer, enabled)| {
        if let Ok(info) = scenes.get(idscene.0) {
            let pos = transform.position();
            info.0.as_ref().unwrap().hemi_light_data(lidx.val(), enabled.0, layer.0 as f32, pos.x, pos.y, pos.z, color.color.x, color.color.y, color.color.z, 1., 1., 0., 0., 0., 0.);
        }
    });
}

pub fn sys_dispose_about_light(
    items: Query<(Entity, &DisposeReady, &SceneID, &SceneItemIndex), (Changed<DisposeReady>, With<LightStrength>)>,
    mut disposecanlist: ResMut<ActionListDisposeCan>,
    mut scenes: Query<(&mut SceneDirectLightsQueue, &mut ScenePointLightsQueue, &mut SceneSpotLightsQueue, &mut SceneHemiLightsQueue)>,
    _empty: Res<SingleEmptyEntity>,
) {
    items.iter().for_each(|(entity, state, idscene, lightindex)| {
        if state.0 == false { return; }

        if let Ok((mut queuedirect, mut queuepoint, mut queuespot, mut queuehemi)) = scenes.get_mut(idscene.0) {
            queuedirect.0.recycle(lightindex, &entity);
            queuepoint.0.recycle(lightindex, &entity);
            queuespot.0.recycle(lightindex, &entity);
            queuehemi.0.recycle(lightindex, &entity);
        }

        disposecanlist.push(OpsDisposeCan::ops(entity));
    });
}

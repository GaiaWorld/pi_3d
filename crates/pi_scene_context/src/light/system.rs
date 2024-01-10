
use bevy::prelude::{Query, Changed, ResMut};
use pi_scene_shell::prelude::*;
use pi_scene_math::{Vector3, coordiante_system::CoordinateSytem3, vector::TToolVector3};

use crate::{
    scene::prelude::*,
    transforms::prelude::*,
    layer_mask::prelude::*,
    flags::*,
};

use super::{spot::{SpotLightInAngle, SpotLightOutAngle}, hemisphere::HemiGrounds, base::*};

pub fn sys_light_index_create(
    mut commands: Commands,
    items: Query<(Entity, &SceneID, Option<&DirectLight>, Option<&PointLight>, Option<&SpotLight>, Option<&HemisphericLight>), Or<(Added<DirectLight>, Added<PointLight>, Added<SpotLight>, Added<HemisphericLight>)>>,
    mut scenes: Query<(&mut SceneDirectLightsQueue, &mut ScenePointLightsQueue, &mut SceneSpotLightsQueue, &mut SceneHemiLightsQueue, &mut SceneLightingInfosDirty)>,
) {

    items.iter().for_each(|(entity, idscene, direct, point, spot, hemi)| {

        if let Ok((mut queuedirect, mut queuepoint, mut queuespot, mut queuehemi, mut dirty)) = scenes.get_mut(idscene.0) {

            if direct.is_some() {
                commands.entity(entity).insert(queuedirect.0.add(entity));
                *dirty = SceneLightingInfosDirty;
            }
            if point.is_some() {
                // log::warn!("Add Point !!");
                commands.entity(entity).insert(queuepoint.0.add(entity));
                *dirty = SceneLightingInfosDirty;
            }
            if spot.is_some() {
                commands.entity(entity).insert(queuespot.0.add(entity));
                *dirty = SceneLightingInfosDirty;
            }
            if hemi.is_some() {
                commands.entity(entity).insert(queuehemi.0.add(entity));
                *dirty = SceneLightingInfosDirty;
            }
        }
    });
}

pub fn sys_direct_light_update(
    items: Query<
        (&DirectLight, &SceneID, &SceneItemIndex, &LightDirection, &LightColor, &LightStrength, &LayerMask, &GlobalEnable, &GlobalMatrix),
        Or<(Changed<LightColor>, Changed<LightStrength>, Changed<LightDirection>, Changed<LayerMask>, Changed<GlobalEnable>, Changed<GlobalMatrix>)>
    >,
    scenes: Query<&SceneLightingInfos>,
) {
    items.iter().for_each(|(_, idscene, lidx, direction, color, strength, layer, enabled, wm)| {
        if let Ok(info) = scenes.get(idscene.0) {
            let mut gdirection = Vector3::zeros();
            CoordinateSytem3::transform_normal(&direction.0, &wm.matrix, &mut gdirection);
            let r = color.0.x * strength.0; let g = color.0.y * strength.0; let b = color.0.z * strength.0;
            info.0.direct_light_data(lidx.val(), enabled.0, layer.0 as f32, gdirection.x, gdirection.y, gdirection.z, r, g, b)
        }
    });
}

pub fn sys_point_light_update(
    items: Query<
        (&PointLight, &SceneID, &SceneItemIndex, &LightRadius, &LightColor, &LightStrength, &GlobalMatrix, &LayerMask, &GlobalEnable),
        Or<(Changed<LightColor>, Changed<LightStrength>, Changed<LightRadius>, Changed<LayerMask>, Changed<GlobalMatrix>, Changed<GlobalEnable>)>
    >,
    scenes: Query<&SceneLightingInfos>,
) {
    items.iter().for_each(|(_, idscene, lidx, range, color, strength, transform, layer, enabled)| {
        if let Ok(info) = scenes.get(idscene.0) {
            let pos = transform.position();
            let r = color.0.x * strength.0; let g = color.0.y * strength.0; let b = color.0.z * strength.0;
            info.0.point_light_data(lidx.val(), enabled.0, layer.0 as f32, pos.x, pos.y, pos.z, r, g, b, range.0, 1.0 / (range.0 * range.0))
        }
    });
}

pub fn sys_spot_light_update(
    items: Query<
        (&SpotLight, &SceneID, &SceneItemIndex, &LightDirection, &LightColor, &LightStrength, &LightRadius, &SpotLightInAngle, &SpotLightOutAngle, &GlobalMatrix, &LayerMask, &GlobalEnable),
        Or<(Changed<LightDirection>, Changed<LightColor>, Changed<LightStrength>, Changed<LightRadius>, Changed<SpotLightInAngle>, Changed<SpotLightOutAngle>, Changed<LayerMask>, Changed<GlobalMatrix>, Changed<GlobalEnable>)>
    >,
    scenes: Query<&SceneLightingInfos>,
) {
    items.iter().for_each(|(_, idscene, lidx, d, color, strength, range, inangel, outangle, transform, layer, enabled)| {
        if let Ok(info) = scenes.get(idscene.0) {
            let pos = transform.position();
            let r = color.0.x * strength.0; let g = color.0.y * strength.0; let b = color.0.z * strength.0;
            info.0.spot_light_data(lidx.val(), enabled.0, layer.0 as f32, pos.x, pos.y, pos.z, r, g, b, range.0, 1.0 / (range.0 * range.0), inangel.0, outangle.0, d.0.x, d.0.y, d.0.z);
        }
    });
}

pub fn sys_hemi_light_update(
    items: Query<
        (&HemiGrounds, &SceneID, &SceneItemIndex, &LightColor, &GlobalMatrix, &LayerMask, &GlobalEnable),
        Or<(Changed<LightColor>, Changed<LayerMask>, Changed<GlobalMatrix>, Changed<GlobalEnable>)>
    >,
    scenes: Query<&SceneLightingInfos>,
) {
    items.iter().for_each(|(_hemi, idscene, lidx, color, transform, layer, enabled)| {
        if let Ok(info) = scenes.get(idscene.0) {
            let pos = transform.position();
            info.0.hemi_light_data(lidx.val(), enabled.0, layer.0 as f32, pos.x, pos.y, pos.z, color.0.x, color.0.y, color.0.z, 1., 1., 0., 0., 0., 0.);
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


use pi_scene_shell::prelude::*;
use pi_scene_math::{Vector3, coordiante_system::CoordinateSytem3, vector::TToolVector3};

use crate::{
    scene::prelude::*,
    transforms::prelude::*,
    layer_mask::prelude::*,
    flags::*,
};

use super::{spot::SpotLightAngle, hemisphere::HemiGrounds, base::*};

pub fn sys_light_update(
    items: Query<
        (&DirectLight, &SceneID, &SceneItemIndex, &LightParam, &GlobalMatrix, &LayerMask, &GlobalEnable, &LightDirection),
        Or<(Changed<LightParam>, Changed<LightDirection>, Changed<LayerMask>, Changed<GlobalEnable>, Changed<GlobalMatrix>)>
    >,
    pointitems: Query<
        (&PointLight, &SceneID, &SceneItemIndex, &LightParam, &GlobalMatrix, &LayerMask, &GlobalEnable),
        Or<(Changed<LightParam>, Changed<LayerMask>, Changed<GlobalMatrix>, Changed<GlobalEnable>)>
    >,
    spotitems: Query<
        (&SpotLight, &SceneID, &SceneItemIndex, &LightParam, &GlobalMatrix, &LayerMask, &GlobalEnable, &LightDirection, &SpotLightAngle),
        Or<(Changed<LightDirection>, Changed<LightParam>, Changed<SpotLightAngle>, Changed<LayerMask>, Changed<GlobalMatrix>, Changed<GlobalEnable>)>
    >,
    hemiitems: Query<
        (&HemiGrounds, &SceneID, &SceneItemIndex, &LightParam, &GlobalMatrix, &LayerMask, &GlobalEnable),
        Or<(Changed<LightParam>, Changed<LayerMask>, Changed<GlobalMatrix>, Changed<GlobalEnable>)>
    >,
    scenes: Query<&SceneLightingInfos>,
) {
    items.iter().for_each(|(_, idscene, lidx, param, wm, layer, enabled, direction)| {
        if let Ok(info) = scenes.get(idscene.0) {
            let mut gdirection = Vector3::zeros();
            CoordinateSytem3::transform_normal_floats(direction.0.x, direction.0.y, direction.0.z, &wm.matrix, &mut gdirection);
            let r = param.color.x * param.strength; let g = param.color.y * param.strength; let b = param.color.z * param.strength;
            info.0.as_ref().unwrap().direct_light_data(lidx.val() as u16, enabled.0, layer.0 as f32, gdirection.x, gdirection.y, gdirection.z, r, g, b)
        }
    });
    pointitems.iter().for_each(|(_, idscene, lidx, param, transform, layer, enabled)| {
        if let Ok(info) = scenes.get(idscene.0) {
            let pos = transform.position();
            let r = param.color.x * param.strength; let g = param.color.y * param.strength; let b = param.color.z * param.strength;
            info.0.as_ref().unwrap().point_light_data(lidx.val() as u16, enabled.0, layer.0 as f32, pos.x, pos.y, pos.z, r, g, b, param.radius, 1.0 / (param.radius * param.radius))
        }
    });
    spotitems.iter().for_each(|(_, idscene, lidx, param, transform, layer, enabled, d, angle)| {
        if let Ok(info) = scenes.get(idscene.0) {
            let pos = transform.position();
            let r = param.color.x * param.strength; let g = param.color.y * param.strength; let b = param.color.z * param.strength;
            info.0.as_ref().unwrap().spot_light_data(lidx.val() as u16, enabled.0, layer.0 as f32, pos.x, pos.y, pos.z, r, g, b, param.radius, 1.0 / (param.radius * param.radius), angle.in_value, angle.out_value, d.0.x, d.0.y, d.0.z);
        }
    });
    hemiitems.iter().for_each(|(_hemi, idscene, lidx, color, transform, layer, enabled)| {
        if let Ok(info) = scenes.get(idscene.0) {
            let pos = transform.position();
            info.0.as_ref().unwrap().hemi_light_data(lidx.val() as u16, enabled.0, layer.0 as f32, pos.x, pos.y, pos.z, color.color.x, color.color.y, color.color.z, 1., 1., 0., 0., 0., 0.);
        }
    });
}

pub fn sys_dispose_about_light(
    items: Query<(Entity, &DisposeReady, &SceneID, &SceneItemIndex, &LightParam), Changed<DisposeReady>>,
    mut disposecanlist: ResMut<ActionListDisposeCan>,
    mut scenes: Query<(&mut SceneDirectLightsQueue, &mut SceneOtherLightsQueue)>,
    _empty: Res<SingleEmptyEntity>,
) {
    items.iter().for_each(|(entity, state, idscene, lightindex, _)| {
        if state.0 == false { return; }

        if let Ok((mut queuedirect, mut queuepoint)) = scenes.get_mut(idscene.0) {
            queuedirect.0.recycle(lightindex, &entity);
            queuepoint.point.recycle(lightindex, &entity);
            queuepoint.spot.recycle(lightindex, &entity);
            queuepoint.hemi.recycle(lightindex, &entity);
        }

        disposecanlist.push(OpsDisposeCan::ops(entity));
    });
}

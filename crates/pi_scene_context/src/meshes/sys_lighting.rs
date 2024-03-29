use pi_scene_shell::prelude::*;

use crate::{
    light::prelude::*,
    layer_mask::prelude::*,
    transforms::prelude::*,
    scene::prelude::*,
    viewer::prelude::*,
    flags::*,
};

use super::model::*;


pub fn sys_model_direct_lighting_modify_by_light(
    scenes: Query<(Entity, &SceneDirectLightsQueue), Changed<SceneDirectLightsQueue>>,
    layermask: Query<&LayerMask>,
    viewers: Query<(&SceneID, &ModelList)>,
    lightindex: Query<&SceneItemIndex>,
    meshes: Query<&ModelLightingIndexs>,
    // mut record: ResMut<pi_scene_shell::run_stage::RunSystemRecord>,
) {
    // record.0.push(String::from("sys_model_direct_lighting_modify_by_light"));
    scenes.iter().for_each(|(scene, queuedirect)| {
        viewers.iter().for_each(|(idscene, models)| {
            if idscene.0 == scene {
                models.0.iter().for_each(|idm| {
                    if let (Ok(ids), Ok(my)) = (meshes.get(*idm), layermask.get(*idm)) {
                        if let Some(ids) = &ids.bind {
                            let mut indexlight = vec![];
                            queuedirect.0.items().for_each(|idlight| {
                                if let (Ok(ly), Ok(lidx)) = (layermask.get(*idlight), lightindex.get(*idlight)) {
                                    if ly.include(my.0) {
                                        indexlight.push(lidx.val());
                                    }
                                }
                            });
                            ids.direct_light_data(&indexlight);
                        }
                    }
                });
            }
        });
    });
}

pub fn sys_model_direct_lighting_modify_by_model(
    scenes: Query<&SceneDirectLightsQueue>,
    transforms: Query<&GlobalMatrix>,
    enabled: Query<&GlobalEnable, With<DirectLight>>,
    layermask: Query<&LayerMask>,
    lightindex: Query<&SceneItemIndex>,
    meshes: Query<(Entity, &SceneID, &ModelLightingIndexs), Changed<LayerMask>>,
    // mut record: ResMut<pi_scene_shell::run_stage::RunSystemRecord>,
) {
    // record.0.push(String::from("sys_model_direct_lighting_modify_by_model"));
    meshes.iter().for_each(|(idm, idscene, ids)| {
        if let Ok(queuedirect) = scenes.get(idscene.0) {
            if let Ok(my) = layermask.get(idm) {
                if let Some(ids) = &ids.bind {
                    let mut indexlight = vec![];
                    queuedirect.0.items().for_each(|idlight| {
                        if let (Ok(_lp), Ok(ly), Ok(lidx), Ok(enabled)) = (transforms.get(*idlight), layermask.get(*idlight), lightindex.get(*idlight), enabled.get(*idlight)) {
                            if enabled.0 && ly.include(my.0) {
                                indexlight.push(lidx.val());
                            }
                        }
                    });
                    ids.direct_light_data(&indexlight);
                }
            }
        }
    });
}

pub fn sys_model_point_lighting_modify_by_model(
    scenes: Query<&ScenePointLightsQueue>,
    transforms: Query<&GlobalMatrix>,
    enabled: Query<&GlobalEnable, With<PointLight>>,
    layermask: Query<&LayerMask>,
    lightindex: Query<&SceneItemIndex>,
    meshes: Query<(Entity, &SceneID, &ModelLightingIndexs, &ModelForcePointLightings), Or<(Changed<LayerMask>, Changed<ModelForcePointLightings>)>>,
    // mut record: ResMut<pi_scene_shell::run_stage::RunSystemRecord>,
) {
    // record.0.push(String::from("sys_model_point_lighting_modify_by_model"));
    meshes.iter().for_each(|(idm, idscene, ids, forcelights)| {
        if let Ok(queuepoint) = scenes.get(idscene.0) {
            if let Ok(my) = layermask.get(idm) {
                if let Some(ids) = &ids.bind {

                    let mut indexlight = vec![];

                    forcelights.0.iter().for_each(|idlight| {
                        if let (Ok(_lp), Ok(_ly), Ok(lidx), Ok(enable)) = (transforms.get(*idlight), layermask.get(*idlight), lightindex.get(*idlight), enabled.get(*idlight)) {
                            if enable.0 {
                                let idx = lidx.val();
                                if indexlight.contains(&idx) == false { indexlight.push(idx); }
                            }
                        }
                    });
                    queuepoint.0.items().for_each(|idlight| {
                        if let (Ok(_lp), Ok(ly), Ok(lidx), Ok(enabled)) = (transforms.get(*idlight), layermask.get(*idlight), lightindex.get(*idlight), enabled.get(*idlight)) {
                            if enabled.0 && ly.include(my.0) {
                                let idx = lidx.val();
                                if indexlight.contains(&idx) == false { indexlight.push(idx); }
                            }
                        }
                    });
                    ids.point_light_data(&indexlight);
                }
            }
        }
    });
}

pub fn sys_model_spot_lighting_modify_by_model(
    scenes: Query<&SceneSpotLightsQueue>,
    transforms: Query<&GlobalMatrix>,
    enabled: Query<&GlobalEnable, With<SpotLight>>,
    layermask: Query<&LayerMask>,
    lightindex: Query<&SceneItemIndex>,
    meshes: Query<(Entity, &SceneID, &ModelLightingIndexs, &ModelForceSpotLightings), Or<(Changed<LayerMask>, Changed<ModelForceSpotLightings>)>>,
    // mut record: ResMut<pi_scene_shell::run_stage::RunSystemRecord>,
) {
    // record.0.push(String::from("sys_model_spot_lighting_modify_by_model"));
    meshes.iter().for_each(|(idm, idscene, ids, forcelights)| {
        if let Ok(queuepoint) = scenes.get(idscene.0) {
            if let Ok(my) = layermask.get(idm) {
                if let Some(ids) = &ids.bind {

                    let mut indexlight = vec![];

                    forcelights.0.iter().for_each(|idlight| {
                        if let (Ok(_lp), Ok(_ly), Ok(lidx), Ok(enable)) = (transforms.get(*idlight), layermask.get(*idlight), lightindex.get(*idlight), enabled.get(*idlight)) {
                            if enable.0 {
                                let idx = lidx.val();
                                if indexlight.contains(&idx) == false { indexlight.push(idx); }
                            }
                        }
                    });
                    queuepoint.0.items().for_each(|idlight| {
                        if let (Ok(_lp), Ok(ly), Ok(lidx), Ok(enabled)) = (transforms.get(*idlight), layermask.get(*idlight), lightindex.get(*idlight), enabled.get(*idlight)) {
                            if enabled.0 && ly.include(my.0) {
                                let idx = lidx.val();
                                if indexlight.contains(&idx) == false { indexlight.push(idx); }
                            }
                        }
                    });
                    ids.spot_light_data(&indexlight);
                }
            }
        }
    });
}
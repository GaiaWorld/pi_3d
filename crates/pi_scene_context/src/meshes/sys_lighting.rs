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
    changes: ComponentChanged<SceneDirectLightsQueue>,
    scenes: Query<(Entity, &SceneDirectLightsQueue)>,
    layermask: Query<&LayerMask>,
    viewers: Query<(&SceneID, &ModelList)>,
    lightindex: Query<&SceneItemIndex>,
    meshes: Query<&ModelLightingIndexs>,
    // mut record: ResMut<pi_scene_shell::run_stage::RunSystemRecord>,
) {
    // record.0.push(String::from("sys_model_direct_lighting_modify_by_light"));
    changes.iter().for_each(|entity| {
        if let Ok((scene, queuedirect)) = scenes.get(*entity) {
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
                                // log::error!("Model Direct: {:?}", &indexlight);
                                ids.direct_light_data(&indexlight);
                            }
                        }
                    });
                }
            });
        }
    });
}

pub fn sys_model_direct_lighting_modify_by_model(
    scenes: Query<&SceneDirectLightsQueue>,
    transforms: Query<&GlobalMatrix>,
    enabled: Query<(&GlobalEnable, &DirectLight)>,
    layermask: Query<&LayerMask>,
    lightindex: Query<&SceneItemIndex>,
    addeds: ComponentAdded<LayerMask>,
    changes: ComponentChanged<LayerMask>,
    meshes: Query<(Entity, &SceneID, &ModelLightingIndexs)>,
    entitysets: Res<EntityFilterForComponentChanged>,
    // mut record: ResMut<pi_scene_shell::run_stage::RunSystemRecord>,
) {
    // record.0.push(String::from("sys_model_direct_lighting_modify_by_model"));
    let mut entities = entitysets.pop();
    changes.iter().for_each(|entity| {
        entities.insert(*entity);
    });
    addeds.iter().for_each(|entity| {
        entities.insert(*entity);
    });
    entities.iter().for_each(|entity| {
        if let Ok((idm, idscene, ids)) = meshes.get(*entity) {
            if let Ok(queuedirect) = scenes.get(idscene.0) {
                if let Ok(my) = layermask.get(idm) {
                    if let Some(ids) = &ids.bind {
                        let mut indexlight = vec![];
                        queuedirect.0.items().for_each(|idlight| {
                            if let (Ok(_lp), Ok(ly), Ok(lidx), Ok((enabled, _))) = (transforms.get(*idlight), layermask.get(*idlight), lightindex.get(*idlight), enabled.get(*idlight)) {
                                if enabled.0 && ly.include(my.0) {
                                    indexlight.push(lidx.val());
                                }
                            }
                        });
                        // log::error!("Model Direct2: {:?}", &indexlight);
                        ids.direct_light_data(&indexlight);
                    }
                }
            }
        }
    });
    entitysets.push(entities);
}

pub fn sys_model_point_lighting_modify_by_model(
    scenes: Query<&SceneOtherLightsQueue>,
    transforms: Query<&GlobalMatrix>,
    enabledpoint: Query<(&GlobalEnable, &PointLight)>,
    enabledspot: Query<(&GlobalEnable, &SpotLight)>,
    enabledhemi: Query<(&GlobalEnable, &HemisphericLight)>,
    layermask: Query<&LayerMask>,
    lightindex: Query<&SceneItemIndex>,
    
    addeds: ComponentAdded<LayerMask>,
    changes: ComponentChanged<LayerMask>,
    changes3: ComponentChanged<ModelForceLightings>,
    meshes: Query<(Entity, &SceneID, &ModelLightingIndexs, &ModelForceLightings)>,
    entitysets: Res<EntityFilterForComponentChanged>,
    // mut record: ResMut<pi_scene_shell::run_stage::RunSystemRecord>,
) {
    // record.0.push(String::from("sys_model_point_lighting_modify_by_model"));
    let mut entities = entitysets.pop();
    changes.iter().for_each(|entity| {
        entities.insert(*entity);
    });
    addeds.iter().for_each(|entity| {
        entities.insert(*entity);
    });
    changes3.iter().for_each(|entity| {
        entities.insert(*entity);
    });
    entities.iter().for_each(|entity| {
        if let Ok((idm, idscene, ids, forcelights)) = meshes.get(*entity) {
            if let Ok(queuepoint) = scenes.get(idscene.0) {
                if let Ok(my) = layermask.get(idm) {
                    if let Some(ids) = &ids.bind {
    
                        let mut indexlightpoint = vec![];
                        let mut indexlightspot = vec![];
                        let mut indexlighthemi = vec![];
    
                        forcelights.point.iter().for_each(|idlight| {
                            if let (Ok(_lp), Ok(_ly), Ok(lidx), Ok((enable, _))) = (transforms.get(*idlight), layermask.get(*idlight), lightindex.get(*idlight), enabledpoint.get(*idlight)) {
                                if enable.0 {
                                    let idx = lidx.val();
                                    if indexlightpoint.contains(&idx) == false { indexlightpoint.push(idx); }
                                }
                            }
                        });
                        forcelights.spot.iter().for_each(|idlight| {
                            if let (Ok(_lp), Ok(_ly), Ok(lidx), Ok((enable, _))) = (transforms.get(*idlight), layermask.get(*idlight), lightindex.get(*idlight), enabledspot.get(*idlight)) {
                                if enable.0 {
                                    let idx = lidx.val();
                                    if indexlightspot.contains(&idx) == false { indexlightspot.push(idx); }
                                }
                            }
                        });
                        forcelights.spot.iter().for_each(|idlight| {
                            if let (Ok(_lp), Ok(_ly), Ok(lidx), Ok((enable, _))) = (transforms.get(*idlight), layermask.get(*idlight), lightindex.get(*idlight), enabledhemi.get(*idlight)) {
                                if enable.0 {
                                    let idx = lidx.val();
                                    if indexlighthemi.contains(&idx) == false { indexlighthemi.push(idx); }
                                }
                            }
                        });
                        queuepoint.point.items().for_each(|idlight| {
                            if let (Ok(_lp), Ok(ly), Ok(lidx), Ok((enabled, _))) = (transforms.get(*idlight), layermask.get(*idlight), lightindex.get(*idlight), enabledpoint.get(*idlight)) {
                                if enabled.0 && ly.include(my.0) {
                                    let idx = lidx.val();
                                    if indexlightpoint.contains(&idx) == false { indexlightpoint.push(idx); }
                                }
                            }
                        });
                        queuepoint.spot.items().for_each(|idlight| {
                            if let (Ok(_lp), Ok(ly), Ok(lidx), Ok((enabled, _))) = (transforms.get(*idlight), layermask.get(*idlight), lightindex.get(*idlight), enabledspot.get(*idlight)) {
                                if enabled.0 && ly.include(my.0) {
                                    let idx = lidx.val();
                                    if indexlightspot.contains(&idx) == false { indexlightspot.push(idx); }
                                }
                            }
                        });
                        queuepoint.hemi.items().for_each(|idlight| {
                            if let (Ok(_lp), Ok(ly), Ok(lidx), Ok((enabled, _))) = (transforms.get(*idlight), layermask.get(*idlight), lightindex.get(*idlight), enabledhemi.get(*idlight)) {
                                if enabled.0 && ly.include(my.0) {
                                    let idx = lidx.val();
                                    if indexlighthemi.contains(&idx) == false { indexlighthemi.push(idx); }
                                }
                            }
                        });
                        ids.point_light_data(&indexlightpoint);
                        ids.spot_light_data(&indexlightspot);
                        ids.hemi_light_data(&indexlighthemi);
                    }
                }
            }
        }
    });
    entitysets.push(entities);
}

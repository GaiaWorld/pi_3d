
use pi_scene_shell::prelude::*;
use pi_scene_math::coordiante_system::CoordinateSytem3;
use pi_scene_shell::prelude::IntoSystemConfigs;

use crate::{
    materials::prelude::*,
    viewer::prelude::*,
    light::prelude::*,
    transforms::prelude::*,
    flags::GlobalEnable,
    scene::prelude::*,
    meshes::prelude::*,
    geometry::prelude::*,
    layer_mask::prelude::{LayerMask, TViewerLayerMask},
};

use super::{
    base::*,
    direct_light::*
};

pub fn sys_shadow_enabled_modify(
    lights: Query<(&GlobalEnable, &LightLinkedShadowID), Changed<GlobalEnable>>,
    mut shadows: Query<(&ShadowLinkedLightID, &mut ViewerActive)>,
) {
    lights.iter().for_each(|(enable, linkedshadow)| {
        if let Some(linkedshadow) = linkedshadow.0 {
            if let Ok((_linkedlight, mut active)) = shadows.get_mut(linkedshadow) {
                active.0 = enable.0;
            }
        }
    });
}

pub fn sys_shadow_param_update_while_mat_create(
    mut shadows: Query<
        &mut LinkedMaterialID
    >,
    materails: Query<
        Entity,
        Changed<BindEffectReset>,
    >,
) {
    shadows.iter_mut().for_each(|mut id_mat| {
        if let Ok(idmaterial) = materails.get(id_mat.0) {
            id_mat.0 = idmaterial;
        }
    });
}

pub fn sys_shadow_param_update(
    shadows: Query<
        (&LinkedMaterialID, &ShadowParam),
        Or<(
            Changed<LinkedMaterialID>, Changed<ShadowParam>
        )>
    >,
    // mut materails: Query<
    //     (&mut BindEffect, &mut BindEffectValueDirty)
    // >,
    mut cmds: ResMut<ActionListUniformFloat>,
) {
    shadows.iter().for_each(|(id_mat, shaow)| {

        cmds.push(OpsUniformFloat::ops(id_mat.0, Atom::from(KEY_SHADOW_DEPTH_BIAS), shaow.bias));
        cmds.push(OpsUniformFloat::ops(id_mat.0, Atom::from(KEY_SHADOW_NORMAL_BIAS), shaow.normalbias));
        cmds.push(OpsUniformFloat::ops(id_mat.0, Atom::from(KEY_SHADOW_DEPTH_SCALE), shaow.depthscale));
        cmds.push(OpsUniformFloat::ops(id_mat.0, Atom::from(KEY_SHADOW_MINZ), shaow.minz));
        cmds.push(OpsUniformFloat::ops(id_mat.0, Atom::from(KEY_SHADOW_MAXZ), shaow.maxz));
    });
}

pub fn sys_shadow_generator_apply_while_shadow_modify(
    shadows: Query<
        (&LinkedMaterialID, &ModelList, &ForceIncludeModelList, &ShadowCastPassTag),
        Or<(Changed<LinkedMaterialID>, Changed<FlagModelList>, Changed<ForceIncludeModelList>, Changed<ShadowCastPassTag>)>
    >,
    mut matcmds: ResMut<ActionListMaterialUse>,
    meshes: Query<&MeshCastShadow, With<Mesh>>,
    instances: Query<&InstanceMesh>,
    empty: Res<SingleEmptyEntity>,
) {
    shadows.iter().for_each(|(id_mat, modelist, forcemodels, passtag)| {
        if id_mat.0 != empty.id() {
            modelist.0.iter().for_each(|id_model| {
                if meshes.contains(*id_model) {
                    matcmds.push(OpsMaterialUse::Use(*id_model, id_mat.0, passtag.0));
                } else if let Ok(id_model) = instances.get(*id_model) {
                    matcmds.push(OpsMaterialUse::Use(id_model.0, id_mat.0, passtag.0));
                }
            });
            forcemodels.0.iter().for_each(|id_model| {
                if meshes.contains(*id_model) {
                    matcmds.push(OpsMaterialUse::Use(*id_model, id_mat.0, passtag.0));
                } else if let Ok(id_model) = instances.get(*id_model) {
                    matcmds.push(OpsMaterialUse::Use(id_model.0, id_mat.0, passtag.0));
                }
            });
        }
    });
}

pub fn sys_light_layermask_to_shadow(
    lights: Query<(Entity, &LightLinkedShadowID), Changed<LayerMask>>,
    layermasks: Query<&LayerMask, ()>,
    mut shadowlayermasks: Query<&mut ShadowLayerMask, ()>,
) {
    lights.iter().for_each(|(idlight, idshadow)| {
        if let (Some(idshadow), Ok(layerlight)) = (idshadow.0, layermasks.get(idlight)) {
            let layerlight = layerlight.clone();
            if let Ok(mut layershadow) = shadowlayermasks.get_mut(idshadow) {
                layershadow.0 = layerlight;
            }
        }
    });
}

pub fn sys_shadow_project_modify_by_direction_light(
    mut shadows: Query<(&ShadowParam, &mut DirectionalShadowProjection), Changed<ShadowParam>>,
    // mut record: ResMut<pi_scene_shell::run_stage::RunSystemRecord>,
) {
    // record.0.push(String::from("sys_shadow_project_modify_by_direction_light"));
    shadows.iter_mut().for_each(|(shaow, mut project)| {
        *project = DirectionalShadowProjection { minz: shaow.minz, maxz: shaow.maxz, frustum_size: shaow.frustum };
    });
}

pub fn sys_shadow_project_modify_by_spot_light(
    lights: Query<(&SpotLightAngle, &LightLinkedShadowID)>,
    mut shadows: Query<(&ShadowLinkedLightID, &ShadowParam, &mut SpotShadowProjection)>,
) {
    shadows.iter_mut().for_each(|(idlight, shadow, mut project)| {
        if let Ok((outangle, _)) = lights.get(idlight.0) {
            *project = SpotShadowProjection { minz: shadow.minz, maxz: shadow.maxz, fov: outangle.out_value };
            // log::warn!("Spot {:?}", maxz.0);
        }
    });
}

pub fn sys_calc_view_matrix_by_light(
    mut lights: Query<(Entity, &LocalPosition, &LightLinkedShadowID, &LightDirection), Or<(Changed<LocalPosition>, Changed<LightLinkedShadowID>, Changed<LightDirection>, Changed<GlobalMatrix>)>>,
    mut transforms: Query<(&GlobalMatrix, &mut AbsoluteTransform)>,
    mut viewers: Query<(&ShadowLinkedLightID, &mut DirectionalShadowDirection, &mut ViewerViewMatrix, &mut ViewerGlobalPosition, &mut ViewerDirection)>,
    // childrens: Query<&NodeParent>,
    childrens: Query<&Up>,
    // mut record: ResMut<pi_scene_shell::run_stage::RunSystemRecord>,
) {
    // record.0.push(String::from("sys_calc_view_matrix_by_light"));
    //  log::debug!("View Matrix Calc:");
    let coordsys = CoordinateSytem3::left();
    lights.iter_mut().for_each(|(entity, l_position, idshadow, ldirection)| {
        if let Some(idshadow) = idshadow.0 {
            if let Ok((_linklight, mut viewcalc, mut viewmatrix, mut viewposition, mut viewdirection)) = viewers.get_mut(idshadow) {
                viewcalc.0.clone_from(&ldirection.0);
                viewdirection.0.clone_from(&ldirection.0);
                // log::warn!("View Matrix Calc: {:?}", viewcalc.0);
                if let Ok(parent) = childrens.get(entity) {
                    let parent_id = parent.parent();
                    if let Ok((parent, mut absolute)) = transforms.get_mut(parent_id) {
                        let iso = absolute.iso(&parent.matrix());
                        let (matrix, pos) = viewcalc.view_matrix(&coordsys, l_position, Some((&parent, iso)));
                        *viewmatrix = matrix;
                        *viewposition = pos;
                    } else {
                        let (matrix, pos) = viewcalc.view_matrix(&coordsys, l_position, None);
                        *viewmatrix = matrix;
                        *viewposition = pos;
                    }
                };
            }
        }
    });
}


pub fn sys_shadow_bind_modify(
    scenes: Query<(Entity, &SceneShadowInfos, &SceneShadowQueue)>,
    shadows: Query<(&ShadowLinkedLightID, &ViewerTransformMatrix, &ShadowParam)>,
    indexs: Query<&SceneItemIndex>,
    // mut record: ResMut<pi_scene_shell::run_stage::RunSystemRecord>,
) {
    // record.0.push(String::from("sys_shadow_bind_modify"));
    scenes.iter().for_each(|(_entity, shadowdata, queueshadow)| {
        shadowdata.0.as_ref().unwrap().reset();
        queueshadow.0.items().for_each(|v| {
            if let (Ok(indexshadow), Ok((light, matrix, shadow))) = (indexs.get(*v), shadows.get(*v)) {
                let uscale = 1.;
                let vscale = 1.;
                let uoff = 0.;
                let voff = 0.;
                if let Ok(indexlight) = indexs.get(light.0) {
                    shadowdata.0.as_ref().unwrap().direct_shadow_data(indexlight.val(), indexshadow.val(), matrix.0.as_slice(), shadow.bias, shadow.normalbias, shadow.depthscale, 0., uscale, vscale, uoff, voff)
                }
            }
        });
    });
}

pub fn sys_dispose_about_shadowcaster(
    items: Query<(Entity, &DisposeReady, &SceneID, &SceneItemIndex, &LinkedMaterialID), (Changed<DisposeReady>, With<ShadowCastPassTag>)>,
    mut disposereadylist: ResMut<ActionListDisposeReadyForRef>,
    mut disposecanlist: ResMut<ActionListDisposeCan>,
    mut scenes: Query< &mut SceneShadowQueue >,
    _empty: Res<SingleEmptyEntity>,
) {
    items.iter().for_each(|(entity, state, idscene, shadowindex, idmaterial)| {
        if state.0 == false { return; }

        if let Ok(mut queueshadow) = scenes.get_mut(idscene.0) {
            queueshadow.0.recycle(shadowindex, &entity);
        }

        disposereadylist.push(OpsDisposeReadyForRef::ops(idmaterial.0));

        disposecanlist.push(OpsDisposeCan::ops(entity));
    });

}

pub fn sys_update_shadow_viewer_model_list_by_viewer<T: TViewerViewMatrix + Component, T2: TViewerProjectMatrix + Component>(
    mut viewers: Query<
        (Entity, &ViewerActive, &SceneID, &ShadowLayerMask, &mut ModelList, &mut FlagModelList),
        (Or<(Changed<ShadowLayerMask>, Changed<ViewerActive>)>, With<T>, With<T2>)
    >,
    items: Query<
        (Entity, &SceneID, &LayerMask, &InstanceSourceRefs, &MeshCastShadow),
    >,
) {
    viewers.iter_mut().for_each(|(_camera, vieweractive, scene, layer, mut list_model, mut flag_list_model)| {
        _sys_update_shadow_viewer_model_list_by_viewer(
            vieweractive, scene, layer, &mut list_model, &mut flag_list_model, &items,
        );
    });

    // log::debug!("SysModelListUpdateByViewer: {:?}", pi_time::Instant::now() - time1);
}
#[inline(never)]
fn _sys_update_shadow_viewer_model_list_by_viewer(
    vieweractive: &ViewerActive, scene: &SceneID, layer: &ShadowLayerMask, list_model: &mut ModelList, flag_list_model: &mut FlagModelList,
    items: &Query<
        (Entity, &SceneID, &LayerMask, &InstanceSourceRefs, &MeshCastShadow),
    >,
) {
    list_model.0.clear();
    // log::warn!("CameraModelListByViewer : {:?}", _camera);
    if vieweractive.0 {
        // log::warn!("SysModelListUpdateByCamera: 0");
        items.iter().for_each(|(id_obj, iscene, ilayer, instances, castshadow)| {
            // log::debug!("SysModelListUpdateByCamera: 1");
            if iscene == scene && layer.include(ilayer.0) && castshadow.0 {
                // log::debug!("SysModelListUpdateByCamera: 2");
                if list_model.0.contains(&id_obj) {
                    // log::warn!("Has Include {:?}", id_obj);
                } else {
                    list_model.0.insert(id_obj);
                    *flag_list_model = FlagModelList::default();
                }
                instances.iter().for_each(|entity| {
                    list_model.0.insert(*entity);
                });
                *flag_list_model = FlagModelList::default();
            } else {
                list_model.0.remove(&id_obj);
                instances.iter().for_each(|entity| {
                    list_model.0.remove(entity);
                });
                // log::warn!("{:?}", (iscene, scene, layer, ilayer));
            }
        });
    }
}

pub fn sys_update_shadow_viewer_model_list_by_model<T: TViewerViewMatrix + Component, T2: TViewerProjectMatrix + Component>(
    mut viewers: Query<
        (&ViewerActive, &SceneID, &ShadowLayerMask, &mut ModelList, &mut FlagModelList),
        (With<T>, With<T2>)
    >,
    items: Query<
        (Entity, &SceneID, Option<&LayerMask>, Option<&InstanceSourceRefs>, &DisposeReady, &AbstructMesh, &MeshCastShadow),
        Or<(Changed<LayerMask>, Changed<DisposeReady>, Changed<InstanceSourceRefs>, Changed<MeshCastShadow>)>,
    >,
    // mut record: ResMut<pi_scene_shell::run_stage::RunSystemRecord>,
) {
    // record.0.push(String::from("sys_update_shadow_viewer_model_list_by_model"));
    // let time1 = pi_time::Instant::now();
    // log::debug!("CameraModelListByModel :");

    items.iter().for_each(|(id_obj, iscene, ilayer, instances, disposestate, _, castshadow)| {
        // log::debug!("CameraModelListByModel : 0");
        viewers.iter_mut().for_each(|(vieweractive, scene, layer, mut list_model, mut flag_list_model)| {
            // log::debug!("CameraModelListByModel : 1");
            _sys_update_shadow_viewer_model_list_by_model(
                id_obj, iscene, ilayer, instances, disposestate, castshadow,
                vieweractive, scene, layer, &mut list_model, &mut flag_list_model
            );
        });
    });

    // log::debug!("SysModelListUpdateByModel: {:?}", pi_time::Instant::now() - time1);
}
#[inline(never)]
fn _sys_update_shadow_viewer_model_list_by_model(
    id_obj: Entity, iscene: &SceneID, ilayer: Option<&LayerMask>, instances: Option<&InstanceSourceRefs>, disposestate: &DisposeReady, castshadow: &MeshCastShadow,
    vieweractive: &ViewerActive, scene: &SceneID, layer: &ShadowLayerMask, list_model: &mut ModelList, flag_list_model: &mut FlagModelList,
) {
    if vieweractive.0 {
        if iscene == scene && disposestate.0 == false {
            if let (Some(ilayer), Some(instances)) = (ilayer, instances) {
                if layer.include(ilayer.0) && castshadow.0 {
                    list_model.0.insert(id_obj);
                    *flag_list_model = FlagModelList::default();
                    instances.iter().for_each(|entity| {
                        list_model.0.insert(*entity);
                    });
                } else {
                    list_model.0.remove(&id_obj);
                    instances.iter().for_each(|entity| {
                        list_model.0.remove(entity);
                    });
                }
            }
        } else {
            list_model.0.remove(&id_obj);
        }
    }
}


#[cfg(not(feature = "use_bevy"))]
use std::collections::BTreeSet;

use pi_scene_shell::prelude::*;

use crate::{
    cullings::prelude::*, flags::*, geometry::{instance::types::ModelInstanceAttributes, prelude::*}, layer_mask::prelude::*, meshes::prelude::*
};

use super::base::*;

#[cfg(feature = "use_bevy")]
struct SceneBoundingFilter<'a, 'w, 's>(
    pub &'a Query<'w, (&'static GlobalEnable, Option<&'static MeshInstanceState>), With<AbstructMesh>>,
    pub &'a XHashSet<Entity>
);
#[cfg(feature = "use_bevy")]
impl<'a, 'w, 's> TFilter for SceneBoundingFilter<'a, 'w, 's> {
    fn filter(&self, entity: Entity) -> bool {
        if self.1.contains(&entity) {
            if let Ok((enable, instances)) = self.0.get(entity) {
                if let Some(instances) = instances {
                    if instances.instance_matrix || instances.instances.len() > 0 {
                        true
                    } else {
                        enable.0
                    }
                } else {
                    enable.0
                }
            } else {
                false
            }
        } else {
            false
        }
    }
}

#[cfg(not(feature = "use_bevy"))]
struct SceneBoundingFilter<'a, 'w>(
    pub &'a Query<'w, (&'static GlobalEnable, Option<&'static MeshInstanceState>), With<AbstructMesh>>,
    pub &'a BTreeSet<Entity>
);
#[cfg(not(feature = "use_bevy"))]
impl<'a, 'w> TFilter for SceneBoundingFilter<'a, 'w> {
    fn filter(&self, entity: Entity) -> bool {
        if self.1.contains(&entity) {
            self.query(entity)
        } else {
            false
        }
    }
    fn query(&self, entity: Entity) -> bool {
        if let Ok((enable, instances)) = self.0.get(entity) {
            if let Some(instances) = instances {
                if instances.instance_matrix || instances.instances.len() > 0 {
                    true
                } else {
                    enable.0
                }
            } else {
                enable.0
            }
        } else {
            false
        }
    }
    fn iter(&self) -> std::collections::btree_set::Iter<Entity> {
        self.1.iter()
    }
}

pub fn sys_abstructmesh_culling_flag_reset(
    mut items: Query<&mut AbstructMeshCullingFlag>,
) {
    items.iter_mut().for_each(|mut item| {
        *item = AbstructMeshCullingFlag(false);
    });
}

pub fn sys_update_viewer_model_list_by_viewer<T: TViewerViewMatrix + Component, T2: TViewerProjectMatrix + Component>(
    mut viewers: Query<
        (Entity, &ViewerActive, &SceneID, &LayerMask, &mut ModelList, &mut FlagModelList),
        (Or<(Changed<LayerMask>, Changed<ViewerActive>)>, With<T>, With<T2>)
    >,
    items: Query<
        (Entity, &SceneID, &LayerMask, &InstanceSourceRefs),
    >,
) {
    // let time1 = pi_time::Instant::now();

    // log::debug!("CameraModelListByViewer :");
    // log::debug!("SysModelListUpdateByCamera: ");
    viewers.iter_mut().for_each(|(_camera, vieweractive, scene, layer, mut list_model, mut flag_list_model)| {
        _sys_update_viewer_model_list_by_viewer(
            vieweractive, scene, layer, &mut list_model, &mut flag_list_model, &items
        );
    });

    // log::debug!("SysModelListUpdateByViewer: {:?}", pi_time::Instant::now() - time1);
}

fn _sys_update_viewer_model_list_by_viewer(
    vieweractive: &ViewerActive, scene: &SceneID, layer: &LayerMask, list_model: &mut ModelList, flag_list_model: &mut FlagModelList,
    items: &Query<
        (Entity, &SceneID, &LayerMask, &InstanceSourceRefs),
    >,
) {
    list_model.0.clear();
    // log::warn!("CameraModelListByViewer : {:?}", _camera);
    if vieweractive.0 {
        // log::warn!("SysModelListUpdateByCamera: 0");
        items.iter().for_each(|(id_obj, iscene, ilayer, instances)| {
            if iscene == scene && layer.include(ilayer.0) {
                if list_model.0.contains(&id_obj) == false {
                    list_model.0.insert(id_obj);
                    *flag_list_model = FlagModelList::default();
                }
                instances.iter().for_each(|(_k, instance)| {
                    list_model.0.insert(*instance);
                });
                *flag_list_model = FlagModelList::default();
            }
        });
    }
}

pub fn sys_update_viewer_model_list_by_model<T: TViewerViewMatrix + Component, T2: TViewerProjectMatrix + Component>(
    mut viewers: Query<
        (&ViewerActive, &SceneID, &LayerMask, &mut ModelList, &mut FlagModelList), (With<T>, With<T2>)
    >,
    items: Query<
        (Entity, &SceneID, &LayerMask, &InstanceSourceRefs, &DisposeReady, &AbstructMesh),
        Changed<FlagMeshNeedRecheckForView>
    >,
) {
    items.iter().for_each(|(id_obj, iscene, ilayer, instances, disposestate, _)| {
        viewers.iter_mut().for_each(|(vieweractive, scene, layer, mut list_model, mut flag_list_model)| {
            // log::error!("CameraModelListByModel : 1");
            _sys_update_viewer_model_list_by_model(
                id_obj, iscene, ilayer, instances, disposestate,
                vieweractive, scene, layer, &mut list_model, &mut flag_list_model
            );
        });
    });

    // log::debug!("SysModelListUpdateByModel: {:?}", pi_time::Instant::now() - time1);
}

fn _sys_update_viewer_model_list_by_model(
    id_obj: Entity, iscene: &SceneID, ilayer: &LayerMask, instances: &InstanceSourceRefs, disposestate: &DisposeReady,
    vieweractive: &ViewerActive, scene: &SceneID, layer: &LayerMask, list_model: &mut ModelList, flag_list_model: &mut FlagModelList,
) {
    if vieweractive.0 {
        if iscene == scene && disposestate.0 == false {
            if layer.include(ilayer.0) {
                list_model.0.insert(id_obj);
                *flag_list_model = FlagModelList::default();
                instances.iter().for_each(|(_k, instance)| {
                    list_model.0.insert(*instance);
                });
            } else {
                list_model.0.remove(&id_obj);
                instances.iter().for_each(|(_k, instance)| {
                    list_model.0.remove(instance);
                });
            }
        } else {
            list_model.0.remove(&id_obj);
        }
    }
}
// 物件变化时暂不好设置viewer的重新剔除，因此暂时总是重新剔除
pub fn sys_tick_viewer_culling(
    mut viewers: Query<
        (&SceneID, &ViewerActive, &ModelList, &ViewerTransformMatrix, &ViewerViewMatrix, &ForceIncludeModelList, &mut ModelListAfterCulling),
        // Or<(Changed<ModelList>, Changed<ViewerTransformMatrix>, Changed<ViewerViewMatrix>, Changed<ForceIncludeModelList>, Changed<ViewerCullingDirty>)>
    >,
    items: Query< (& GlobalEnable, Option<& MeshInstanceState>), With<AbstructMesh> >,
    mut flags: Query<&mut AbstructMeshCullingFlag>,
    mut meshes: Query<(&mut InstanceSourceRefs, &ModelInstanceAttributes)>,
    mut scenes: Query<
        &mut SceneBoundingPool
    >,
    mut performance: ResMut<Performance>,
    entitysets: Res<EntityFilterForComponentChanged>,
) {
    // log::error!("sys_tick_viewer_culling");
    // performance.systems.push(String::from("sys_tick_viewer_culling"));
    let mut sources = entitysets.pop();
    if performance.debug { performance.t_culling = pi_time::Instant::now(); }
    viewers.iter_mut().for_each(|(idscene, vieweractive, list_model, transform, _cameraview, forceincludes, mut cullings)| {
        cullings.0.clear();
        if vieweractive.0 {
            if let Ok(mut culling) = scenes.get_mut(idscene.0) {
                culling.culling(
                    transform,
                    SceneBoundingFilter(&items, &list_model.0),
                    &mut cullings.0
                );
                // log::error!("Culling: {:?}", (cullings.0.len(), culling.size(), list_model.0.len()));
            } else {
                list_model.0.iter().for_each(|objid| {
                    if let Ok((enable, instances)) = items.get(objid.clone()) {
                        if let Some(instances) = instances {
                            if instances.instance_matrix || instances.instances.len() > 0 {
                                cullings.0.push(objid.clone());
                            } else if enable.0 {
                                cullings.0.push(objid.clone());
                            }
                        } else {
                            if enable.0 {
                                cullings.0.push(objid.clone());
                            }
                        }
                    }
                });
            }
            
            forceincludes.0.iter().for_each(|objid: &Entity| {
                if let Ok((enable, instances)) = items.get(objid.clone()) {
                        if let Some(instances) = instances {
                            if instances.instance_matrix || instances.instances.len() > 0 {
                                cullings.0.push(objid.clone());
                            } else if enable.0 {
                                cullings.0.push(objid.clone());
                            }
                        } else {
                            if enable.0 {
                                cullings.0.push(objid.clone());
                            }
                        }
                }
            });

            cullings.0.iter().for_each(|id| {
                if let Ok(mut flag) = flags.get_mut(*id) {
                    *flag = AbstructMeshCullingFlag(true);
                }
                if !sources.insert(id) { return; }
                if let Ok((mut flag, attrs)) = meshes.get_mut(*id) {
                    if attrs.bytes().len() > 0 {
                        flag.set_changed();
                    }
                }
            });
        }
    });
    entitysets.push(sources);
    // // 尝试记录已成功剔除的后续不再计算剔除逻辑，但测试结果耗时更长
    // scenes.iter_mut().for_each(|mut items| {
    //     items.reset_temp();
    // });
    if performance.debug { performance.culling = (pi_time::Instant::now() - performance.t_culling).as_micros() as u32; }
}

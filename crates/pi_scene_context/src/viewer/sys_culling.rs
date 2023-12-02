

use pi_engine_shell::prelude::*;
use pi_hash::XHashSet;

use crate::{
    layer_mask::prelude::*,
    meshes::prelude::*,
    geometry::prelude::*,
    cullings::prelude::*,
    commands::*,
    flags::*,
};

use super::base::*;

struct SceneBoundingFilter<'a, 'w, 's>(pub &'a Query<'w, 's, (&'static GlobalEnable, Option<&'static InstanceSourceRefs>), With<AbstructMesh>>, pub &'a XHashSet<Entity>);
impl<'a, 'w, 's> TFilter for SceneBoundingFilter<'a, 'w, 's> {
    fn filter(&self, entity: Entity) -> bool {
        if self.1.contains(&entity) {
            if let Ok((enable, instances)) = self.0.get(entity) {
                if let Some(instances) = instances {
                    if instances.len() > 0 {
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

pub fn sys_abstructmesh_culling_flag_reset(
    mut items: Query<&mut AbstructMeshCullingFlag>,
) {
    items.iter_mut().for_each(|mut item| {
        *item = AbstructMeshCullingFlag(false);
    });
}

    pub fn sys_update_viewer_model_list_by_viewer<T: TViewerViewMatrix + Component, T2: TViewerProjectMatrix + Component, L: TViewerLayerMask + Component>(
        mut viewers: Query<
            (Entity, &ViewerActive, &SceneID, &L, &mut ModelList, &mut FlagModelList, &T, &T2),
            Or<(Changed<L>, Changed<ViewerActive>)>
        >,
        items: Query<
            (Entity, &SceneID, &LayerMask, &InstanceSourceRefs),
        >,
    ) {
        // let time1 = pi_time::Instant::now();

        // log::debug!("CameraModelListByViewer :");
        // log::debug!("SysModelListUpdateByCamera: ");
        viewers.iter_mut().for_each(|(_camera, vieweractive, scene, layer, mut list_model, mut flag_list_model, _, _)| {
            list_model.0.clear();
            // log::warn!("CameraModelListByViewer : {:?}", _camera);
            if vieweractive.0 {
                // log::warn!("SysModelListUpdateByCamera: 0");
                items.iter().for_each(|(id_obj, iscene, ilayer, instances)| {
                    // log::debug!("SysModelListUpdateByCamera: 1");
                    if iscene == scene && layer.include(ilayer.0) {
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
                        // log::warn!("{:?}", (iscene, scene, layer, ilayer));
                    }
                });
            }
        });

        // log::debug!("SysModelListUpdateByViewer: {:?}", pi_time::Instant::now() - time1);
    }

    pub fn sys_update_viewer_model_list_by_model<T: TViewerViewMatrix + Component, T2: TViewerProjectMatrix + Component, L: TViewerLayerMask + Component>(
        mut viewers: Query<
            (&ViewerActive, &SceneID, &L, &mut ModelList, &mut FlagModelList, &T, &T2),
        >,
        items: Query<
            (Entity, &SceneID, Option<&LayerMask>, Option<&InstanceSourceRefs>, &DisposeReady, &AbstructMesh),
            Or<(Changed<LayerMask>, Changed<DisposeReady>, Changed<InstanceSourceRefs>)>,
        >,
    ) {
        // let time1 = pi_time::Instant::now();
        // log::debug!("CameraModelListByModel :");

        items.iter().for_each(|(id_obj, iscene, ilayer, instances, disposestate, _)| {
            // log::error!("CameraModelListByModel : 0");
            viewers.iter_mut().for_each(|(vieweractive, scene, layer, mut list_model, mut flag_list_model, _, _)| {
                // log::error!("CameraModelListByModel : 1");
                if vieweractive.0 {
                    if iscene == scene && disposestate.0 == false {
                        // log::error!("CameraModelListByModel : 2");
                        if let (Some(ilayer), Some(instances)) = (ilayer, instances) {
                            if layer.include(ilayer.0) {
                                // log::error!("CameraModelListByModel : 3");
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
            });
        });

        // log::debug!("SysModelListUpdateByModel: {:?}", pi_time::Instant::now() - time1);
    }

    pub fn sys_tick_viewer_culling<T: TViewerViewMatrix + Component, T2: TViewerProjectMatrix + Component, R: TCullingPerformance + Resource>(
        mut viewers: Query<
            (&SceneID, &ViewerActive, &ModelList, &ViewerTransformMatrix, &ViewerViewMatrix, &ForceIncludeModelList, &mut ModelListAfterCulling),
            (With<T>, With<T2>)
        >,
        items: Query<
            (&'static GlobalEnable, Option<&'static InstanceSourceRefs>),
            With<AbstructMesh>
        >,
        mut flags: Query<&mut AbstructMeshCullingFlag>,
        scenes: Query<
            &SceneBoundingPool
        >,
        mut performance: ResMut<R>
    ) {
        let time1 = pi_time::Instant::now();
        // log::warn!("SysModelListAfterCullinUpdateByCamera: ");
        viewers.iter_mut().for_each(|(idscene, vieweractive, list_model, transform, _cameraview, forceincludes, mut cullings)| {
            // log::warn!("SysViewerCulling: {:?}", vieweractive);
            cullings.0.clear();
            if vieweractive.0 {
                if let Ok(culling) = scenes.get(idscene.0) {
                    culling.culling(
                        transform,
                        SceneBoundingFilter(&items, &list_model.0),
                        &mut cullings.0
                    );
                } else {
                    // log::warn!("ModelList: {:?}", (list_model.0.len(), forceincludes.0.len()));
                    list_model.0.iter().for_each(|objid| {
                        // log::debug!("SysModelListAfterCullinUpdateByCamera: 1");
                        if let Ok((enable, instances)) = items.get(objid.clone()) {
                            // log::warn!("Moldellist Geo: {:?}, {:?}", enable.0, geo_enable.0);
                            // log::debug!("SysModelListAfterCullinUpdateByCamera: 2");
                            if let Some(instances) = instances {
                                if instances.len() > 0 {
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
                    // log::error!("forceincludes: ");
                    if let Ok((enable, instances)) = items.get(objid.clone()) {
                        // log::warn!("Moldellist Geo: {:?}, {:?}", enable.0, geo_enable.0);
                        // log::debug!("SysModelListAfterCullinUpdateByCamera: 2");
                        if let Some(instances) = instances {
                            if instances.len() > 0 {
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
                });
            }
            // log::warn!("Moldellist: {:?}, {:?}, {:?}", vieweractive.0, liet_model.0.len(), cullings.0.len());
        });

        performance.culling_time((pi_time::Instant::now() - time1).as_micros() as u32);
        
        // log::debug!("SysModelListAfterCullingTick: {:?}", pi_time::Instant::now() - time1);
    }
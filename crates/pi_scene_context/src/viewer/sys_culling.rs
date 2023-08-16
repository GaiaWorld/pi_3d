

use pi_engine_shell::prelude::*;

use crate::{
    layer_mask::prelude::*,
    geometry::prelude::*,
    meshes::prelude::*, prelude::GlobalEnable
};

use super::base::*;


    pub fn sys_update_viewer_model_list_by_viewer<T: TViewerViewMatrix + Component, T2: TViewerProjectMatrix + Component>(
        mut viewers: Query<
            (ObjectID, &ViewerActive, &SceneID, &LayerMask, &mut ModelList, Option<&T>, Option<&T2>),
            Or<(Changed<LayerMask>, Changed<ViewerActive>)>
        >,
        items: Query<
            (ObjectID, &SceneID, &LayerMask, &Mesh),
        >,
        mut commands: Commands,
    ) {
        let time1 = pi_time::Instant::now();

        // log::debug!("CameraModelListByViewer :");
        // log::debug!("SysModelListUpdateByCamera: ");
        viewers.iter_mut().for_each(|(camera, vieweractive, scene, layer, mut list_model, _, _)| {
            list_model.0.clear();
            // log::debug!("CameraModelListByViewer : 0");
            if vieweractive.0 {
                // log::debug!("SysModelListUpdateByCamera: 0");
                items.iter().for_each(|(id_obj, iscene, ilayer, _)| {
                    // log::debug!("SysModelListUpdateByCamera: 1");
                    if iscene == scene && layer.include(ilayer) {
                        // log::debug!("SysModelListUpdateByCamera: 2");
                        if list_model.0.contains_key(&id_obj) {
                            // log::warn!("Has Include {:?}", id_obj);
                        } else {
                            list_model.0.insert(id_obj, id_obj);
                        }
                    }
                });
            }
    
            commands.entity(camera).insert(FlagModelList(true));
        });

        // log::debug!("SysModelListUpdateByViewer: {:?}", pi_time::Instant::now() - time1);
    }

    pub fn sys_update_viewer_model_list_by_model<T: TViewerViewMatrix + Component, T2: TViewerProjectMatrix + Component>(
        mut viewers: Query<
            (ObjectID, &ViewerActive, &SceneID, &LayerMask, &mut ModelList, Option<&T>, Option<&T2>),
        >,
        items: Query<
            (ObjectID, &SceneID, &LayerMask, &Mesh),
            Changed<LayerMask>,
        >,
        mut commands: Commands,
    ) {
        let time1 = pi_time::Instant::now();
        // log::debug!("CameraModelListByModel :");

        items.iter().for_each(|(id_obj, iscene, ilayer, _)| {
            // log::debug!("CameraModelListByModel : 0");
            viewers.iter_mut().for_each(|(id_viewer, vieweractive, scene, layer, mut list_model, _, _)| {
                // log::debug!("CameraModelListByModel : 1");
                if vieweractive.0 {
                    if iscene == scene && layer.include(ilayer) {
                        if list_model.0.contains_key(&id_obj) {
                            // log::warn!("Has Include {:?}", id_obj);
                        } else {
                            list_model.0.insert(id_obj, id_obj);
                        }
                    } else {
                        list_model.0.remove(&id_obj);
                    }

                    commands.entity(id_viewer).insert(FlagModelList(true));
                }
            });
        });

        // log::debug!("SysModelListUpdateByModel: {:?}", pi_time::Instant::now() - time1);
    }

    pub fn sys_tick_viewer_culling<T: TViewerViewMatrix + Component, T2: TViewerProjectMatrix + Component>(
        mut viewers: Query<
            (ObjectID, &ViewerActive, &ModelList, &ViewerGlobalPosition, &ViewerViewMatrix, &mut ModelListAfterCulling),
            (With<T>, With<T2>)
        >,
        items: Query<
            (ObjectID, &GlobalEnable, &RenderWorldMatrix, &RenderGeometryEable),
            With<AbstructMesh>
        >,
    ) {
        let time1 = pi_time::Instant::now();
        // log::debug!("SysModelListAfterCullinUpdateByCamera: ");
        viewers.iter_mut().for_each(|(id_viewer, vieweractive, liet_model, camerapos, cameraview, mut cullings)| {
            log::warn!("SysViewerCulling: {:?}", vieweractive);
            cullings.0.clear();
            if vieweractive.0 {
                log::warn!("ModelList: {:?}", liet_model.0.len());
                liet_model.0.iter().for_each(|(objid, _)| {
                    // log::debug!("SysModelListAfterCullinUpdateByCamera: 1");
                    if let Ok((_, enable, _, geo_enable)) = items.get(objid.clone()) {
                        log::warn!("Moldellist Geo: {:?}, {:?}", enable.0, geo_enable.0);
                        // log::debug!("SysModelListAfterCullinUpdateByCamera: 2");
                        if enable.0 && geo_enable.0 {
                            cullings.0.push(objid.clone());
                        }
                    }
                });
            }
            // log::warn!("Moldellist: {:?}, {:?}, {:?}", vieweractive.0, liet_model.0.len(), cullings.0.len());
        });
        
        // log::debug!("SysModelListAfterCullingTick: {:?}", pi_time::Instant::now() - time1);
    }


use pi_engine_shell::prelude::*;
use pi_hash::XHashSet;

use crate::{
    layer_mask::prelude::*,
    meshes::prelude::*, prelude::GlobalEnable, cullings::prelude::*
};

use super::base::*;

struct SceneBoundingFilter<'a, 'w, 's>(pub &'a Query<'w, 's, &'static GlobalEnable, With<AbstructMesh>>, pub &'a XHashSet<Entity>);
impl<'a, 'w, 's> TFilter for SceneBoundingFilter<'a, 'w, 's> {
    fn filter(&self, entity: Entity) -> bool {
        if self.1.contains(&entity) {
            if let Ok(enable) = self.0.get(entity) {
                enable.0
            } else {
                false
            }
        } else {
            false
        }
    }
}


    pub fn sys_update_viewer_model_list_by_viewer<T: TViewerViewMatrix + Component, T2: TViewerProjectMatrix + Component>(
        mut viewers: Query<
            (Entity, &ViewerActive, &SceneID, &LayerMask, &mut ModelList, &mut FlagModelList, &T, &T2),
            Or<(Changed<LayerMask>, Changed<ViewerActive>)>
        >,
        items: Query<
            (Entity, &SceneID, &LayerMask, &Mesh),
        >,
    ) {
        // let time1 = pi_time::Instant::now();

        // log::debug!("CameraModelListByViewer :");
        // log::debug!("SysModelListUpdateByCamera: ");
        viewers.iter_mut().for_each(|(_camera, vieweractive, scene, layer, mut list_model, mut flag_list_model, _, _)| {
            list_model.0.clear();
            // log::debug!("CameraModelListByViewer : 0");
            if vieweractive.0 {
                // log::debug!("SysModelListUpdateByCamera: 0");
                items.iter().for_each(|(id_obj, iscene, ilayer, _)| {
                    // log::debug!("SysModelListUpdateByCamera: 1");
                    if iscene == scene && layer.include(ilayer) {
                        // log::debug!("SysModelListUpdateByCamera: 2");
                        if list_model.0.contains(&id_obj) {
                            // log::warn!("Has Include {:?}", id_obj);
                        } else {
                            list_model.0.insert(id_obj);
                            *flag_list_model = FlagModelList::default();
                        }
                    }
                });
            }
        });

        // log::debug!("SysModelListUpdateByViewer: {:?}", pi_time::Instant::now() - time1);
    }

    pub fn sys_update_viewer_model_list_by_model<T: TViewerViewMatrix + Component, T2: TViewerProjectMatrix + Component>(
        mut viewers: Query<
            (&ViewerActive, &SceneID, &LayerMask, &mut ModelList, &mut FlagModelList, &T, &T2),
        >,
        items: Query<
            (Entity, &SceneID, &LayerMask, &Mesh),
            Changed<LayerMask>,
        >,
    ) {
        // let time1 = pi_time::Instant::now();
        // log::debug!("CameraModelListByModel :");

        items.iter().for_each(|(id_obj, iscene, ilayer, _)| {
            // log::debug!("CameraModelListByModel : 0");
            viewers.iter_mut().for_each(|(vieweractive, scene, layer, mut list_model, mut flag_list_model, _, _)| {
                // log::debug!("CameraModelListByModel : 1");
                if vieweractive.0 {
                    if iscene == scene && layer.include(ilayer) {
                        if list_model.0.contains(&id_obj) {
                            // log::warn!("Has Include {:?}", id_obj);
                        } else {
                            list_model.0.insert(id_obj);
                            *flag_list_model = FlagModelList::default();
                        }
                    } else {
                        list_model.0.remove(&id_obj);
                    }
                }
            });
        });

        // log::debug!("SysModelListUpdateByModel: {:?}", pi_time::Instant::now() - time1);
    }

    pub fn sys_tick_viewer_culling<T: TViewerViewMatrix + Component, T2: TViewerProjectMatrix + Component>(
        mut viewers: Query<
            (&SceneID, &ViewerActive, &ModelList, &ViewerTransformMatrix, &ViewerViewMatrix, &mut ModelListAfterCulling),
            (With<T>, With<T2>)
        >,
        items: Query<
            &'static GlobalEnable,
            With<AbstructMesh>
        >,
        scenes: Query<
            &SceneBoundingPool
        >,
        mut performance: ResMut<Performance>
    ) {
        let time1 = pi_time::Instant::now();
        // log::debug!("SysModelListAfterCullinUpdateByCamera: ");
        viewers.iter_mut().for_each(|(idscene, vieweractive, list_model, transform, _cameraview, mut cullings)| {
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
                    // log::warn!("ModelList: {:?}", liet_model.0.len());
                    list_model.0.iter().for_each(|objid| {
                        // log::debug!("SysModelListAfterCullinUpdateByCamera: 1");
                        if let Ok(enable) = items.get(objid.clone()) {
                            // log::warn!("Moldellist Geo: {:?}, {:?}", enable.0, geo_enable.0);
                            // log::debug!("SysModelListAfterCullinUpdateByCamera: 2");
                            if enable.0 {
                                cullings.0.push(objid.clone());
                            }
                        }
                    });
                }
            }
            // log::warn!("Moldellist: {:?}, {:?}, {:?}", vieweractive.0, liet_model.0.len(), cullings.0.len());
        });

        performance.culling = (pi_time::Instant::now() - time1).as_micros() as u32;
        
        // log::debug!("SysModelListAfterCullingTick: {:?}", pi_time::Instant::now() - time1);
    }
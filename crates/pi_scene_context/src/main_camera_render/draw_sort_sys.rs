use std::{vec};

use pi_ecs::{prelude::{Query}, query::{Changed, Or}};
use pi_ecs_macros::setup;
use pi_engine_shell::{object::ObjectID, run_stage::TSystemStageInfo};

use crate::{
    object::GameObject, 
    renderers::{
        ModelList, ModelListAfterCulling,
    },
    flags::SceneID,
    layer_mask::{LayerMask, command::SysLayerMaskCommand},
    transforms::{transform_node::{WorldMatrix}, transform_node_sys::SysWorldMatrixCalc},
    geometry::{geometry::{RenderGeometry, RenderGeometryEable}, sys_vertex_buffer_use::{SysRenderGeometryInit}},
    scene::command::SysSceneCommand,
    viewer::{ViewerGlobalPosition, ViewerViewMatrix},
    cameras::{SysViewerUpdatedForCamera, camera::CameraParam},
    bindgroup::uniform_buffer::{SysDynUnifromBufferUpdate},
};

use super::{MainCameraRenderer};

pub struct SysModelListUpdateByCamera;
impl TSystemStageInfo for SysModelListUpdateByCamera { 

}
#[setup]
impl SysModelListUpdateByCamera {
    #[system]
    pub fn sys(
        mut cameras: Query<
            GameObject,
            (ObjectID, &SceneID, &LayerMask, &CameraParam, &mut ModelList),
            // Changed<LayerMask>
        >,
        items: Query<
            GameObject,
            (ObjectID, &SceneID, &LayerMask, &RenderGeometry),
        >,
    ) {
        // log::debug!("SysModelListUpdateByCamera: ");
        cameras.iter_mut().for_each(|(camera, scene, layer, _, mut modellist)| {
            // log::debug!("SysModelListUpdateByCamera: 0");
            let mut list = vec![];
            items.iter().for_each(|(obj, iscene, ilayer, _)| {
                // log::debug!("SysModelListUpdateByCamera: 1");
                if iscene == scene && layer.include(ilayer) {
                    // log::debug!("SysModelListUpdateByCamera: 2");
                    list.push(obj.clone());
                }
            });

            modellist.0 = list;
        });
    }
}

pub struct SysModelListAfterCullinUpdateByCamera;
impl TSystemStageInfo for SysModelListAfterCullinUpdateByCamera {
    fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
        vec![
            SysViewerUpdatedForCamera::key(), SysModelListUpdateByCamera::key(), SysModelListUpdateByGeometry::key()
        ]
    }
}
#[setup]
impl SysModelListAfterCullinUpdateByCamera {
    #[system]
    pub fn sys(
        mut cameras: Query<
            GameObject,
            (ObjectID, &ModelList, &ViewerGlobalPosition, &ViewerViewMatrix, &mut ModelListAfterCulling),
            // Or<(Changed<ViewerGlobalPosition>, Changed<ViewerViewMatrix>, Changed<ModelList>)>
        >,
        items: Query<
            GameObject,
            (ObjectID, &WorldMatrix, &RenderGeometry, &RenderGeometryEable)
        >,
    ) {
        // log::debug!("SysModelListAfterCullinUpdateByCamera: ");
        cameras.iter_mut().for_each(|(camera, models, camerapos, cameraview, mut cullings)| {
            // log::debug!("SysModelListAfterCullinUpdateByCamera: 0");
            let mut list = vec![];
            models.0.iter().for_each(|objid| {
                // log::debug!("SysModelListAfterCullinUpdateByCamera: 1");
                if let Some((_, worldmat, _, geo_enable)) = items.get(objid.clone()) {
                    // log::debug!("SysModelListAfterCullinUpdateByCamera: 2");
                    if geo_enable.0 {
                        list.push(objid.clone());
                    }
                }
            });

            cullings.0 = list;
        });
    }
}

pub struct SysModelListUpdateByGeometry;
impl TSystemStageInfo for SysModelListUpdateByGeometry {
    fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
        vec![
            SysWorldMatrixCalc::key(), SysModelListUpdateByCamera::key(),
        ]
    }
}
#[setup]
impl SysModelListUpdateByGeometry {
    #[system]
    pub fn sys(
        mut cameras: Query<
            GameObject,
            (ObjectID, &SceneID, &LayerMask, &mut ModelList)
        >,
        items: Query<
            GameObject,
            (ObjectID, &SceneID, &LayerMask, &RenderGeometry, &RenderGeometryEable),
            Or<(Changed<RenderGeometry>, Changed<SceneID>, Changed<LayerMask>, Changed<RenderGeometryEable>)>
        >
    ) {
        // log::debug!("SysModelListUpdateByGeometry: ");
        items.iter().for_each(|(obj, iscene, ilayer, _, geo_enable)| {
            // log::debug!("SysModelListUpdateByGeometry: 1");
            if geo_enable.0 {
                cameras.iter_mut().for_each(|(camera, scene, layer, mut model_list)| {
                    // log::debug!("SysModelListUpdateByGeometry: 2");
                    if iscene == scene && layer.include(ilayer) {
                        // log::debug!("SysModelListUpdateByGeometry: 3");
                        if model_list.0.contains(&obj) == false {
                            model_list.0.push(obj.clone());
                        }
                    } else {
                        let len = model_list.0.len();
                        for i in 0..len {
                            if model_list.0.get(i).unwrap() == &obj {
                                model_list.0.swap_remove(i);
                                return;
                            }
                        }
                    }
                });
            }
        });
    }
}

pub struct SysModelListAfterCullinUpdateByGeometry;
impl TSystemStageInfo for SysModelListAfterCullinUpdateByGeometry {
    fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
        vec![
            SysViewerUpdatedForCamera::key(), SysWorldMatrixCalc::key(), SysModelListAfterCullinUpdateByCamera::key()
        ]
    }
}
#[setup]
impl SysModelListAfterCullinUpdateByGeometry {
    #[system]
    pub fn sys(
        mut cameras: Query<
            GameObject,
            (ObjectID, &ModelList, &ViewerGlobalPosition, &ViewerViewMatrix, &mut ModelListAfterCulling),
        >,
        items: Query<
            GameObject,
            (ObjectID, &WorldMatrix, &RenderGeometry, &RenderGeometryEable),
            Or<(Changed<RenderGeometry>, Changed<WorldMatrix>, Changed<RenderGeometryEable>)>
        >,
    ) {
        // log::debug!("SysModelListAfterCullinUpdateByGeometry: ");
        items.iter().for_each(|(obj, worldmat, _, geo_enable)| {
            // log::debug!("SysModelListAfterCullinUpdateByGeometry: 0");
            if geo_enable.0 {
                cameras.iter_mut().for_each(|(camera, models, camerapos, cameraview, mut cullings)| {
                    // log::debug!("SysModelListAfterCullinUpdateByGeometry: 1, {}", models.0.len());
                    if models.0.contains(&obj) == true && cullings.0.contains(&obj) == false {
                        // log::debug!("SysModelListAfterCullinUpdateByGeometry: 2");
                        cullings.0.push(obj.clone());
                    }
                });
            }
        });
    }
}

pub struct SysModelListAfferCullingUpdateByModelWorldMatrix;
impl TSystemStageInfo for SysModelListAfferCullingUpdateByModelWorldMatrix {
    fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
        vec![
            SysModelListUpdateByGeometry::key(), SysWorldMatrixCalc::key(), SysModelListAfterCullinUpdateByGeometry::key()
        ]
    }
}
#[setup]
impl SysModelListAfferCullingUpdateByModelWorldMatrix {
    pub fn sys(
        mut cameras: Query<
            GameObject,
            (ObjectID, &ModelList, &mut ModelListAfterCulling, &ViewerGlobalPosition, &ViewerViewMatrix),
        >,
        items: Query<
            GameObject,
            (ObjectID, &WorldMatrix),
            Changed<WorldMatrix>
        >
    ) {
        // log::debug!("SysModelListAfferCullingUpdateByModelWorldMatrix: ");
        items.iter().for_each(|(obj, worldmat)| {
            cameras.iter_mut().for_each(|(camera, modellist, cullinglist, camerapos, cameraview)| {
                // log::debug!("SysModelListAfferCullingUpdateByModelWorldMatrix: 0");
                // if modellist.0.contains(obj) {
                //     if iscene == scene && layer.include(ilayer) {
                //         if cullinglist.0.contains(obj) == false {
                //             cullinglist.0.push(obj.clone());
                //         }
                //     } else {
                //         match cullinglist.0.binary_search(obj) {
                //             Ok(i) => {
                //                 cullinglist.0.swap_remove(i);
                //             },
                //             Err(_) => {},
                //         }
                //     }
                // } else {
                //     match cullinglist.0.binary_search(obj) {
                //         Ok(i) => {
                //             cullinglist.0.swap_remove(i);
                //         },
                //         Err(_) => {},
                //     }
                // }
            });
        });
    }
}

pub struct DrawSortTick;
impl TSystemStageInfo for DrawSortTick {
    fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
        vec![
            SysDynUnifromBufferUpdate::key()
        ]
    }
}
#[setup]
impl DrawSortTick {
    #[system]
    pub fn tick(
        mut main_camera_renderers: Query<GameObject, &mut MainCameraRenderer>,
    ) {
        // log::debug!("Draw Sort Tick");
        // main_camera_renderers.iter_mut().for_each(|mut item| {
        //     item.opaque_draws.draws.sort();
        //     item.skybox_draws.draws.sort();
        //     item.transparent_draws.draws.sort();
        // });
    }
}

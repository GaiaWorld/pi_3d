use std::time::Instant;

use pi_ecs::{prelude::{Query, Commands}, query::{Changed, Or}};
use pi_ecs_macros::setup;
use pi_engine_shell::{run_stage::TSystemStageInfo, object::{ObjectID, GameObject}};

use crate::{flags::SceneID, layer_mask::LayerMask, geometry::geometry::{RenderGeometry, RenderGeometryEable}, transforms::transform_node::WorldMatrix, meshes::{model::SysRenderMatrixUpdate, abstract_mesh::AbstructMesh, Mesh}};

use super::{ModelList, ViewerGlobalPosition, ViewerViewMatrix, ModelListAfterCulling, ViewerActive, FlagModelList};

/// * ModelList 在视口参数变化时重新搜集
///   * LayerMask
// pub struct SysModelListUpdateByViewer;
// impl TSystemStageInfo for SysModelListUpdateByViewer { 
//     // fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
//     //     vec![

//     //     ]
//     // }
// }
// #[setup]
// impl SysModelListUpdateByViewer {
//     #[system]
    fn sys_update_viewer_model_list_by_viewer(
        mut viewers: Query<
            (ObjectID, &ViewerActive, &SceneID, &LayerMask, &mut ModelList),
            Or<(Changed<LayerMask>, Changed<ViewerActive>)>
        >,
        items: Query<
            (ObjectID, &SceneID, &LayerMask, &Mesh),
        >,
        mut flag_model_cmd: Commands<GameObject, FlagModelList>,
    ) {
        let time1 = Instant::now();

        // log::debug!("SysModelListUpdateByCamera: ");
        viewers.iter_mut().for_each(|(camera, vieweractive, scene, layer, mut list_model)| {
            list_model.0.clear();
            if vieweractive.0 {
                // log::debug!("SysModelListUpdateByCamera: 0");
                items.iter().for_each(|(obj, iscene, ilayer, _)| {
                    // log::debug!("SysModelListUpdateByCamera: 1");
                    if iscene == scene && layer.include(ilayer) {
                        log::debug!("SysModelListUpdateByCamera: 2");
                        list_model.0.insert(obj.clone(), obj.clone());
                    }
                });
            }
    
            flag_model_cmd.insert(camera, FlagModelList(true));
        });

        log::debug!("SysModelListUpdateByViewer: {:?}", Instant::now() - time1);
    }
// }

/// * ModelList 在Model参数变化时 移除 或 插入 Model
///   * LayerMask, RenderGeometry
// pub struct SysModelListUpdateByModel;
// impl TSystemStageInfo for SysModelListUpdateByModel { 
//     // fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
//     //     vec![

//     //     ]
//     // }
// }
// #[setup]
// impl SysModelListUpdateByModel {
//     #[system]
    pub fn sys_update_viewer_model_list_by_model(
        mut viewers: Query<
            (ObjectID, &ViewerActive, &SceneID, &LayerMask, &mut ModelList)
        >,
        items: Query<
            (ObjectID, &SceneID, &LayerMask, &Mesh),
            Changed<LayerMask>,
        >,
        mut flag_model_cmd: Commands<GameObject, FlagModelList>,
    ) {
        let time1 = Instant::now();

        items.iter().for_each(|(id_obj, iscene, ilayer, _)| {
            viewers.iter_mut().for_each(|(id_viewer, vieweractive, scene, layer, mut list_model)| {
                if vieweractive.0 {
                    if iscene == scene && layer.include(ilayer) {
                        list_model.0.insert(id_obj.clone(), id_obj);
                    } else {
                        list_model.0.remove(&id_obj);
                    }

                    flag_model_cmd.insert(id_viewer, FlagModelList(true));
                }
            });
        });

        log::debug!("SysModelListUpdateByModel: {:?}", Instant::now() - time1);
    }
// }


// /// * ModelListAfterCulling 每帧重新搜集
// pub struct SysModelListAfterCullingTick;
// impl TSystemStageInfo for SysModelListAfterCullingTick {
//     fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
//         vec![
//             SysModelListUpdateByViewer::key(), SysModelListUpdateByModel::key(), SysRenderMatrixUpdate::key()
//         ]
//     }
// }
// #[setup]
// impl SysModelListAfterCullingTick {
//     #[system]
    pub fn sys_tick_viewer_culling(
        mut viewers: Query<
            (ObjectID, &ViewerActive, &ModelList, &ViewerGlobalPosition, &ViewerViewMatrix, &mut ModelListAfterCulling),
        >,
        items: Query<
            (ObjectID, &WorldMatrix, &RenderGeometryEable)
        >,
    ) {
        let time1 = Instant::now();
        // log::debug!("SysModelListAfterCullinUpdateByCamera: ");
        viewers.iter_mut().for_each(|(id_viewer, vieweractive, liet_model, camerapos, cameraview, mut cullings)| {
            // log::debug!("SysModelListAfterCullinUpdateByCamera: 0");
            cullings.0.clear();
            if vieweractive.0 {
                liet_model.0.iter().for_each(|(objid, _)| {
                    // log::debug!("SysModelListAfterCullinUpdateByCamera: 1");
                    if let Some((_, worldmat, geo_enable)) = items.get(objid.clone()) {
                        // log::debug!("SysModelListAfterCullinUpdateByCamera: 2");
                        if geo_enable.0 {
                            cullings.0.push(objid.clone());
                        }
                    }
                });
            }
            log::warn!("Moldellist: {:?}, {:?}, {:?}", vieweractive.0, liet_model.0.len(), cullings.0.len());
        });
        
        log::debug!("SysModelListAfterCullingTick: {:?}", Instant::now() - time1);
    }
// }
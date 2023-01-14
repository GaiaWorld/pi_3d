use std::marker::PhantomData;

use pi_ecs::{prelude::{Query, Component, Commands}, query::{Changed, Or}};
use pi_ecs_macros::setup;
use pi_ecs_utils::prelude::EntityTree;
use pi_engine_shell::{run_stage::TSystemStageInfo, object::{GameObject, ObjectID}};
use pi_scene_math::coordiante_system::CoordinateSytem3;
use pi_slotmap_tree::Storage;

use crate::transforms::{command::SysTransformNodeCommand, transform_node::{LocalPosition, GlobalTransform}, transform_node_sys::SysWorldMatrixCalc};

use super::{ViewerViewMatrix, ViewerGlobalPosition, ViewerProjectionMatrix, ViewerTransformMatrix, TViewerViewMatrix, TViewerProjectMatrix};


pub(crate) struct SysViewerViewMatrixByViewCalc<T: TViewerViewMatrix + Component, S: TSystemStageInfo + 'static>(PhantomData<(T, S)>);
impl<T: TViewerViewMatrix + Component, S: TSystemStageInfo + 'static> TSystemStageInfo for SysViewerViewMatrixByViewCalc<T, S> {
    fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
        vec![
            S::key(), SysTransformNodeCommand::key(), 
        ]
    }
}
#[setup]
impl<T: TViewerViewMatrix + Component, S: TSystemStageInfo + 'static> SysViewerViewMatrixByViewCalc<T, S> {
    #[system]
    pub fn calc(
        mut query_cameras: Query<GameObject, (ObjectID, &T, &LocalPosition), Changed<T>>,
        transforms: Query<GameObject, &GlobalTransform>,
        mut view_cmd: Commands<GameObject, ViewerViewMatrix>,
        mut pos_cmd: Commands<GameObject, ViewerGlobalPosition>,
        idtree: EntityTree<GameObject>,
    ) {
        //  log::debug!("View Matrix Calc:");
        let coordsys = CoordinateSytem3::right();
        for (entity, viewcalc, l_position) in query_cameras.iter_mut() {
            match idtree.get_up(entity) {
                Some(level) => {
                    let parent_id = level.parent();
                    if let Some(parent) = transforms.get(parent_id) {
                        let (viewmatrix, pos) = viewcalc.view_matrix(&coordsys, l_position, Some(&parent));
                        view_cmd.insert(entity, viewmatrix);
                        pos_cmd.insert(entity, pos);
                    } else {
                        let (viewmatrix, pos) = viewcalc.view_matrix(&coordsys, l_position, None);
                        view_cmd.insert(entity, viewmatrix);
                        pos_cmd.insert(entity, pos);
                    }
                },
                None => {
                    let (viewmatrix, pos) = viewcalc.view_matrix(&coordsys, l_position, None);
                    view_cmd.insert(entity, viewmatrix);
                    pos_cmd.insert(entity, pos);
                },
            };
        }
    }
}

pub(crate) struct SysViewerViewMatrixUpdateByParentModify<T: TViewerViewMatrix + Component>(PhantomData<T>);
impl<T: TViewerViewMatrix + Component> TSystemStageInfo for SysViewerViewMatrixUpdateByParentModify<T> {
    fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
        vec![
            SysWorldMatrixCalc::key()
        ]
    }
}
#[setup]
impl<T: TViewerViewMatrix + Component> SysViewerViewMatrixUpdateByParentModify<T> {
    #[system]
    pub fn calc(
        mut query_cameras: Query<GameObject, (ObjectID, &T, &LocalPosition)>,
        dirty_globals: Query<GameObject, &GlobalTransform, Changed<GlobalTransform>>,
        mut view_cmd: Commands<GameObject, ViewerViewMatrix>,
        mut pos_cmd: Commands<GameObject, ViewerGlobalPosition>,
        idtree: EntityTree<GameObject>,
    ) {
        //  log::debug!("View Matrix Calc:");
        let coordsys = CoordinateSytem3::right();
        for (entity, viewcalc, l_position) in query_cameras.iter_mut() {
            match idtree.get_up(entity) {
                Some(parent_id) => {
                    let parent_id = parent_id.parent();
                    if let Some(parent) = dirty_globals.get(parent_id) {
                        let (viewmatrix, pos) = viewcalc.view_matrix(&coordsys, l_position, Some(&parent));
                        view_cmd.insert(entity, viewmatrix);
                        pos_cmd.insert(entity, pos);
                    }
                },
                None => {
                    
                },
            };
        }
    }
}

// pub(crate) struct SysViewerViewMatrixUpdateByLocalPos<T: TViewerViewMatrix + Component>(PhantomData<T>);
// impl<T: TViewerViewMatrix + Component> TSystemStageInfo for SysViewerViewMatrixUpdateByLocalPos<T> {
//     fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
//         vec![
//             SysTransformNodeCommand::key(), SysViewerViewMatrixByViewCalc::
//         ]
//     }
// }
// #[setup]
// impl<T: TViewerViewMatrix + Component> SysViewerViewMatrixUpdateByLocalPos<T> {
//     #[system]
//     pub fn calc(
//         mut viewers: Query<GameObject, (ObjectID, &T, &LocalPosition), Changed<LocalPosition>>,
//         transforms: Query<GameObject, &GlobalTransform>,
//         mut view_cmd: Commands<GameObject, ViewerViewMatrix>,
//         mut pos_cmd: Commands<GameObject, ViewerGlobalPosition>,
//         idtree: EntityTree<GameObject>,
//     ) {
//         //  log::debug!("View Matrix Calc:");
//         let coordsys = CoordinateSytem3::left();
//         for (entity, viewcalc, l_position) in viewers.iter_mut() {
//             match idtree.get_up(entity) {
//                 Some(level) => {
//                     let parent_id = level.parent();
//                     if let Some(parent) = transforms.get(parent_id) {
//                         let (viewmatrix, pos) = viewcalc.view_matrix(&coordsys, l_position, Some(&parent));
//                         view_cmd.insert(entity, viewmatrix);
//                         pos_cmd.insert(entity, pos);
//                     } else {
//                         let (viewmatrix, pos) = viewcalc.view_matrix(&coordsys, l_position, None);
//                         view_cmd.insert(entity, viewmatrix);
//                         pos_cmd.insert(entity, pos);
//                     }
//                 },
//                 None => {
//                     let (viewmatrix, pos) = viewcalc.view_matrix(&coordsys, l_position, None);
//                     view_cmd.insert(entity, viewmatrix);
//                     pos_cmd.insert(entity, pos);
//                 },
//             };
//         }
//     }
// }


pub(crate) struct SysViewerProjectionCalc<T: TViewerProjectMatrix + Component, S: TSystemStageInfo + 'static>(PhantomData<(T, S)>);
impl<T: TViewerProjectMatrix + Component, S: TSystemStageInfo> TSystemStageInfo for SysViewerProjectionCalc<T, S> {
    fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
        vec![
            S::key(), 
        ]
    }
}
#[setup]
impl<T: TViewerProjectMatrix + Component, S: TSystemStageInfo + 'static> SysViewerProjectionCalc<T, S> {
    #[system]
    pub fn calc(
        mut viewers: Query<GameObject, (ObjectID, &T), Changed<T>>,
        mut project_cmd: Commands<GameObject, ViewerProjectionMatrix>,
    ) {
        //  log::debug!("Projection Matrix Calc:");
        viewers.iter_mut().for_each(|(obj, projectcalc)| {
            let project = projectcalc.project_matrix(1.0);
            project_cmd.insert(obj, project);
        });
    }
}

pub struct SysViewerUpdated<T: TViewerViewMatrix + Component, S: TSystemStageInfo + 'static, T2: TViewerProjectMatrix + Component, S2: TSystemStageInfo + 'static>(PhantomData<(T, S, T2, S2)>);
impl<T: TViewerViewMatrix + Component, S: TSystemStageInfo + 'static, T2: TViewerProjectMatrix + Component, S2: TSystemStageInfo + 'static> TSystemStageInfo for SysViewerUpdated<T, S, T2, S2> {
    fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
        vec![
            SysViewerViewMatrixByViewCalc::<T, S>::key(), SysViewerViewMatrixUpdateByParentModify::<T>::key(), SysViewerProjectionCalc::<T2, S2>::key(), 
        ]
    }
}
#[setup]
impl<T: TViewerViewMatrix + Component, S: TSystemStageInfo + 'static, T2: TViewerProjectMatrix + Component, S2: TSystemStageInfo + 'static> SysViewerUpdated<T, S, T2, S2> {
    #[system]
    pub fn calc(
        mut viewers: Query<GameObject, (ObjectID, &T, &T2, &ViewerViewMatrix, &ViewerProjectionMatrix), Or<(Changed<ViewerViewMatrix>, Changed<ViewerProjectionMatrix>)>>,
        mut vp_cmd: Commands<GameObject, ViewerTransformMatrix>,
    ) {
        viewers.iter_mut().for_each(|(obj, _, _, view_matrix, project_matrix)| {
            // log::debug!("SysCamera Transform Matrix: p = {:?}, v = {:?}", project_matrix.0, view_matrix.0);

            // transform_matrix.0 = project_matrix.0 * view_matrix.0;
            vp_cmd.insert(obj, ViewerTransformMatrix(project_matrix.0 * view_matrix.0));

            // view_matrix.0.
            // transform_matrix.0 = view_matrix.0 * project_matrix.0;
            // log::debug!("Transform Matrix v * p {}", transform_matrix.0);
            // transform_matrix.0 = project_matrix.0 * view_matrix.0;
            // log::debug!("Transform Matrix p * v {}", transform_matrix.0);
            // view_matrix.0.mul_to(&project_matrix.0, &mut transform_matrix.0);
            // log::debug!("Transform Matrix {}", transform_matrix.0);
            // project_matrix.0.mul_to(&view_matrix.0, &mut transform_matrix.0);
            // log::debug!("Transform Matrix {}", transform_matrix.0);
            // project_matrix.0.mul_to(&view_matrix.0.transpose(), &mut transform_matrix.0);
            // log::debug!("Transform Matrix {}", transform_matrix.0);
            // project_matrix.0.transpose().mul_to(&view_matrix.0, &mut transform_matrix.0);
            // log::debug!("Transform Matrix {}", transform_matrix.0);
            // view_matrix.0.transpose().mul_to(&project_matrix.0, &mut transform_matrix.0);
            // log::debug!("Transform Matrix {}", transform_matrix.0);
            // view_matrix.0.mul_to(&project_matrix.0.transpose(), &mut transform_matrix.0);
            // log::debug!("Transform Matrix {}", transform_matrix.0);

            // let temp = Vector4::new(1., 1., 1., 1.);
            // log::debug!(">>>>>>> {}", transform_matrix.0 * temp);
            // let temp = Vector4::new(1., 1., 2., 1.);
            // log::debug!(">>>>>>> {}", transform_matrix.0 * temp);
            // let temp = Vector4::new(1., 1., -1., 1.);
            // log::debug!(">>>>>>> {}", transform_matrix.0 * temp);
            
            // transform_matrix.0.transpose_mut();
        });
    }
}


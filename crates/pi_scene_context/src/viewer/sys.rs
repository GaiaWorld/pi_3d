

use bevy::prelude::With;
use pi_engine_shell::prelude::*;
use pi_scene_math::coordiante_system::CoordinateSytem3;

use crate::{transforms::transform_node::{LocalPosition, GlobalTransform, WorldMatrix}, prelude::NodeParent};

use super::base::*;


    pub fn sys_calc_view_matrix_by_viewer<T: TViewerViewMatrix + Component>(
        mut viewers: Query<(ObjectID, &T, &LocalPosition, &mut ViewerViewMatrix, &mut ViewerGlobalPosition), Or<(Changed<T>, Changed<WorldMatrix>)>>,
        mut transforms: Query<&mut GlobalTransform>,
        childrens: Query<&NodeParent>,
    ) {
        //  log::debug!("View Matrix Calc:");
        let coordsys = CoordinateSytem3::left();
        for (entity, viewcalc, l_position, mut viewmatrix, mut viewposition) in viewers.iter_mut() {
            // log::debug!("ViewMatrix {:?}", l_position.0);
            // panic!("LocalPosition");
            if let Ok(parent) = childrens.get(entity) {
                match &parent.0 {
                    Some(parent_id) => {
                        if let Ok(mut parent) = transforms.get_mut(*parent_id) {
                            let (matrix, pos) = viewcalc.view_matrix(&coordsys, l_position, Some(&mut parent));
                            *viewmatrix = matrix;
                            *viewposition = pos;
                        } else {
                            let (matrix, pos) = viewcalc.view_matrix(&coordsys, l_position, None);
                            *viewmatrix = matrix;
                            *viewposition = pos;
                        }
                    },
                    None => {
                        let (matrix, pos) = viewcalc.view_matrix(&coordsys, l_position, None);
                        *viewmatrix = matrix;
                        *viewposition = pos;
                    },
                }
            };
        }
    }


    pub fn sys_calc_proj_matrix<T: TViewerProjectMatrix + Component>(
        mut viewers: Query<(&T, &ViewerSize, &ViewerAspect, &mut ViewerProjectionMatrix), Or<(Changed<T>, Changed<ViewerSize>, Changed<ViewerAspect>)>>,
    ) {
        //  log::debug!("Projection Matrix Calc:");
        viewers.iter_mut().for_each(|(projectcalc, viewersize, vieweraspect, mut viewprojection)| {
            let aspect = match vieweraspect {
                ViewerAspect::Auto => (viewersize.0 as f32) / (viewersize.1 as f32),
                ViewerAspect::Custom(val) => *val,
            };
            let project = projectcalc.project_matrix(aspect);
            *viewprojection = project;
        });
    }

    pub fn sys_calc_transform_matrix<T: TViewerViewMatrix + Component, T2: TViewerProjectMatrix + Component>(
        mut viewers: Query<(&T, &T2, &ViewerViewMatrix, &ViewerProjectionMatrix, &mut ViewerTransformMatrix), Or<(Changed<ViewerViewMatrix>, Changed<ViewerProjectionMatrix>)>>,
    ) {
        viewers.iter_mut().for_each(|(_, _, view_matrix, project_matrix, mut transform)| {
            // log::debug!("SysCamera Transform Matrix: p = {:?}, v = {:?}", project_matrix.0, view_matrix.0);

            // transform_matrix.0 = project_matrix.0 * view_matrix.0;
            *transform = ViewerTransformMatrix(project_matrix.0 * view_matrix.0);

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

    pub fn sys_update_viewer_uniform<T: TViewerViewMatrix + Component, T2: TViewerProjectMatrix + Component>(
        viewers: Query<
            (&BindViewer, &ViewerViewMatrix, &ViewerProjectionMatrix, &ViewerTransformMatrix, &ViewerGlobalPosition, &ViewerDirection),
            (
                Or<(
                    Changed<BindViewer>, Changed<ViewerTransformMatrix>, 
                )>,
                With<T>, With<T2>
            )
        >
    ) {
        viewers.iter().for_each(
            |(
                bind,
                viewmatrix, projmatrix, transmatrix, position, direction
            )| {
                // log::debug!("SysViewerUpdated: {:?}, {:?}", bind.0.data().offset(), bind.0.data().size());

                viewmatrix.update(bind.0.data());
                projmatrix.update(bind.0.data());
                transmatrix.update(bind.0.data());
                position.update(bind.0.data());
                direction.update(bind.0.data());
            }
        );
    }

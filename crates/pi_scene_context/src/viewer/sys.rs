use std::marker::PhantomData;

use bevy::prelude::With;
use pi_engine_shell::prelude::*;
use pi_scene_math::coordiante_system::CoordinateSytem3;
use pi_slotmap_tree::Storage;

use crate::{transforms::{transform_node::{LocalPosition, GlobalTransform, WorldMatrix}}};

use super::base::*;


    pub fn sys_calc_view_matrix_by_viewer<T: TViewerViewMatrix + Component>(
        mut query_cameras: Query<(ObjectID, &T, &LocalPosition), Or<(Changed<T>, Changed<WorldMatrix>)>>,
        mut transforms: Query<&mut GlobalTransform>,
        mut commands: Commands,
        idtree: EntityTreeMut,
    ) {
        //  log::debug!("View Matrix Calc:");
        let coordsys = CoordinateSytem3::left();
        for (entity, viewcalc, l_position) in query_cameras.iter_mut() {
            log::debug!("ViewMatrix {:?}", l_position.0);
            // panic!("LocalPosition");
            match idtree.get_up(entity) {
                Some(level) => {
                    let parent_id = level.parent();
                    if let Ok(mut parent) = transforms.get_mut(parent_id) {
                        let (viewmatrix, pos) = viewcalc.view_matrix(&coordsys, l_position, Some(&mut parent));
                        commands.entity(entity)
                            .insert(viewmatrix)
                            .insert(pos);
                    } else {
                        let (viewmatrix, pos) = viewcalc.view_matrix(&coordsys, l_position, None);
                        commands.entity(entity)
                            .insert(viewmatrix)
                            .insert(pos);
                    }
                },
                None => {
                    let (viewmatrix, pos) = viewcalc.view_matrix(&coordsys, l_position, None);
                    commands.entity(entity)
                            .insert(viewmatrix)
                            .insert(pos);
                },
            };
        }
    }


    pub fn sys_calc_proj_matrix<T: TViewerProjectMatrix + Component>(
        mut viewers: Query<(ObjectID, &T, &ViewerSize, &ViewerAspect), Or<(Changed<T>, Changed<ViewerSize>, Changed<ViewerAspect>)>>,
        mut commands: Commands,
    ) {
        //  log::debug!("Projection Matrix Calc:");
        viewers.iter_mut().for_each(|(entity, projectcalc, viewersize, vieweraspect)| {
            let aspect = match vieweraspect {
                ViewerAspect::Auto => (viewersize.0 as f32) / (viewersize.1 as f32),
                ViewerAspect::Custom(val) => *val,
            };
            let project = projectcalc.project_matrix(aspect);
            commands.entity(entity).insert(project);
        });
    }

    pub fn sys_calc_transform_matrix<T: TViewerViewMatrix + Component, T2: TViewerProjectMatrix + Component>(
        mut viewers: Query<(ObjectID, &T, &T2, &ViewerViewMatrix, &ViewerProjectionMatrix), Or<(Changed<ViewerViewMatrix>, Changed<ViewerProjectionMatrix>)>>,
        mut commands: Commands,
    ) {
        viewers.iter_mut().for_each(|(entity, _, _, view_matrix, project_matrix)| {
            // log::debug!("SysCamera Transform Matrix: p = {:?}, v = {:?}", project_matrix.0, view_matrix.0);

            // transform_matrix.0 = project_matrix.0 * view_matrix.0;
            commands.entity(entity).insert(ViewerTransformMatrix(project_matrix.0 * view_matrix.0));

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
                log::debug!("SysViewerUpdated: {:?}, {:?}", bind.0.data().offset(), bind.0.data().size());

                viewmatrix.update(bind.0.data());
                projmatrix.update(bind.0.data());
                transmatrix.update(bind.0.data());
                position.update(bind.0.data());
                direction.update(bind.0.data());
            }
        );
    }

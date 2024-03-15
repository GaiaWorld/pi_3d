
use pi_scene_shell::prelude::*;
use pi_scene_math::coordiante_system::CoordinateSytem3;

use crate::transforms::prelude::*;

use super::base::*;


    pub fn sys_calc_view_matrix_by_viewer<T: TViewerViewMatrix + Component>(
        mut viewers: Query<(ObjectID, &T, &LocalPosition, &mut ViewerViewMatrix, &mut ViewerGlobalPosition), Or<(Changed<T>, Changed<GlobalMatrix>)>>,
        mut transforms: Query<(&GlobalMatrix, &mut AbsoluteTransform)>,
        // childrens: Query<&NodeParent>,
        childrens: Query<&Up>,
        tree: EntityTree,
    ) {
        //  log::debug!("View Matrix Calc:");
        let coordsys = CoordinateSytem3::left();
        for (entity, viewcalc, l_position, mut viewmatrix, mut viewposition) in viewers.iter_mut() {
            if let Some(up) = tree.get_up(entity) {
                // up.parent()
                let parent_id = up.parent();
                if let Ok((parent, mut absolute)) = transforms.get_mut(parent_id) {
                    let iso = absolute.iso(parent.matrix());
                    let (matrix, pos) = viewcalc.view_matrix(&coordsys, l_position, Some((&parent, iso)));
                    *viewmatrix = matrix;
                    *viewposition = pos;
                } else {
                    let (matrix, pos) = viewcalc.view_matrix(&coordsys, l_position, None);
                    *viewmatrix = matrix;
                    *viewposition = pos;
                }
            }
            
            if let Ok(parent) = childrens.get(entity) {
                let parent_id = parent.parent();
                if let Ok((parent, mut absolute)) = transforms.get_mut(parent_id) {
                    let iso = absolute.iso(parent.matrix());
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


    pub fn sys_calc_proj_matrix<T: TViewerProjectMatrix + Component>(
        mut viewers: Query<(&T, &ViewerAspect, &mut ViewerProjectionMatrix), Or<(Changed<T>, Changed<ViewerAspect>)>>,
    ) {
        //  log::debug!("Projection Matrix Calc:");
        viewers.iter_mut().for_each(|(projectcalc, vieweraspect, mut viewprojection)| {
            let aspect = vieweraspect.0;
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

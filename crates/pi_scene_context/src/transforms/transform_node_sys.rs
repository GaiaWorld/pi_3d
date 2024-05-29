
use std::ops::Deref;

use pi_scene_shell::prelude::*;
use pi_scene_math::{coordiante_system::CoordinateSytem3, vector::TToolMatrix, Matrix, Rotation3, Quaternion};

use crate::{
    scene::coordinate_system::SceneCoordinateSytem3D,
    flags::*,
};

use super::transform_node::*;
use super::prelude::*;

    pub fn sys_local_euler_calc_rotation(
        localmatrixs: Query<(Entity, &LocalEulerAngles), Changed<LocalEulerAngles>>,
        mut loacl_quaternions: Query<(&mut LocalRotationQuaternion, &mut LocalRotation)>,
    ) {
        localmatrixs.iter().for_each(|(entity, euler)| {
            if let Ok((mut loacl_quaternion, mut local_rotation)) = loacl_quaternions.get_mut(entity) {
                let rotation = Rotation3::from_euler_angles(euler.0.x, euler.0.y, euler.0.z);
                let quaternion = Quaternion::from_rotation_matrix(&rotation);
                *loacl_quaternion = LocalRotationQuaternion(quaternion.quaternion().clone());
                // log::error!("loacl_quaternion from euler {:?}", (entity, loacl_quaternion));
                *local_rotation = LocalRotation(rotation);
            }
        });
    }

    pub fn sys_local_quaternion_calc_rotation(
        localmatrixs: Query<(Entity, &LocalRotationQuaternion), Changed<LocalRotationQuaternion>>,
        mut local_rotation: Query<&mut LocalRotation>,
    ) {
        localmatrixs.iter().for_each(|(entity, quat)| {
            if let Ok(mut local_rotation) = local_rotation.get_mut(entity) {
                // if quat.is_added() {
                    // log::warn!("Quaternion: {:?}", quat);
                    let rotation = Quaternion::from_quaternion(quat.0).to_rotation_matrix();
                    // log::warn!("Quaternion: Ok");
                    // *loacl_quaternion = LocalRotationQuaternion(quaternion);
                    *local_rotation = LocalRotation(rotation);
                    // log::error!("local_rotation {:?}", (entity, quat.0));
                // }
            }
        });
    }

    pub fn sys_local_matrix_calc(
        mut state: ResMut<StateTransform>,
        mut localmatrixs: Query<(Entity, &LocalPosition, &LocalScaling, &LocalRotation, &mut LocalMatrix), (Changed<LocalPosition>, Changed<LocalScaling>, Changed<LocalRotation>)>,
    ) {
        // log::warn!("LocalMatrix: ");
        let time = pi_time::Instant::now();
        localmatrixs.iter_mut().for_each(|(_entity, position, scaling, rotation, mut localmatrix)| {
            // log::warn!("LocalMatrixCalc: {:?}", entity);
            let mut matrix = Matrix::identity();
            CoordinateSytem3::matrix4_compose_rotation(&scaling.0, &rotation.0, &position.0, &mut matrix);

            // let mut affine = Matrix::identity();
            // affine.append_nonuniform_scaling_mut(&scaling.0);
            // rotation.0.to_homogeneous().mul_to(&affine, &mut matrix);
            // matrix.append_translation_mut(&position.0);
            
            // commands.entity(obj).insert(LocalMatrix(matrix, true));
            // localmatrix.0 = matrix;
            // localmatrix.1 = true;
            *localmatrix = LocalMatrix(matrix);
        });
        let time1 = pi_time::Instant::now();
        state.calc_local_time = (time1 - time).as_micros() as u32;
    }

#[derive(Clone)]
struct TmpCalcWorldMatrix {
    node: Entity,
    dirty: bool,
    matrix: Matrix,
    enable: bool,
}

pub fn sys_tree_layer_changed(
    layers: Query<(Entity, &Up), ((Changed<Layer>, Changed<Enable>, Changed<LocalMatrix>), With<TransformNodeDirty>)>,
    // mut dirtylist: ResMut<TransformDirtyRoots>,
    mut state: ResMut<StateTransform>,
    mut dirtylist: Query<&mut TransformNodeDirty>,
) {
    let time = pi_time::Instant::now();

    layers.iter().for_each(|(entity, parent)| {
        iter_dirty(entity, parent.parent(), &layers, &mut dirtylist, 0);
    });
    
    let time1 = pi_time::Instant::now();

    state.calc_world_time = (time1 - time).as_micros() as u32;
}
fn iter_dirty(
    entity: Entity,
    parent: Entity,
    layers: &Query<(Entity, &Up), ((Changed<Layer>, Changed<Enable>, Changed<LocalMatrix>), With<TransformNodeDirty>)>,
    dirtylist: &mut Query<&mut TransformNodeDirty>,
    level: usize,
) {
    if level == 512 { return; }
    if let Ok((entity, parent)) = layers.get(parent) {
        iter_dirty(entity, parent.parent(), layers, dirtylist, level + 1);
    } else {
        if let Ok(mut item) = dirtylist.get_mut(entity) {
            *item = TransformNodeDirty;
            // log::warn!("Dirty ");
        }
    }
}

    pub fn sys_world_matrix_calc(
        _query_scenes: Query<(Entity, &SceneCoordinateSytem3D)>,
        // mut nodes: Query<(Ref<LocalMatrix>, &Enable, &mut GlobalEnable, Ref<NodeParent>)>,
        mut nodes: Query<(Ticker<&LocalMatrix>, &Enable, &mut GlobalEnable, &Up)>,
        mut transforms: Query<(&mut GlobalMatrix, &mut AbsoluteTransform)>,
        mut state: ResMut<StateTransform>,
        tree: EntityTree,
        dirtylist: Query<Entity, Changed<TransformNodeDirty>>,
    ) {
        let time = pi_time::Instant::now();
        let mut level = 1;

        {
            // // log::warn!("World Matrix Calc:");
            // for (root, _) in query_scenes.iter() {
            //     let mut temp_ids: Vec<TmpCalcWorldMatrix> = vec![];
            //     if let Ok(childs) = parents.get(root) {
            //         childs.iter().for_each(|child| {
            //             let tmp = calc_world_root(
            //                 &mut nodes,
            //                 &mut transforms,
            //                 *child,
            //             );

            //             if let Ok(childs) = parents.get(tmp.node) {
            //                 childs.iter().for_each(|child| {
            //                     calc_world_one(
            //                         *child,
            //                         &mut nodes,
            //                         &mut transforms,
            //                         &mut temp_ids,
            //                         &tmp,
            //                     );
            //                 });
            //             }
            //         });
            //     }
                
            //     calc_world(
            //         &mut nodes,
            //         &mut transforms,
            //         & parents,
            //         temp_ids
            //     );
            // }
        }

        {
            dirtylist.iter().for_each(|child| {
                let mut temp_ids: Vec<TmpCalcWorldMatrix> = vec![];

                let tmp = if let Some(parent) = tree.get_up(child) {
                    if let (Ok((transform, _)), Ok((_, _, penable, _))) = (transforms.get(parent.parent()), nodes.get(parent.parent())) {
                        calc_world_root_bytree( penable.0, &transform.matrix.clone(), &mut nodes,  &mut transforms,  child, )
                    }else {
                        calc_world_root_bytree( true, &Matrix::identity(), &mut nodes,  &mut transforms,  child, )
                    }
                } else {
                    calc_world_root_bytree( true, &Matrix::identity(), &mut nodes,  &mut transforms,  child, )
                };

                if let Some(node_children_head) = tree.get_down(tmp.node) {
                    tree.iter(node_children_head.head()).for_each(|child| {
                        calc_world_one_bytree(
                            child,
                            &mut nodes,
                            &mut transforms,
                            &mut temp_ids,
                            &tmp,
                        );
                    });
                }

                let templevel = calc_world_bytree(
                    &mut nodes,
                    &mut transforms,
                    &tree,
                    temp_ids
                );

                level = level.max(templevel);
            });
        }

        let time1 = pi_time::Instant::now();

        state.max_level = level as u32;
        state.calc_world_time += (time1 - time).as_micros() as u32;
        // log::warn!("World Matrix Calc: {:?}", time1 - time);
    }

    pub fn sys_world_matrix_calc2(
        // query_scenes: Query<(Entity, &SceneCoordinateSytem3D)>,
        // mut nodes: Query<(&mut LocalMatrix, &Enable, &mut GlobalEnable)>,
        // mut transforms: Query<&mut GlobalTransform>,
        // parents: Query<&NodeChilds>,
        // node_parent_changes: Query<&NodeParent, Changed<NodeParent>>,
    ) {
        // let time = pi_time::Instant::now();

        // // log::debug!("World Matrix Calc:");
        // for (root, _) in query_scenes.iter() {
        //     let mut temp_ids: Vec<(Entity, bool, Matrix, bool)> = vec![];
        //     let mut idflag: usize = 0;
        //     tree.iter(root).for_each(|entity| {
        //         let tmp = calc_world_root(
        //             &mut nodes,
        //             &mut transforms,
        //             entity,
        //         );
                
        //         match tree.get_down(p_id) {
        //             Some(node_children_head) => {
        //                 let node_children_head = node_children_head.head.0;
        //                 tree.iter(node_children_head).for_each(|entity| {
        //                     idflag += 1;
        //                     if idflag % 2 == 1 {
        //                         // log::warn!("Calc WM: {:?}", entity);
        //                         calc_world_one(
        //                             &mut nodes,
        //                             &mut transforms,
        //                             &mut temp_ids,
        //                             entity,
        //                             p_dirty, p_enable,
        //                             &p_m
        //                         );
        //                     }

        //                 });
        //             },
        //             None => {
        //             },
        //         }
        //     });

        //     calc_world(
        //         &mut nodes,
        //         &mut transforms,
        //         & tree,
        //         temp_ids
        //     );
        // }

        // let time1 = pi_time::Instant::now();
        // log::debug!("World Matrix Calc2: {:?}", time1 - time);
    }


fn _calc_world_one(
    entity: Entity,
    nodes: &mut Query<(Ticker<&LocalMatrix> , &Enable, &mut GlobalEnable, &Up)>,
    transforms: &mut Query<(&mut GlobalMatrix, &mut AbsoluteTransform)>,
    temp_list: &mut Vec<TmpCalcWorldMatrix>,
    tmp: &TmpCalcWorldMatrix,
) {
    match (nodes.get_mut(entity), transforms.get_mut(entity)) {
        (Ok((lmatrix, enable, mut globalenable, _parent)), Ok((mut gtransform, mut absolute))) => {
            let mut resultenable = enable.bool() && tmp.enable;

            let dirty = tmp.dirty || lmatrix.is_changed();
    
            // log::warn!(">>>>> calc_world_one {:?}", lmatrix.0);
            if dirty {
                let ( transform, flag) = GlobalMatrix::calc(&tmp.matrix, &lmatrix);
                resultenable = resultenable && flag;
                *gtransform = transform;
                absolute.reset_while_world_matrix_update();
            };

            if globalenable.0 != resultenable {
                globalenable.0 = resultenable;
            }
            temp_list.push(TmpCalcWorldMatrix { node: entity, dirty, matrix: gtransform.matrix.clone(), enable: resultenable });
        },
        (_, _) => {
            
        },
    }
}

fn calc_world_bytree(
    nodes: &mut Query<(Ticker<&LocalMatrix>, &Enable, &mut GlobalEnable, &Up)>,
    transforms: &mut Query<(&mut GlobalMatrix, &mut AbsoluteTransform)>,
    tree: &EntityTree,
    mut temp_ids: Vec<TmpCalcWorldMatrix>
) -> u32 {
    // 广度优先遍历 - 最大遍历到深度 65535
    let max = 65535;
    let mut deep = 0;
    loop {
        let mut temp_list = vec![];
        if temp_ids.len() > 0 && deep < max {
            temp_ids.into_iter().for_each(|tmp| {
                if let Some(node_children_head) = tree.get_down(tmp.node) {
                    tree.iter(node_children_head.head()).for_each(|child| {
                        calc_world_one_bytree(
                            child,
                            nodes,
                            transforms,
                            &mut temp_list,
                            &tmp
                        );
                    });
                }
            });
            deep += 1;
        } else {
            break;
        }
        temp_ids = temp_list;
    }

    return deep;
}

fn calc_world_one_bytree(
    entity: Entity,
    nodes: &mut Query<(Ticker<&LocalMatrix>, &Enable, &mut GlobalEnable, &Up)>,
    transforms: &mut Query<(&mut GlobalMatrix, &mut AbsoluteTransform)>,
    temp_list: &mut Vec<TmpCalcWorldMatrix>,
    tmp: &TmpCalcWorldMatrix,
) {
    match (nodes.get_mut(entity), transforms.get_mut(entity)) {
        (Ok((lmatrix, enable, mut globalenable, _parent)), Ok((mut gtransform, mut absolute))) => {
            let mut resultenable = enable.bool() && tmp.enable;

            
            let dirty = tmp.dirty || lmatrix.is_changed();
    
    // let r = lmatrix.deref();
            // log::warn!(">>>>> calc_world_one 2 {:?}", lmatrix.0);
            if dirty {
                let ( transform, flag) = GlobalMatrix::calc(&tmp.matrix, lmatrix.deref());
                resultenable = resultenable && flag;
                *gtransform = transform;
                absolute.reset_while_world_matrix_update();
            };

            if globalenable.0 != resultenable {
                globalenable.0 = resultenable;
            }
            temp_list.push(TmpCalcWorldMatrix { node: entity, dirty, matrix: gtransform.matrix.clone(), enable: globalenable.0 });
        },
        (_, _) => {
            
        },
    }
}

fn calc_world_root_bytree(
    penable: bool,
    p_m: &Matrix,
    nodes: &mut Query<(Ticker<&LocalMatrix>, &Enable, &mut GlobalEnable, &Up)>,
    transforms: &mut Query<(&mut GlobalMatrix, &mut AbsoluteTransform)>,
    entity: Entity,
) -> TmpCalcWorldMatrix {
    match (nodes.get_mut(entity), transforms.get_mut(entity)) {
        (Ok((lmatrix, enable, mut globalenable, _parent)), Ok((mut gtransform, mut absolute))) => {
            let mut resultenable = enable.bool() && penable;

            let dirty = lmatrix.is_changed();

            if dirty {
                // log::debug!(">>>>> calc_world_one 4");
                let (transform, flag) = GlobalMatrix::calc(p_m, &lmatrix);
                resultenable = resultenable && flag;
                *gtransform = transform;
                absolute.reset_while_world_matrix_update();
            }

            if globalenable.0 != resultenable {
                globalenable.0 = resultenable;
            }
            TmpCalcWorldMatrix {
                node: entity,
                dirty,
                matrix: gtransform.matrix.clone(),
                enable: globalenable.0
            }
        },
        (_, _) => {
            // log::debug!(">>>>> WorldMatrixCalc Root");
            // (entity, false, Matrix::identity(), true)
            TmpCalcWorldMatrix {
                node: entity,
                dirty: false,
                matrix: Matrix::identity(),
                enable: true
            }
        },
    }
}

pub fn sys_dispose_about_transform_node(
    items: Query<(Entity, &DisposeReady, &TransformNode), Changed<DisposeReady>>,
    mut _disposereadylist: ResMut<ActionListDisposeReadyForRef>,
    mut disposecanlist: ResMut<ActionListDisposeCan>,
) {
    items.iter().for_each(|(entity, state, _)| {
        if state.0 == false { return }

        disposecanlist.push(OpsDisposeCan::ops(entity));
    });
}
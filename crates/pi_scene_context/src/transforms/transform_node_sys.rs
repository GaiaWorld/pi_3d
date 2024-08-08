
use pi_scene_shell::prelude::*;
use pi_scene_math::{coordiante_system::CoordinateSytem3, vector::TToolMatrix, Matrix, Rotation3, Quaternion};

use crate::{
    scene::coordinate_system::SceneCoordinateSytem3D,
    flags::*,
};

use super::transform_node::*;
use super::prelude::*;

    pub fn sys_local_euler_calc_rotation(
        changed: ComponentChanged<LocalEulerAngles>,
        localmatrixs: Query<&LocalEulerAngles>,
        mut loacl_quaternions: Query<(&mut LocalRotationQuaternion, &mut LocalRotation)>,
    ) {
        changed.iter().for_each(|entity| {
            if let (Ok(euler), Ok((mut loacl_quaternion, mut local_rotation))) = (localmatrixs.get(*entity), loacl_quaternions.get_mut(*entity)) {
                let rotation = Rotation3::from_euler_angles(euler.0.x, euler.0.y, euler.0.z);
                let quaternion = Quaternion::from_rotation_matrix(&rotation);
                *loacl_quaternion = LocalRotationQuaternion(quaternion.quaternion().clone());
                // log::error!("loacl_quaternion from euler {:?}", (entity, loacl_quaternion));
                *local_rotation = LocalRotation(rotation);
            }
        });
    }

    pub fn sys_local_quaternion_calc_rotation(
        changed: ComponentChanged<LocalRotationQuaternion>,
        localmatrixs: Query<&LocalRotationQuaternion>,
        mut local_rotation: Query<(&mut LocalRotation)>,
        changes: ComponentChanged<LocalScaling>,
        changes2: ComponentChanged<LocalPosition>,
        mut localflags: Query< &mut FlagLocalMatrix>,
    ) {
        changed.iter().for_each(|entity| {
            if let (Ok(quat), Ok(mut local_rotation)) = (localmatrixs.get(*entity), local_rotation.get_mut(*entity)) {
                // log::warn!("Quaternion: {:?}", quat);
                let rotation = Quaternion::from_quaternion(quat.0).to_rotation_matrix();
                // log::warn!("Quaternion: Ok");
                // *loacl_quaternion = LocalRotationQuaternion(quaternion);
                *local_rotation = LocalRotation(rotation);
                if let Ok(mut flag) = localflags.get_mut(*entity) {
                    *flag = FlagLocalMatrix;
                }
            }
        });
        changes.iter().for_each(|entity| {
            if let Ok(mut flag) = localflags.get_mut(*entity) {
                *flag = FlagLocalMatrix;
            }
        });
        changes2.iter().for_each(|entity| {
            if let Ok(mut flag) = localflags.get_mut(*entity) {
                *flag = FlagLocalMatrix;
            }
        });
    }

    pub fn sys_local_matrix_calc(
        mut state: ResMut<StateTransform>,
        changes: ComponentChanged<FlagLocalMatrix>,
        mut localmatrixs: Query<(Entity, &LocalPosition, &LocalScaling, &LocalRotation, &mut LocalMatrix)>,
    ) {
        // log::warn!("LocalMatrix: ");
        // let time = pi_time::Instant::now();
        changes.iter().for_each(|entity| {
            if let Ok((_entity, position, scaling, rotation, mut localmatrix)) = localmatrixs.get_mut(*entity) {
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
            }
        });
        // let time1 = pi_time::Instant::now();
        // state.calc_local_time = (time1 - time).as_micros() as u32;
    }

#[derive(Clone)]
struct TmpCalcWorldMatrix {
    node: Entity,
    dirty: bool,
    matrix: Matrix,
    enable: bool,
}

pub fn sys_tree_layer_changed(
    changes0: ComponentChanged<Layer>,
    changes1: ComponentChanged<Enable>,
    changes2: ComponentChanged<LocalMatrix>,
    adds0: ComponentAdded<Layer>,
    adds1: ComponentAdded<Enable>,
    adds2: ComponentAdded<LocalMatrix>,

    mut layers: Query<(Entity, &mut TransformNodeDirty)>,
    // mut dirtylist: ResMut<TransformDirtyRoots>,
    mut state: ResMut<StateTransform>,
    tree: EntityTree,
) {
    // let time = pi_time::Instant::now();

    let changes = changes0.iter().chain(changes1.iter()).chain(changes2.iter())
        .chain(adds0.iter()).chain(adds1.iter()).chain(adds2.iter());

    changes.for_each(|entity| {
        if let Ok((entity, mut item)) = layers.get_mut(*entity) {
            *item = TransformNodeDirty(true);
        }
    });

    let changes = changes0.iter().chain(changes1.iter()).chain(changes2.iter())
        .chain(adds0.iter()).chain(adds1.iter()).chain(adds2.iter());
    changes.for_each(|entity| {
        if let Ok((entity, mut item)) = layers.get_mut(*entity) {
            if let Some(down) = tree.get_down(entity) {
                tree.iter(down.head()).for_each(|child| {
                    iter_dirty( child, &mut layers, 0, &tree);
                });
            }
        }
    });
    
    // let time1 = pi_time::Instant::now();
    // state.calc_world_time = (time1 - time).as_micros() as u32;
}

#[inline(never)]
fn iter_dirty(
    child: Entity,
    layers: &mut Query<(Entity, &mut TransformNodeDirty)>,
    level: usize,
    tree: &EntityTree,
) {
    if level == 512 { return; }
    if let Ok((entity, mut item)) = layers.get_mut(child) {
        *item = TransformNodeDirty(false);
        if let Some(down) = tree.get_down(child) {
            tree.iter(down.head()).for_each(|child| {
                iter_dirty( child, layers, level + 1, &tree);
            });
        }
    }
}

    pub fn sys_world_matrix_calc(
        _query_scenes: Query<(Entity, &SceneCoordinateSytem3D)>,
        // mut nodes: Query<(Ref<LocalMatrix>, &Enable, &mut GlobalEnable, Ref<NodeParent>)>,
        mut nodes: Query<(Ref<LocalMatrix>, &Enable, &mut GlobalEnable, &Up)>,
        mut transforms: Query<(&mut GlobalMatrix, &mut AbsoluteTransform)>,
        mut state: ResMut<StateTransform>,
        tree: EntityTree,
        changes: ComponentChanged<TransformNodeDirty>,
        adds: ComponentAdded<TransformNodeDirty>,
        dirtyflags: Query<&TransformNodeDirty>,
    ) {
        // let time = pi_time::Instant::now();
        let mut level = 1;

        let changes = changes.iter().chain(adds.iter());
        {
            changes.for_each(|child| {
                let child = *child;
                let mut temp_ids: Vec<TmpCalcWorldMatrix> = vec![];

                if let Ok(flag) = dirtyflags.get(child) {
                    if flag.0 == false {
                        return;
                    }
                } else {
                    return;
                }

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

        // let time1 = pi_time::Instant::now();

        state.max_level = level as u32;
        // state.calc_world_time += (time1 - time).as_micros() as u32;
        // log::warn!("World Matrix Calc: {:?}", time1 - time);
    }

    pub fn sys_world_matrix_calc2(
    ) {
    }


#[inline(never)]
fn _calc_world_one(
    entity: Entity,
    nodes: &mut Query<(Ref<LocalMatrix>, &Enable, &mut GlobalEnable, &Up)>,
    transforms: &mut Query<(&mut GlobalMatrix, &mut AbsoluteTransform)>,
    temp_list: &mut Vec<TmpCalcWorldMatrix>,
    tmp: &TmpCalcWorldMatrix,
) {
    match (nodes.get_mut(entity), transforms.get_mut(entity)) {
        (Ok((lmatrix, enable, mut globalenable, _parent)), Ok((mut gtransform, mut absolute))) => {
            let mut resultenable = enable.bool() && tmp.enable;

            let dirty = tmp.dirty || lmatrix.is_changed();
    
            // log::warn!(">>>>> calc_world_one {:?}", lmatrix.1);
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

#[inline(never)]
fn calc_world_bytree(
    nodes: &mut Query<(Ref<LocalMatrix>, &Enable, &mut GlobalEnable, &Up)>,
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

#[inline(never)]
fn calc_world_one_bytree(
    entity: Entity,
    nodes: &mut Query<(Ref<LocalMatrix>, &Enable, &mut GlobalEnable, &Up)>,
    transforms: &mut Query<(&mut GlobalMatrix, &mut AbsoluteTransform)>,
    temp_list: &mut Vec<TmpCalcWorldMatrix>,
    tmp: &TmpCalcWorldMatrix,
) {
    match (nodes.get_mut(entity), transforms.get_mut(entity)) {
        (Ok((lmatrix, enable, mut globalenable, _parent)), Ok((mut gtransform, mut absolute))) => {
            let mut resultenable = enable.bool() && tmp.enable;

            
            let dirty = tmp.dirty || lmatrix.is_changed();
    
            // log::warn!(">>>>> calc_world_one {:?}", lmatrix.1);
            if dirty {
                let ( transform, flag) = GlobalMatrix::calc(&tmp.matrix, &lmatrix);
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

#[inline(never)]
fn calc_world_root_bytree(
    penable: bool,
    p_m: &Matrix,
    nodes: &mut Query<(Ref<LocalMatrix>, &Enable, &mut GlobalEnable, &Up)>,
    transforms: &mut Query<(&mut GlobalMatrix, &mut AbsoluteTransform)>,
    entity: Entity,
) -> TmpCalcWorldMatrix {
    match (nodes.get_mut(entity), transforms.get_mut(entity)) {
        (Ok((lmatrix, enable, mut globalenable, _parent)), Ok((mut gtransform, mut absolute))) => {
            let mut resultenable = enable.bool() && penable;

            let dirty = lmatrix.is_changed();

            if dirty {
                // log::debug!(">>>>> GlobalTransform 0");
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
    changes: ComponentChanged<DisposeReady>,
    items: Query<(Entity, &DisposeReady, &TransformNode)>,
    mut _disposereadylist: ResMut<ActionListDisposeReadyForRef>,
    mut disposecanlist: ResMut<ActionListDisposeCan>,
) {
    changes.iter().for_each(|entity| {
        if let Ok((entity, state, _)) = items.get(*entity) {
            if state.0 == false { return }
    
            disposecanlist.push(OpsDisposeCan::ops(entity));
        }
    });
}

use pi_scene_shell::prelude::*;
use pi_scene_math::{Matrix, Quaternion};

use crate::{
    scene::coordinate_system::SceneCoordinateSytem3D,
    flags::*,
};

use super::transform_node::*;
use super::prelude::*;

    pub fn sys_local_euler_calc_rotation(
        changed: ComponentChanged<LocalEulerAngles>,
        localeulers: Query<&LocalEulerAngles>,
        mut loacl_quaternions: Query<(&mut LocalRotationQuaternion, &mut LocalRotation)>,
    ) {
        changed.iter().for_each(|entity| {
            if let (Ok(euler), Ok((mut loacl_quaternion, mut local_rotation))) = (localeulers.get(*entity), loacl_quaternions.get_mut(*entity)) {
                CoordinateSytem3::rotation_matrix_from_euler_angles_toref(euler.0.x, euler.0.y, euler.0.z, &mut local_rotation.0);
                CoordinateSytem3::quaternion_from_rotation(&mut loacl_quaternion.0, &local_rotation.0);
                // log::error!("loacl_quaternion from euler {:?}", (entity, loacl_quaternion));
            }
        });
    }

    pub fn sys_local_quaternion_calc_rotation(
        changes: ComponentChanged<LocalScaling>,
        changes2: ComponentChanged<LocalPosition>,
        changed: ComponentChanged<LocalRotationQuaternion>,
        localmatrixs: Query<&LocalRotationQuaternion>,
        mut local_rotation: Query<&mut LocalRotation>,
        mut localflags: Query< &mut FlagLocalMatrix>,
        entitysets: Res<EntityFilterForComponentChanged>,
    ) {
        let mut entities = entitysets.pop();
        changed.iter().for_each(|entity| {
            if !entities.insert(entity) { return; }
            if let (Ok(quat), Ok(mut local_rotation)) = (localmatrixs.get(*entity), local_rotation.get_mut(*entity)) {
                // log::warn!("Quaternion: {:?}", quat);
                CoordinateSytem3::quaternion_to_rotation(&quat.0, &mut local_rotation.0);
                // log::warn!("Quaternion: Ok");
                // *loacl_quaternion = LocalRotationQuaternion(quaternion);
                if let Ok(mut flag) = localflags.get_mut(*entity) {
                    *flag = FlagLocalMatrix;
                }
            }
        });
        entitysets.push(entities);
        let mut entities = entitysets.pop();
        changes.iter().chain(changes2.iter()).for_each(|entity| {
            if !entities.insert(entity) { return; }
            if let Ok(mut flag) = localflags.get_mut(*entity) {
                *flag = FlagLocalMatrix;
            }
        });
        entitysets.push(entities);
    }

    pub fn sys_local_matrix_calc(
        mut performance: ResMut<Performance>,
        changes: ComponentChanged<FlagLocalMatrix>,
        mut localmatrixs: Query<(Entity, &LocalPosition, &LocalScaling, &LocalRotationQuaternion, &mut LocalMatrix)>,
        entitysets: Res<EntityFilterForComponentChanged>,
    ) {
        // log::warn!("LocalMatrix: ");
        if performance.debug { performance.t_worldmatrix = pi_time::Instant::now(); }

        let mut entities = entitysets.pop();
        // changes.iter().for_each(|entity| {
        //     entities.insert(*entity);
        // });
        changes.iter().for_each(|entity| {
            if !entities.insert(entity) { return; }
            if let Ok((_entity, position, scaling, quat, mut localmatrix)) = localmatrixs.get_mut(*entity) {
                // log::warn!("LocalMatrixCalc: {:?}", entity);
                CoordinateSytem3::matrix4_compose_quaternion(&scaling.0, &Quaternion::from_quaternion(quat.0), &position.0, &mut localmatrix.0);
                // CoordinateSytem3::matrix4_compose_rotation(&scaling.0, &rotation.0, &position.0, &mut localmatrix.0);
            }
        });

        entitysets.push(entities);
        if performance.debug { performance.worldmatrix = (pi_time::Instant::now() - performance.t_worldmatrix).as_micros() as u32; }
    }

pub struct TmpCalcWorldMatrix {
    node: Entity,
    dirty: bool,
    matrix: RecyclableMatrix,
    enable: bool,
}

#[derive(Resource, Default, Deref, DerefMut)]
pub struct TmpTransformWorldCalc0(Vec<TmpCalcWorldMatrix>);
impl MemSize for TmpTransformWorldCalc0 {
    fn memsize(&self) -> usize {
        self.0.capacity() * 76
    }
}

#[derive(Resource, Default, Deref, DerefMut)]
pub struct TmpTransformWorldCalc1(Vec<TmpCalcWorldMatrix>);
impl MemSize for TmpTransformWorldCalc1 {
    fn memsize(&self) -> usize {
        self.0.capacity() * 76
    }
}

pub fn sys_transform_dirty(
    add0: ComponentAdded<TransformNodeParent>,
    changes0: ComponentChanged<TransformNodeParent>,
    changes1: ComponentChanged<Enable>,
    changes2: ComponentChanged<LocalMatrix>,

    mut layers: Query<&mut TransformNodeDirty>,
    tree: EntityTree,
    entitysets: Res<EntityFilterForComponentChanged>,
    // mut performance: ResMut<Performance>,
) {
    // performance.systems.push(String::from("sys_transform_dirty"));

    let changes = changes0.iter().chain(add0.iter()).chain(changes1.iter()).chain(changes2.iter());
    let mut temp = vec![];
    let mut entities = entitysets.pop();
    // changes0.iter().for_each(|entity| {
    //     entities.insert(*entity);
    // });
    // changes1.iter().for_each(|entity| {
    //     entities.insert(*entity);
    // });
    // changes2.iter().for_each(|entity| {
    //     entities.insert(*entity);
    // });
    changes.for_each(|entity| {
        if !entities.insert(entity) { return; }
        temp.push(*entity);
        if let Ok(mut item) = layers.get_mut(*entity) {
            *item = TransformNodeDirty(true);
        }
    });

    entitysets.push(entities);
    let mut entities = entitysets.pop();
    temp.iter().for_each(|entity| {
        if !entities.insert(entity) { return; }
        if let Ok(mut _item) = layers.get_mut(*entity) {
            if let Some(down) = tree.get_down(*entity) {
                tree.iter(down.head()).for_each(|child| {
                    iter_dirty( child, &mut layers, 0, &tree);
                });
            }
        }
    });
    entitysets.push(entities);
}

fn iter_dirty(
    child: Entity,
    layers: &mut Query<&mut TransformNodeDirty>,
    level: usize,
    tree: &EntityTree,
) {
    if level == 512 { return; }
    if let Ok(mut item) = layers.get_mut(child) {
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
        mut nodes: Query<(Ref<LocalMatrix>, &Enable, &mut GlobalEnable, &Up, &mut GlobalMatrix, &mut AbsoluteTransform)>,
        mut state: ResMut<StateTransform>,
        tree: EntityTree,
        changes: ComponentChanged<TransformNodeDirty>,
        dirtyflags: Query<&TransformNodeDirty>,
        mut temp0: ResMut<TmpTransformWorldCalc0>,
        mut temp1: ResMut<TmpTransformWorldCalc1>,
        mut performance: ResMut<Performance>,
        matrixpool: Res<ResMatrixPool>,
        entitysets: Res<EntityFilterForComponentChanged>,
    ) {
        // performance.systems.push(String::from("sys_world_matrix_calc"));
        if performance.debug { performance.t_worldmatrix = pi_time::Instant::now(); }

        let mut entities = entitysets.pop();
        // changes.iter().for_each(|entity| {
        //     entities.insert(*entity);
        // });
        let mut level = 1;
        {
            changes.iter().for_each(|child| {
                let child = *child;

                if !entities.insert(&child) { return; }
                if let Ok(flag) = dirtyflags.get(child) {
                    if flag.0 == false {
                        return;
                    }
                } else {
                    return;
                }
                temp0.clear();
                temp1.clear();

                let tmp = if let Some(parent) = tree.get_up(child) {
                    // if let (Ok((transform, _)), Ok((_, _, penable, _))) = (transforms.get(parent.parent()), nodes.get(parent.parent())) {
                    if let Ok((_, _, penable, _, transform, _, )) = nodes.get(parent.parent()) {
                        // log::error!("{:?}", (parent.parent(), penable.0));
                        calc_world_root_bytree( penable.0, &transform.matrix.clone(), &mut nodes,  child, &matrixpool)
                    }else {
                        calc_world_root_bytree( true, &Matrix::identity(), &mut nodes,  child, &matrixpool)
                    }
                } else {
                    calc_world_root_bytree( true, &Matrix::identity(), &mut nodes,  child, &matrixpool)
                };

                if let Some(node_children_head) = tree.get_down(tmp.node) {
                    tree.iter(node_children_head.head()).for_each(|child| {
                        calc_world_one_bytree(
                            child,
                            &mut nodes,
                            &mut temp0,
                            &tmp, &matrixpool,
                        );
                    });
                }

                let templevel = calc_world_bytree(
                    &mut nodes,
                    &tree,
                    &mut temp0,
                    &mut temp1, &matrixpool,
                );

                level = level.max(templevel);
            });
        }

        state.max_level = level as u32;
        
        entitysets.push(entities);
        if performance.debug { performance.worldmatrix += (pi_time::Instant::now() - performance.t_worldmatrix).as_micros() as u32; }
    }

fn _calc_world_one(
    entity: Entity,
    nodes: &mut Query<(Ref<LocalMatrix>, &Enable, &mut GlobalEnable, &Up, &mut GlobalMatrix, &mut AbsoluteTransform)>,
    temp_list: &mut Vec<TmpCalcWorldMatrix>,
    tmp: &TmpCalcWorldMatrix,
    matrixpool: &ResMatrixPool,
) {
    if let Ok((lmatrix, enable, mut globalenable, _parent, mut gtransform, mut absolute)) = nodes.get_mut(entity) {
        let mut resultenable = enable.bool() && tmp.enable;

        let dirty = tmp.dirty || lmatrix.is_changed();
        // log::error!("{:?}", (entity, enable.bool(), tmp.enable));

        // log::warn!(">>>>> calc_world_one {:?}", lmatrix.1);
        if dirty {
            let flag = gtransform.calc(tmp.matrix.as_ref(), &lmatrix);
            resultenable = resultenable && flag;
            absolute.reset_while_world_matrix_update();
        };

        if globalenable.0 != resultenable {
            globalenable.0 = resultenable;
        }
        let mut matrix = matrixpool.identity();
        matrix.as_mut().clone_from(&gtransform.matrix);
        temp_list.push(TmpCalcWorldMatrix { node: entity, dirty, matrix, enable: resultenable });
    }
}

fn calc_world_bytree<'a>(
    nodes: &mut Query<(Ref<LocalMatrix>, &Enable, &mut GlobalEnable, &Up, &mut GlobalMatrix, &mut AbsoluteTransform)>,
    tree: &EntityTree,
    mut temp0: &'a mut Vec<TmpCalcWorldMatrix>,
    mut temp1: &'a mut Vec<TmpCalcWorldMatrix>,
    matrixpool: &ResMatrixPool,
) -> u32 {
    // 广度优先遍历 - 最大遍历到深度 1024
    let max = 1024;
    let mut deep = 0;
    loop {
        // let mut temp_list = vec![];
        if temp0.len() > 0 && deep < max {
            temp0.drain(..).for_each(|tmp| {
                if let Some(node_children_head) = tree.get_down(tmp.node) {
                    tree.iter(node_children_head.head()).for_each(|child| {
                        calc_world_one_bytree(
                            child,
                            nodes,
                            temp1,
                            &tmp,
                            matrixpool
                        );
                    });
                }
            });
            deep += 1;
        } else {
            break;
        }
        (temp0, temp1) = (temp1, temp0);
    }

    temp0.clear();
    temp1.clear();

    return deep;
}

fn calc_world_one_bytree(
    entity: Entity,
    nodes: &mut Query<(Ref<LocalMatrix>, &Enable, &mut GlobalEnable, &Up, &mut GlobalMatrix, &mut AbsoluteTransform)>,
    temp_list: &mut Vec<TmpCalcWorldMatrix>,
    tmp: &TmpCalcWorldMatrix,
    matrixpool: &ResMatrixPool,
) {
    if let Ok((lmatrix, enable, mut globalenable, _parent, mut gtransform, mut absolute)) = nodes.get_mut(entity) {
        let mut resultenable = enable.bool() && tmp.enable;

        let dirty = tmp.dirty || lmatrix.is_changed();

        // log::error!("{:?}", (entity, enable.bool(), tmp.enable));
        // log::warn!(">>>>> calc_world_one {:?}", lmatrix.1);
        if dirty {
            let flag = gtransform.calc(tmp.matrix.as_ref(), &lmatrix);
            resultenable = resultenable && flag;
            absolute.reset_while_world_matrix_update();
        };

        if globalenable.0 != resultenable {
            globalenable.0 = resultenable;
        }
        let mut matrix = matrixpool.identity();
        matrix.as_mut().clone_from(&gtransform.matrix);
        temp_list.push(TmpCalcWorldMatrix { node: entity, dirty, matrix, enable: globalenable.0 });
    }
}

fn calc_world_root_bytree(
    penable: bool,
    p_m: &Matrix,
    nodes: &mut Query<(Ref<LocalMatrix>, &Enable, &mut GlobalEnable, &Up, &mut GlobalMatrix, &mut AbsoluteTransform)>,
    entity: Entity,
    matrixpool: &ResMatrixPool,
) -> TmpCalcWorldMatrix {
    if let Ok((lmatrix, enable, mut globalenable, _parent, mut gtransform, mut absolute)) = nodes.get_mut(entity) {
        let mut resultenable = enable.bool() && penable;
        // log::error!("{:?}", (entity, enable.bool(), penable));

        let dirty = true; // lmatrix.is_changed();

        if dirty {
            // log::debug!(">>>>> GlobalTransform 0");
            let flag = gtransform.calc(p_m, &lmatrix);
            resultenable = resultenable && flag;
            absolute.reset_while_world_matrix_update();
        }

        if globalenable.0 != resultenable {
            globalenable.0 = resultenable;
        }
        let mut matrix = matrixpool.identity();
        matrix.as_mut().clone_from(&gtransform.matrix);
        TmpCalcWorldMatrix {
            node: entity,
            dirty,
            matrix,
            enable: globalenable.0
        }
    } else {
        let matrix = matrixpool.identity();
        TmpCalcWorldMatrix {
            node: entity,
            dirty: false,
            matrix,
            enable: true
        }
    }
}

pub fn sys_dispose_about_transform_node(
    // changes: ComponentChanged<DisposeReady>,
    // items: Query<(Entity, &DisposeReady, &TransformNode)>,
    // mut _disposereadylist: ResMut<ActionListDisposeReadyForRef>,
    // mut disposecan: Query<&mut DisposeCan>,
) {
    // changes.iter().for_each(|entity| {
    //     if let Ok((entity, state, _)) = items.get(*entity) {
    //         if state.0 == false { return }
    //         if let Ok(mut dispose) = disposecan.get_mut(*entity) { dispose.0 = true; }
    //     }
    // });
}
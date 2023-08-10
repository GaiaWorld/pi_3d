

use pi_bevy_ecs_extend::prelude::EntityTree;
use pi_engine_shell::prelude::*;
use pi_scene_math::{coordiante_system::CoordinateSytem3, vector::{TToolMatrix}, Matrix, Rotation3, Quaternion};

use crate::{scene::coordinate_system::SceneCoordinateSytem3D, prelude::{TransformRecord, Enable, GlobalEnable}, commands::{DisposeReady, ActionListDisposeReady, ActionListDisposeCan, OpsDisposeCan}};

use super::{
    transform_node::*,
};

// pub struct SysLocalEulerModifyCalc;
// impl TSystemStageInfo for SysLocalEulerModifyCalc {
//     fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
//         vec![
//         ]
//     }
// }
// #[setup]
// impl SysLocalEulerModifyCalc {
//     #[system]
    pub fn sys_local_euler_calc_rotation(
        localmatrixs: Query<(ObjectID, &LocalEulerAngles), Changed<LocalEulerAngles>>,
        mut loacl_quaternions: Query<(&mut LocalRotationQuaternion, &mut LocalRotation)>,
    ) {
        localmatrixs.iter().for_each(|(entity, euler)| {
            if let Ok((mut loacl_quaternion, mut local_rotation)) = loacl_quaternions.get_mut(entity) {
                let rotation = Rotation3::from_euler_angles(euler.0.x, euler.0.y, euler.0.z);
                let quaternion = Quaternion::from_rotation_matrix(&rotation);
                *loacl_quaternion = LocalRotationQuaternion(quaternion.quaternion().clone(), false);
                *local_rotation = LocalRotation(rotation);
            }
        });
    }
// }

// pub struct SysLocalQuaternionModifyCalc;
// impl TSystemStageInfo for SysLocalQuaternionModifyCalc {
//     fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
//         vec![
//             SysLocalEulerModifyCalc::key(), 
//         ]
//     }
// }
// #[setup]
// impl SysLocalQuaternionModifyCalc {
//     #[system]
    pub fn sys_local_quaternion_calc_rotation(
        localmatrixs: Query<(ObjectID, &LocalRotationQuaternion), Changed<LocalRotationQuaternion>>,
        mut loacl_eulers: Query<&mut LocalRotation>,
    ) {
        localmatrixs.iter().for_each(|(entity, quat)| {
            if let Ok(mut local_rotation) = loacl_eulers.get_mut(entity) {
                if quat.1 {
                    // log::warn!("Quaternion: {:?}", quat);
                    let rotation = Quaternion::from_quaternion(quat.0).to_rotation_matrix();
                    // let (z, x, y) = rotation.euler_angles();
                    // *loacl_quaternion = LocalRotationQuaternion(quaternion);
                    *local_rotation = LocalRotation(rotation);
                }
            }
        });
    }
// }


// pub struct SysLocalMatrixCalc;
// impl TSystemStageInfo for SysLocalMatrixCalc {
//     fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
//         vec![
//             SysLocalEulerModifyCalc::key(), 
//             SysLocalQuaternionModifyCalc::key()
//         ]
//     }
// }
// #[setup]
// impl SysLocalMatrixCalc {
//     #[system]
    pub fn sys_local_matrix_calc(
        mut localmatrixs: Query<(ObjectID, &LocalPosition, &LocalScaling, &LocalRotation, &mut LocalMatrix), Or<(Changed<LocalPosition>, Changed<LocalScaling>, Changed<LocalRotation>)>>,
    ) {
        let time = pi_time::Instant::now();
        localmatrixs.iter_mut().for_each(|(entity, position, scaling, rotation, mut localmatrix)| {
            // log::warn!("LocalMatrixCalc: {:?}", entity);
            let mut matrix = Matrix::identity();
            CoordinateSytem3::matrix4_compose_rotation(&scaling.0, &rotation.0, &position.0, &mut matrix);
            // commands.entity(obj).insert(LocalMatrix(matrix, true));
            // localmatrix.0 = matrix;
            // localmatrix.1 = true;
            *localmatrix = LocalMatrix(matrix, true);
        });
        let time1 = pi_time::Instant::now();
        // log::debug!("Local Matrix Calc: {:?}", time1 - time);
    }
// }

/// 经过测试 temp_ids.push((v, true, Some(g_transform.matrix.clone()))); 拷贝父矩阵 比 临时取 父矩阵更高效 - 200ms : 300ms 
// pub struct SysWorldMatrixCalc;
// impl TSystemStageInfo for SysWorldMatrixCalc {
//     fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
//         vec![
//             SysLocalMatrixCalc::key()
//         ]
//     }
// }
// #[setup]
// impl SysWorldMatrixCalc {
//     #[system]
    pub fn sys_world_matrix_calc(
        query_scenes: Query<(ObjectID, &SceneCoordinateSytem3D)>,
        mut nodes: Query<(&mut LocalMatrix, &mut WorldMatrix, &mut WorldMatrixInv, &Enable, &mut GlobalEnable)>,
        mut transforms: Query<&mut GlobalTransform>,
        tree: EntityTree,
        mut record: ResMut<TransformRecord>,
    ) {
        let time = pi_time::Instant::now();

        // log::debug!("World Matrix Calc:");
        for (root, _) in query_scenes.iter() {
            let mut temp_ids: Vec<(ObjectID, bool, Matrix, bool)> = vec![];
            // let mut idflag: usize = 0;
            // log::warn!("World Matrix Calc: 0");
            tree.iter(root).for_each(|entity| {
                let (p_id, p_dirty, p_m, p_enable) = calc_world_root(
                    &mut nodes,
                    &mut transforms,
                    entity,
                );
                // log::warn!("World Matrix Calc: Root {:?}", entity);
                match tree.get_down(p_id) {
                    Some(node_children_head) => {
                        // log::warn!("World Matrix Calc: 2");
                        let node_children_head = node_children_head.head.0;
                        tree.iter(node_children_head).for_each(|entity| {
                            // log::warn!("World Matrix Calc: 2 {:?}", entity);
                            // idflag += 1;
                            // if idflag % 2 == 0 {
                                calc_world_one(
                                    &mut nodes,
                                    &mut transforms,
                                    &mut temp_ids,
                                    entity,
                                    p_dirty, p_enable,
                                    &p_m,
                                );
                            // }
                        });
                    },
                    None => {
                    },
                }
            });

            // log::warn!("World Matrix Calc: Level 1 ");
            calc_world(
                &mut nodes,
                &mut transforms,
                & tree,
                temp_ids
            );

        }

        let time1 = pi_time::Instant::now();

        record.all_wmcompute = (time1 - time).as_millis() as u32;
        // log::debug!("World Matrix Calc: {:?}", time1 - time);
    }
// }

// pub struct SysWorldMatrixCalc2;
// impl TSystemStageInfo for SysWorldMatrixCalc2 {
//     fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
//         vec![
//             SysLocalMatrixCalc::key()
//         ]
//     }
// }
// #[setup]
// impl SysWorldMatrixCalc2 {
//     #[system]
    pub fn sys_world_matrix_calc2(
        query_scenes: Query<(ObjectID, &SceneCoordinateSytem3D)>,
        mut nodes: Query<(&mut LocalMatrix, &mut WorldMatrix, &mut WorldMatrixInv, &Enable, &mut GlobalEnable)>,
        mut transforms: Query<&mut GlobalTransform>,
        tree: EntityTree,
        mut record: ResMut<TransformRecord>,
    ) {
        let time = pi_time::Instant::now();

        // log::debug!("World Matrix Calc:");
        for (root, _) in query_scenes.iter() {
            let mut temp_ids: Vec<(ObjectID, bool, Matrix, bool)> = vec![];
            let mut idflag: usize = 0;
            tree.iter(root).for_each(|entity| {
                let (p_id, p_dirty, p_m, p_enable) = calc_world_root(
                    &mut nodes,
                    &mut transforms,
                    entity,
                );
                
                match tree.get_down(p_id) {
                    Some(node_children_head) => {
                        let node_children_head = node_children_head.head.0;
                        tree.iter(node_children_head).for_each(|entity| {
                            idflag += 1;
                            if idflag % 2 == 1 {
                                // log::warn!("Calc WM: {:?}", entity);
                                calc_world_one(
                                    &mut nodes,
                                    &mut transforms,
                                    &mut temp_ids,
                                    entity,
                                    p_dirty, p_enable,
                                    &p_m
                                );
                            }

                        });
                    },
                    None => {
                    },
                }
            });

            calc_world(
                &mut nodes,
                &mut transforms,
                & tree,
                temp_ids
            );
        }

        let time1 = pi_time::Instant::now();
        // log::debug!("World Matrix Calc2: {:?}", time1 - time);
    }
// }

fn calc_world(
    nodes: &mut Query<(&mut LocalMatrix, &mut WorldMatrix, &mut WorldMatrixInv, &Enable, &mut GlobalEnable)>,
    transforms: &mut Query<&mut GlobalTransform>,
    tree: &EntityTree,
    mut temp_ids: Vec<(ObjectID, bool, Matrix, bool)>
) {
        // 广度优先遍历 - 最大遍历到深度 65535
        let max = 65535;
        let mut deep = 0;
        loop {
            let mut temp_list = vec![];
            if temp_ids.len() > 0 && deep < max {
                temp_ids.into_iter().for_each(|(p_id, p_dirty, p_m, p_enable)| {
                    match tree.get_down(p_id) {
                        Some(node_children_head) => {
                            let node_children_head = node_children_head.head.0;
                            tree.iter(node_children_head).for_each(|entity| {
                                // log::warn!("calc_world 2: {:?}", entity);
                                calc_world_one(
                                    nodes,
                                    transforms,
                                    &mut temp_list,
                                    entity,
                                    p_dirty, p_enable,
                                    &p_m
                                );
                            }); 
                        },
                        None => {},
                    }
                });
                deep += 1;
            } else {
                break;
            }
            temp_ids = temp_list;
        }
}

fn calc_world_one(
    nodes: &mut Query<(&mut LocalMatrix, &mut WorldMatrix, &mut WorldMatrixInv, &Enable, &mut GlobalEnable)>,
    transforms: &mut Query<&mut GlobalTransform>,
    temp_list: &mut Vec<(ObjectID, bool, Matrix, bool)>,
    entity: ObjectID,
    p_dirty: bool,
    p_enable: bool,
    p_m: &Matrix,
) {
    match (nodes.get_mut(entity), transforms.get_mut(entity)) {
        (Ok((mut lmatrix, mut wmatrix, mut wmatrixinv, enable, mut globalenable)), Ok(mut gtransform)) => {
            globalenable.0 = enable.bool() && p_enable;

            let real_dirty = p_dirty || lmatrix.1 ;
            lmatrix.1 = false;
            // log::warn!(">>>>> calc_world_one {:?}", lmatrix.1);
            if real_dirty {
                let transform = GlobalTransform::calc(&p_m, &lmatrix);
                let matrix = transform.matrix.clone();
                // let matrix = lmatrix.0.clone();
                
                // log::warn!("Calc WM: {:?}", entity);

                temp_list.push((entity, true, matrix.clone(), globalenable.0));

                *wmatrix = WorldMatrix::new(transform.matrix.clone());
                
                match transform.matrix.try_inverse() {
                    Some(inv) => *wmatrixinv = WorldMatrixInv::new(inv),
                    None => *wmatrixinv = WorldMatrixInv::new(Matrix::identity()),
                };
                *gtransform = transform;
            } else {
                temp_list.push((entity, false, gtransform.matrix.clone(), globalenable.0));
            }
        },
        (_, _) => {
            
        },
    }
}

fn calc_world_root(
    nodes: &mut Query<(&mut LocalMatrix, &mut WorldMatrix, &mut WorldMatrixInv, &Enable, &mut GlobalEnable)>,
    transforms: &mut Query<&mut GlobalTransform>,
    entity: ObjectID,
) -> (ObjectID, bool, Matrix, bool) {
    match (nodes.get_mut(entity), transforms.get_mut(entity)) {
        (Ok((mut lmatrix, mut wmatrix, mut wmatrixinv, enable, mut globalenable)), Ok(mut gtransform)) => {
            globalenable.0 = enable.bool();

            if lmatrix.1 {
                lmatrix.1 = false;
                // log::debug!(">>>>> GlobalTransform 0");
                let transform = GlobalTransform::calc(&Matrix::identity(), &lmatrix);
                let matrix = transform.matrix.clone();
                // let matrix = lmatrix.0.clone();

                *wmatrix = WorldMatrix::new(transform.matrix.clone());
                match transform.matrix.try_inverse() {
                    Some(inv) => *wmatrixinv = WorldMatrixInv::new(inv),
                    None => *wmatrixinv = WorldMatrixInv::new(Matrix::identity()),
                };
                *gtransform = transform;

                (entity, true, matrix, globalenable.0)
            } else {
                (entity, false, gtransform.matrix.clone(), globalenable.0)
            }
        },
        (_, _) => {
            // log::debug!(">>>>> WorldMatrixCalc Root");
            (entity, false, Matrix::identity(), true)
        },
    }
}

pub fn sys_dispose_about_transform_node(
    items: Query<(Entity, &DisposeReady, &TransformNode), Changed<DisposeReady>>,
    mut disposereadylist: ResMut<ActionListDisposeReady>,
    mut disposecanlist: ResMut<ActionListDisposeCan>,
) {
    items.iter().for_each(|(entity, state, _)| {
        if state.0 == false { return }

        disposecanlist.push(OpsDisposeCan::ops(entity));
    });
}
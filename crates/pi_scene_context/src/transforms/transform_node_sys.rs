
use std::time::Instant;

use pi_ecs_macros::{setup};
use pi_ecs::{prelude::{Query, Commands}, query::{Changed, Or}};
use pi_ecs_utils::prelude::EntityTree;
use pi_engine_shell::run_stage::TSystemStageInfo;
use pi_scene_math::{coordiante_system::CoordinateSytem3, vector::{TToolMatrix}, Matrix, Rotation3, Quaternion, Vector3};
use pi_slotmap_tree::Storage;

use crate::{object::{GameObject, ObjectID}, scene::coordinate_system::SceneCoordinateSytem};

use super::{
    transform_node::{GlobalTransform, LocalMatrix, LocalRotation, LocalPosition, LocalScaling, WorldMatrix, WorldMatrixInv, LocalEulerAngles, LocalRotationQuaternion},
    command::SysTransformNodeCreateCommand
};

pub struct SysLocalEulerModifyCalc;
impl TSystemStageInfo for SysLocalEulerModifyCalc {
    fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
        vec![
        ]
    }
}
#[setup]
impl SysLocalEulerModifyCalc {
    #[system]
    pub fn calc(
        mut localmatrixs: Query<GameObject, (ObjectID, &LocalEulerAngles, &mut LocalRotationQuaternion), Changed<LocalEulerAngles>>,
        mut rot_cmd: Commands<GameObject, LocalRotation>,
    ) {
        localmatrixs.iter_mut().for_each(|(obj, euler, mut quat)| {
            let rotation = Rotation3::from_euler_angles(euler.0.x, euler.0.y, euler.0.z);
            let quaternion = Quaternion::from_rotation_matrix(&rotation); 
            quat.0 = quaternion;
            rot_cmd.insert(obj, LocalRotation(rotation));
        });
    }
}

pub struct SysLocalQuaternionModifyCalc;
impl TSystemStageInfo for SysLocalQuaternionModifyCalc {
    fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
        vec![
            SysLocalEulerModifyCalc::key(), 
        ]
    }
}
#[setup]
impl SysLocalQuaternionModifyCalc {
    #[system]
    pub fn calc(
        mut localmatrixs: Query<GameObject, (ObjectID, &mut LocalEulerAngles, &LocalRotationQuaternion), Changed<LocalRotationQuaternion>>,
        mut rot_cmd: Commands<GameObject, LocalRotation>,
    ) {
        localmatrixs.iter_mut().for_each(|(obj, mut euler,  quat)| {
            let rotation = quat.0.to_rotation_matrix();
            let (z, x, y) = rotation.euler_angles();

            euler.0 = Vector3::from_column_slice(&[x, y, z]);
            rot_cmd.insert(obj, LocalRotation(rotation));
        });
    }
}


pub struct SysLocalMatrixCalc;
impl TSystemStageInfo for SysLocalMatrixCalc {
    fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
        vec![
            SysLocalEulerModifyCalc::key(), 
            SysLocalQuaternionModifyCalc::key()
        ]
    }
}
#[setup]
impl SysLocalMatrixCalc {
    #[system]
    pub fn calc(
        mut localmatrixs: Query<GameObject, (ObjectID, &LocalPosition, &LocalScaling, &LocalRotation), Or<(Changed<LocalPosition>, Changed<LocalScaling>, Changed<LocalRotation>)>>,
        mut lm_cmd: Commands<GameObject, LocalMatrix>,
    ) {
        let time = Instant::now();
        localmatrixs.iter_mut().for_each(|(obj, position, scaling, rotation)| {
            log::debug!("LocalMatrixCalc:");
            let mut matrix = Matrix::identity();
            CoordinateSytem3::matrix4_compose_rotation(&scaling.0, &rotation.0, &position.0, &mut matrix);
            lm_cmd.insert(obj, LocalMatrix(matrix, true));
        });
        let time1 = Instant::now();
        log::info!("Local Matrix Calc: {:?}", time1 - time);
    }
}

/// 经过测试 temp_ids.push((v, true, Some(g_transform.matrix.clone()))); 拷贝父矩阵 比 临时取 父矩阵更高效 - 200ms : 300ms 
pub struct SysWorldMatrixCalc;
impl TSystemStageInfo for SysWorldMatrixCalc {
    fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
        vec![
            SysLocalMatrixCalc::key()
        ]
    }
}
#[setup]
impl SysWorldMatrixCalc {
    #[system]
    pub fn calc(
        query_scenes: Query<GameObject, (ObjectID, &SceneCoordinateSytem)>,
        mut globaltransforms: Query<GameObject, (&mut LocalMatrix, &GlobalTransform)>,
        mut gtr_cmd: Commands<GameObject, GlobalTransform>,
        mut wm_cmd: Commands<GameObject, WorldMatrix>,
        mut wminv_cmd: Commands<GameObject, WorldMatrixInv>,
        tree: EntityTree<GameObject>,
    ) {
        let time = Instant::now();

        // log::debug!("World Matrix Calc:");
        for (root, _) in query_scenes.iter() {
            let mut temp_ids: Vec<(ObjectID, bool, Matrix)> = vec![];
            let mut idflag: usize = 0;
            tree.iter(root).for_each(|entity| {
                let (p_id, p_dirty, p_m) = calc_world_root(
                    &mut globaltransforms,
                    &mut gtr_cmd,
                    &mut wm_cmd,
                    &mut wminv_cmd,
                    entity,
                );
                
                match tree.get_down(p_id) {
                    Some(node_children_head) => {
                        let node_children_head = node_children_head.head;
                        tree.iter(node_children_head).for_each(|entity| {
                            idflag += 1;
                            if idflag % 2 == 0 {
                                calc_world_one(
                                    &mut globaltransforms,
                                    &mut gtr_cmd,
                                    &mut wm_cmd,
                                    &mut wminv_cmd,
                                    &mut temp_ids,
                                    entity,
                                    p_dirty,
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
                &mut globaltransforms,
                &mut gtr_cmd,
                &mut wm_cmd,
                &mut wminv_cmd,
                & tree,
                temp_ids
            );
        }

        let time1 = Instant::now();
        log::info!("World Matrix Calc: {:?}", time1 - time);
    }
}

pub struct SysWorldMatrixCalc2;
impl TSystemStageInfo for SysWorldMatrixCalc2 {
    fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
        vec![
            SysLocalMatrixCalc::key()
        ]
    }
}
#[setup]
impl SysWorldMatrixCalc2 {
    #[system]
    pub fn calc(
        query_scenes: Query<GameObject, (ObjectID, &SceneCoordinateSytem)>,
        mut globaltransforms: Query<GameObject, (&mut LocalMatrix, &GlobalTransform)>,
        mut gtr_cmd: Commands<GameObject, GlobalTransform>,
        mut wm_cmd: Commands<GameObject, WorldMatrix>,
        mut wminv_cmd: Commands<GameObject, WorldMatrixInv>,
        tree: EntityTree<GameObject>,
    ) {
        let time = Instant::now();

        // log::debug!("World Matrix Calc:");
        for (root, _) in query_scenes.iter() {
            let mut temp_ids: Vec<(ObjectID, bool, Matrix)> = vec![];
            let mut idflag: usize = 0;
            tree.iter(root).for_each(|entity| {
                let (p_id, p_dirty, p_m) = calc_world_root(
                    &mut globaltransforms,
                    &mut gtr_cmd,
                    &mut wm_cmd,
                    &mut wminv_cmd,
                    entity,
                );
                
                match tree.get_down(p_id) {
                    Some(node_children_head) => {
                        let node_children_head = node_children_head.head;
                        tree.iter(node_children_head).for_each(|entity| {
                            idflag += 1;
                            if idflag % 2 == 1 {
                                calc_world_one(
                                    &mut globaltransforms,
                                    &mut gtr_cmd,
                                    &mut wm_cmd,
                                    &mut wminv_cmd,
                                    &mut temp_ids,
                                    entity,
                                    p_dirty,
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
                &mut globaltransforms,
                &mut gtr_cmd,
                &mut wm_cmd,
                &mut wminv_cmd,
                & tree,
                temp_ids
            );
        }

        let time1 = Instant::now();
        log::info!("World Matrix Calc2: {:?}", time1 - time);
    }
}

fn calc_world(
    globaltransforms: &mut Query<GameObject, (&mut LocalMatrix, &GlobalTransform)>,
    gtr_cmd: &mut Commands<GameObject, GlobalTransform>,
    wm_cmd: &mut Commands<GameObject, WorldMatrix>,
    wminv_cmd: &mut Commands<GameObject, WorldMatrixInv>,
    tree: &EntityTree<GameObject>,
    mut temp_ids: Vec<(ObjectID, bool, Matrix)>
) {

        // 广度优先遍历 - 最大遍历到深度 65535
        let max = 65535;
        let mut deep = 0;
        loop {
            let mut temp_list = vec![];
            if temp_ids.len() > 0 && deep < max {
                temp_ids.into_iter().for_each(|(p_id, p_dirty, p_m)| {
                    match tree.get_down(p_id) {
                        Some(node_children_head) => {
                            let node_children_head = node_children_head.head;
                            tree.iter(node_children_head).for_each(|entity| {
                                calc_world_one(
                                    globaltransforms,
                                    gtr_cmd,
                                    wm_cmd,
                                    wminv_cmd,
                                    &mut temp_list,
                                    entity,
                                    p_dirty,
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
    globaltransforms: &mut Query<GameObject, (&mut LocalMatrix, &GlobalTransform)>,
    gtr_cmd: &mut Commands<GameObject, GlobalTransform>,
    wm_cmd: &mut Commands<GameObject, WorldMatrix>,
    wminv_cmd: &mut Commands<GameObject, WorldMatrixInv>,
    temp_list: &mut Vec<(ObjectID, bool, Matrix)>,
    entity: ObjectID,
    p_dirty: bool,
    p_m: &Matrix,
) {
    match globaltransforms.get_mut(entity) {
        Some((mut lmatrix, transform)) => {
            let real_dirty = p_dirty || lmatrix.1 ;
            lmatrix.1 = false;
            if real_dirty {
                // log::debug!(">>>>> GlobalTransform 2");
                let transform = GlobalTransform::calc(&p_m, &lmatrix);
                let matrix = transform.matrix.clone();
                // let matrix = lmatrix.0.clone();

                temp_list.push((entity, true, matrix.clone()));

                wm_cmd.insert(entity, WorldMatrix::new(transform.matrix.clone()));
                wminv_cmd.insert(entity, WorldMatrixInv::new(transform.matrix_inv.clone()));
                gtr_cmd.insert(entity, transform);
            } else {
                temp_list.push((entity, false, transform.matrix.clone()));
            }
        },
        None => {
            
        },
    }
}

fn calc_world_root(
    globaltransforms: &mut Query<GameObject, (&mut LocalMatrix, &GlobalTransform)>,
    gtr_cmd: &mut Commands<GameObject, GlobalTransform>,
    wm_cmd: &mut Commands<GameObject, WorldMatrix>,
    wminv_cmd: &mut Commands<GameObject, WorldMatrixInv>,
    entity: ObjectID,
) -> (ObjectID, bool, Matrix) {
    match globaltransforms.get_mut(entity) {
        Some((mut lmatrix, transform)) => {
            if lmatrix.1 {
                lmatrix.1 = false;
                // log::debug!(">>>>> GlobalTransform 0");
                let transform = GlobalTransform::calc(&Matrix::identity(), &lmatrix);
                let matrix = transform.matrix.clone();
                // let matrix = lmatrix.0.clone();

                wm_cmd.insert(entity, WorldMatrix::new(transform.matrix.clone()));
                wminv_cmd.insert(entity, WorldMatrixInv::new(transform.matrix_inv.clone()));
                gtr_cmd.insert(entity, transform);

                (entity, true, matrix)
            } else {
                (entity, false, transform.matrix.clone())
            }
        },
        None => {
            // log::debug!(">>>>> WorldMatrixCalc Root");
            (entity, false, Matrix::identity())
        },
    }
}
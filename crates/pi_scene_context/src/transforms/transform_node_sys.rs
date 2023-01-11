
use std::time::Instant;

use pi_ecs_macros::{setup};
use pi_ecs::{prelude::{Query, Commands}, query::{Write, Changed, Or}};
use pi_ecs_utils::prelude::EntityTree;
use pi_engine_shell::run_stage::TSystemStageInfo;
use pi_scene_math::{coordiante_system::CoordinateSytem3, vector::{TToolRotation, TToolMatrix}, Matrix, Rotation3};
use pi_slotmap_tree::Storage;

use crate::{object::{GameObject, ObjectID}, scene::coordinate_system::SceneCoordinateSytem};

use super::{transform_node::{GlobalTransform, LocalMatrix, LocalRotation, LocalEulerAngles, LocalRotationQuaternion, LocalRoationWithQuaternion, LocalPosition, LocalScaling, WorldMatrix, WorldMatrixInv}, command::SysTransformNodeCommand};


pub struct SysLocalMatrixCalc;
impl TSystemStageInfo for SysLocalMatrixCalc {
    fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
        vec![
            SysTransformNodeCommand::key(),
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
        localmatrixs.iter_mut().for_each(|(obj, position, scaling, rotation)| {
             log::debug!("LocalMatrixCalc:");
            let mut matrix = Matrix::identity();
            CoordinateSytem3::matrix4_compose_rotation(&scaling.0, &rotation.0, &position.0, &mut matrix);
            lm_cmd.insert(obj, LocalMatrix(matrix, true));
        });
    }
}

/// 经过测试 temp_ids.push((v, true, Some(g_transform.matrix.clone()))); 拷贝父矩阵 比 临时取 父矩阵更高效 - 200ms : 300ms 
pub struct SysWorldMatrixCalc;
impl TSystemStageInfo for SysWorldMatrixCalc {
    fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
        vec![
            SysTransformNodeCommand::key(), SysLocalMatrixCalc::key()
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
            tree.iter(root).for_each(|entity| {
                match globaltransforms.get_mut(entity) {
                    Some((mut lmatrix, transform)) => {
                        if lmatrix.1 {
                            lmatrix.1 = false;
                            log::debug!(">>>>> GlobalTransform 0");
                            let transform = GlobalTransform::calc(&Matrix::identity(), &lmatrix);
                            temp_ids.push((entity, true, transform.matrix.clone()));

                            wm_cmd.insert(entity, WorldMatrix::new(transform.matrix.clone()));
                            wminv_cmd.insert(entity, WorldMatrixInv::new(transform.matrix_inv.clone()));
                            gtr_cmd.insert(entity, transform);
                        } else {
                            temp_ids.push((entity, false, transform.matrix.clone()));
                        }
                    },
                    None => {
                        log::debug!(">>>>> WorldMatrixCalc Root");
                        temp_ids.push((entity, false, Matrix::identity()));
                    },
                }
            });
    
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
                                    match globaltransforms.get_mut(entity) {
                                        Some((mut lmatrix, transform)) => {
                                            let real_dirty = p_dirty || lmatrix.1 ;
                                            lmatrix.1 = false;
                                            if real_dirty {
                                                log::debug!(">>>>> GlobalTransform 2");
                                                let transform = GlobalTransform::calc(&p_m, &lmatrix);
                                                temp_list.push((entity, true, transform.matrix.clone()));

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

        let time1 = Instant::now();
        log::info!("World Matrix Calc: {:?}", time1 - time);
    }
}

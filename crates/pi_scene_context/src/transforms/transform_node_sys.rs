
use std::time::Instant;

use pi_ecs_macros::{setup};
use pi_ecs::{prelude::{Query}, query::{Write, Changed, Or}};
use pi_ecs_utils::prelude::EntityTree;
use pi_scene_math::{coordiante_system::CoordinateSytem3, vector::{TToolRotation, TToolMatrix}, Matrix, Rotation3};
use pi_slotmap_tree::Storage;

use crate::{object::{GameObject, ObjectID}, scene::coordinate_system::SceneCoordinateSytem, meshes::model::BuildinModelBind};

use super::{transform_node::{GlobalTransform, LocalMatrix, LocalRotation, LocalEulerAngles, LocalRotationQuaternion, LocalRoationWithQuaternion, LocalPosition, LocalScaling, WorldMatrix, WorldMatrixInv}};

pub struct LocalRotationMatrixCalc;
#[setup]
impl LocalRotationMatrixCalc {
    #[system]
    pub fn calc(
        mut local_rotations: Query<
            GameObject,
            (Write<LocalRotation>, &LocalEulerAngles, &LocalRotationQuaternion, &LocalRoationWithQuaternion),
            Or<(Changed<LocalEulerAngles>, Changed<LocalRotationQuaternion>, Changed<LocalRoationWithQuaternion>)>
        >,
    ) {
        //  println!("LocalRotationMatrixCalc:");
        // let coordsys = CoordinateSytem3::left();
        // local_rotations.iter_mut().for_each(|(mut rotation, euler, quaternion, withquaternion)| {
        //     match withquaternion.0 {
        //         true => {
        //             rotation.write(LocalRotation(quaternion.0.to_rotation_matrix()));
        //         },
        //         false => {
        //             let mut temp = Rotation3::identity();
        //             CoordinateSytem3::rotation_matrix_mut_euler_angles(euler.0.x, euler.0.y, euler.0.z, &mut temp);
        //             rotation.write(LocalRotation(temp));
        //         },
        //     }
        // });
    }
}

pub struct LocalMatrixCalc;
#[setup]
impl LocalMatrixCalc {
    #[system]
    pub fn calc(
        mut localmatrixs: Query<GameObject, (Write<LocalMatrix>, &LocalPosition, &LocalScaling, &LocalRotation), Or<(Changed<LocalPosition>, Changed<LocalScaling>, Changed<LocalRotation>)>>,
    ) {
        localmatrixs.iter_mut().for_each(|(mut localmatrix, position, scaling, rotation)| {
            //  println!("LocalMatrixCalc:");
            let mut matrix = Matrix::identity();
            CoordinateSytem3::matrix4_compose_rotation(&scaling.0, &rotation.0, &position.0, &mut matrix);
            localmatrix.write(LocalMatrix(matrix, true));
        });
    }
}

/// 经过测试 temp_ids.push((v, true, Some(g_transform.matrix.clone()))); 拷贝父矩阵 比 临时取 父矩阵更高效 - 200ms : 300ms 
pub struct WorldMatrixCalc;
#[setup]
impl WorldMatrixCalc {
    #[system]
    pub fn calc(
        query_scenes: Query<GameObject, (ObjectID, &SceneCoordinateSytem)>,
        mut globaltransforms: Query<GameObject, (&mut LocalMatrix, Write<GlobalTransform>, Write<WorldMatrix>, Write<WorldMatrixInv>)>,
        tree: EntityTree<GameObject>,
    ) {
        let time = Instant::now();

        // println!("World Matrix Calc:");
        for (root, _) in query_scenes.iter() {
            let mut temp_ids: Vec<(ObjectID, bool, Matrix)> = vec![];
            tree.iter(root).for_each(|entity| {
                match globaltransforms.get_mut(entity) {
                    Some((mut lmatrix, mut g_transform, mut worldmatrxi, mut worldmatrix_inv)) => {

                        if lmatrix.1 {
                            lmatrix.1 = false;
                            // println!(">>>>> local_dirty 0");
                            let transform = GlobalTransform::calc(&Matrix::identity(), &lmatrix);
                            worldmatrxi.write(WorldMatrix::new(transform.matrix.clone()));
                            worldmatrix_inv.write(WorldMatrixInv::new(transform.matrix_inv.clone()));
                            temp_ids.push((entity, true, transform.matrix.clone()));

                            g_transform.write(transform);
                        } else {
                            if let Some(transform) = g_transform.get() {
                                temp_ids.push((entity, false, transform.matrix.clone()));
                            } else {
                                temp_ids.push((entity, false, Matrix::identity()));
                            }
                        }
                    },
                    None => {
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
                                        Some((mut lmatrix, mut g_transform, mut worldmatrxi, mut worldmatrix_inv)) => {
                                            let real_dirty = p_dirty || lmatrix.1 ;
                                            lmatrix.1 = false;
                                            if real_dirty {
                                                // println!(">>>>> local_dirty 2");
                                                let transform = GlobalTransform::calc(&p_m, &lmatrix);
                                                worldmatrxi.write(WorldMatrix::new(transform.matrix.clone()));
                                                worldmatrix_inv.write(WorldMatrixInv::new(transform.matrix_inv.clone()));
                                                temp_list.push((entity, true, transform.matrix.clone()));

                                                g_transform.write(transform);
                                            } else {
                                                if let Some(transform) = g_transform.get() {
                                                    temp_list.push((entity, false, transform.matrix.clone()));
                                                } else {
                                                    temp_list.push((entity, false, Matrix::identity()));
                                                }
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
        println!("World Matrix Calc: {:?}", time1 - time);
    }
}

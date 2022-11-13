
use std::time::Instant;

use pi_ecs_macros::{setup};
use pi_ecs::{prelude::{Query}, query::{Write, With}};
use pi_ecs_utils::prelude::EntityTree;
use pi_scene_math::{coordiante_system::CoordinateSytem3, vector::{TToolRotation, TToolMatrix}, Matrix};
use pi_slotmap_tree::Storage;

use crate::{object::{GameObject, ObjectID}, scene::coordinate_system::SceneCoordinateSytem, transforms::dirty::DirtyGlobalTransform};

use super::{transform_node::{LocalTransform, GlobalTransform}, dirty::DirtyLocalTransform};

pub struct LocalRotationMatrixCalc;
#[setup]
impl LocalRotationMatrixCalc {
    #[system]
    pub fn calc(
        mut query_locals: Query<GameObject, (&mut LocalTransform), With<DirtyLocalTransform>>,
    ) {
        //  println!("LocalRotationMatrixCalc:");
        let coordsys = CoordinateSytem3::left();
        query_locals.iter_mut().for_each(|(mut l_transform)| {
            match l_transform.use_quaternion {
                true => {
                    let m = l_transform.quaternion.to_rotation_matrix();
                    l_transform.rotation.clone_from(&m);
                },
                false => {
                    CoordinateSytem3::rotation_matrix_mut_euler_angles(l_transform.euler.x, l_transform.euler.y, l_transform.euler.z, &mut l_transform.rotation);
                },
            }
        });
    }
}

pub struct LocalMatrixCalc;
#[setup]
impl LocalMatrixCalc {
    #[system]
    pub fn calc(
        mut query_locals: Query<GameObject, (&mut LocalTransform), With<DirtyLocalTransform>>,
    ) {
        //  println!("LocalMatrixCalc:");
        query_locals.iter_mut().for_each(|(mut l_transform)| {
            let scaling = l_transform.scaling.clone();
            let position = l_transform.position.clone();
            let rotation = l_transform.rotation.clone();
            CoordinateSytem3::matrix4_compose_rotation(&scaling, &rotation, &position, &mut l_transform.matrix);
            //  println!("{}", l_transform.matrix);
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
        mut query_ms: Query<GameObject, (&LocalTransform, &mut GlobalTransform, Write<DirtyGlobalTransform>)>,
        query_local_dirty: Query<GameObject, &DirtyLocalTransform>,
        tree: EntityTree<GameObject>,
    ) {
        let time = Instant::now();

        //  println!("World Matrix Calc:");
        for (root, _) in query_scenes.iter() {
            let mut temp_ids: Vec<(ObjectID, bool, Option<Matrix>)> = vec![];
            tree.iter(root).for_each(|entity| {
                match query_ms.get_mut(entity) {
                    Some((l_transform, mut g_transform, mut dirty_global)) => {
                        if query_local_dirty.get(entity).is_some() {
                            g_transform.calc(None, l_transform);
                            temp_ids.push((entity, true, Some(g_transform.matrix.clone())));
                            dirty_global.insert_no_notify(DirtyGlobalTransform);
                        } else {
                            temp_ids.push((entity, false, Some(g_transform.matrix.clone())));
                        }
                    },
                    None => {
                        temp_ids.push((entity, false, None));
                    },
                }
            });
    
            // 广度优先遍历 - 最大遍历到深度 65535
            let max = 65535;
            let mut deep = 0;
            loop {
                let mut temp = vec![];
                if temp_ids.len() > 0 && deep < max {
                    temp_ids.into_iter().for_each(|(p_id, p_dirty, p_m)| {
                        match tree.get_down(p_id) {
                            Some(node_children_head) => {
                                let node_children_head = node_children_head.head;
                                tree.iter(node_children_head).for_each(|entity| {
                                    match query_ms.get_mut(entity) {
                                        Some((l_transform, mut g_transform, mut dirty_global)) => {
                                            let real_dirty = p_dirty || query_local_dirty.get(entity).is_some();
                                            if real_dirty {
                                                // println!("Transform real_dirty >>>>>>>>>> ");
                                                g_transform.calc(p_m, l_transform);
                                                temp.push((entity, true, Some(g_transform.matrix.clone())));
                                                dirty_global.insert_no_notify(DirtyGlobalTransform);
                                            } else {
                                                temp.push((entity, false, Some(g_transform.matrix.clone())));
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
                temp_ids = temp;
            }
        }

        let time1 = Instant::now();
        // println!("World Matrix Calc: {:?}", time1 - time);
    }
}
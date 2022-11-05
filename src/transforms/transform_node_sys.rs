
use pi_ecs_macros::{setup};
use pi_ecs::{prelude::{Query}};
use pi_ecs_utils::prelude::EntityTree;
use pi_scene_math::{coordiante_system::CoordinateSytem3, vector::{TToolRotation, TToolMatrix}};
use pi_slotmap_tree::Storage;

use crate::{object::{GameObject, ObjectID}, scene::coordinate_system::SceneCoordinateSytem};

use super::transform_node::{LocalTransform, TransformDirty, GlobalTransform};

// pub struct TransformNodeInit;
// #[setup]
// impl TransformNodeInit {
//     /// 监听到 TransformNode 组件被添加, 则 添加 LocalPosition, 
//     #[listen()]
//     pub fn calc(
//     ) {

//     }
// }
pub struct LocalRotationMatrixCalc;
#[setup]
impl LocalRotationMatrixCalc {
    #[system]
    pub fn calc(
        mut query_locals: Query<GameObject, (&mut TransformDirty, &mut LocalTransform)>,
    ) {
        println!("LocalRotationMatrixCalc:");
        let coordsys = CoordinateSytem3::left();
        query_locals.iter_mut().for_each(|(dirty, mut l_transform)| {
            match dirty.0 {
                true => {
                    match l_transform.use_quaternion {
                        true => {
                            let m = l_transform.quaternion.to_rotation_matrix();
                            l_transform.rotation.clone_from(&m);
                        },
                        false => {
                            coordsys.rotation_matrix_mut_euler_angles(l_transform.euler.x, l_transform.euler.y, l_transform.euler.z, &mut l_transform.rotation);
                        },
                    }
                },
                false => {},
            }
        });
    }
}

pub struct LocalMatrixCalc;
#[setup]
impl LocalMatrixCalc {
    #[system]
    pub fn calc(
        mut query_locals: Query<GameObject, (&mut TransformDirty, &mut LocalTransform)>,
    ) {
        println!("LocalMatrixCalc:");
        query_locals.iter_mut().for_each(|(dirty, mut l_transform)| {
            match dirty.0 {
                true => {
                    let scaling = l_transform.scaling.clone();
                    let position = l_transform.position.clone();
                    let rotation = l_transform.rotation.clone();
                    CoordinateSytem3::matrix4_compose_rotation(&scaling, &rotation, &position, &mut l_transform.matrix);
                    println!("{}", l_transform.matrix);
                },
                false => {
                    //
                },
            }
        });
    }
}

pub struct WorldMatrixCalc;
#[setup]
impl WorldMatrixCalc {
    #[system]
    pub fn calc(
        query_scenes: Query<GameObject, (ObjectID, &SceneCoordinateSytem)>,
        mut query_ms: Query<GameObject, (&mut TransformDirty, &LocalTransform, &mut GlobalTransform)>,
        tree: EntityTree<GameObject>,
    ) {
        println!("World Matrix Calc:");
        for (root, _) in query_scenes.iter() {
            // println!("Get Scene {:?}", root);
            let mut temp_ids = vec![];
            tree.iter(root).for_each(|v| {
                match query_ms.get_mut(v) {
                    Some((mut dirty, l_transform, mut g_transform)) => {
                        if dirty.0 {
                            g_transform.calc(None, l_transform);
                            temp_ids.push((v, true, Some(g_transform.matrix.clone())));
                        } else {
                            temp_ids.push((v, false, Some(g_transform.matrix.clone())));
                        }
                        dirty.0 = false;
                    },
                    None => {
                        temp_ids.push((v, false, None));
                    },
                }
            });
    
            let max = 4096;
            let mut i = 0;
            loop {
                // println!("temp_ids {}", temp_ids.len());
                let mut temp = vec![];
                if temp_ids.len() > 0 && i < max {
                    temp_ids.into_iter().for_each(|(p_id, p_dirty, p_m)| {
                        // println!("Parent {:?}", p_id);

                        match tree.get_down(p_id) {
                            Some(node_children_head) => {
                                let node_children_head = node_children_head.head;
                                tree.iter(node_children_head).for_each(|entity| {
                                    println!("Child {:?}", entity);

                                    match query_ms.get_mut(entity) {
                                        Some((mut dirty, l_transform, mut g_transform)) => {
                                            let real_dirty = dirty.0 || p_dirty;
                                            if real_dirty {
                                                g_transform.calc(p_m, l_transform);
                                                temp.push((entity, true, Some(g_transform.matrix.clone())));
                                            } else {
                                                temp.push((entity, false, Some(g_transform.matrix.clone())));
                                            }
                                            dirty.0 = false;

                                            println!("{}", g_transform.matrix);
                                        },
                                        None => {
                                            println!("Child Not Found {:?}", entity);
                                        },
                                    }
                                }); 
                            },
                            None => {},
                        }
                    });
                    i += 1;
                } else {
                    break;
                }
                temp_ids = temp;
            }
        }
    }
    // #[system]
    // pub fn calc(
    //     query_scenes: Query<GameObject, (ObjectID, &SceneParam)>,
    //     mut query_ms: Query<GameObject, (&mut TransformDirty, &LocalMatrix, &mut WorldMatrix, &mut GlobalPosition, &mut GlobalRotation, &mut GlobalScaling, &mut GlobalIsometry)>,
    //     tree: EntityTree<GameObject>,
    // ) {
    //     println!("World Matrix Calc:");
    //     for (root, _) in query_scenes.iter() {
    //         let mut temp_ids = vec![];
    //         tree.iter(root).for_each(|v| {
    //             match query_ms.get_mut(v) {
    //                 Some((mut dirty, l_m, mut w_m, mut g_p, mut g_r, mut g_s, mut g_i)) => {
    //                     if dirty.0 {
    //                         calc_world_matrix(None, l_m, &mut w_m, &mut g_p, &mut g_r, &mut g_s, &mut g_i);
    //                         temp_ids.push((v, true, Some(w_m.0.clone())));
    //                     } else {
    //                         temp_ids.push((v, false, Some(w_m.0.clone())));
    //                     }
    //                     dirty.0 = false;
    //                 },
    //                 None => {
    //                     temp_ids.push((v, false, None));
    //                 },
    //             }
    //         });
    
    //         let max = 4096;
    //         let mut i = 0;
    //         loop {
    //             let mut temp = vec![];
    //             if temp_ids.len() > 0 && i < max {
    //                 temp_ids.iter().for_each(|(p_id, p_dirty, p_m)| {
    //                     let p_dirty = *p_dirty;
    //                     tree.iter(*p_id).for_each(|entity| {
    //                         match query_ms.get_mut(entity) {
    //                             Some((mut dirty, l_m, mut w_m, mut g_p, mut g_r, mut g_s, mut g_i)) => {
    //                                 if dirty.0 || p_dirty {
    //                                     match p_m {
    //                                         Some(p_m) => calc_world_matrix(Some(p_m), l_m, &mut w_m, &mut g_p, &mut g_r, &mut g_s, &mut g_i),
    //                                         None => calc_world_matrix(None, l_m, &mut w_m, &mut g_p, &mut g_r, &mut g_s, &mut g_i),
    //                                     };
    //                                     temp.push((entity, true, Some(w_m.0.clone())));
    //                                 } else {
    //                                     temp.push((entity, false, Some(w_m.0.clone())));
    //                                 }
    //                                 dirty.0 = false;
    //                             },
    //                             None => {},
    //                         }
    //                     }); 
    //                 });
    //                 i += 1;
    //             } else {
    //                 break;
    //             }
    //             temp_ids = temp;
    //         }
    //     }
    // }
}

use pi_scene_shell::prelude::*;
use pi_scene_math::Matrix;

use crate::transforms::transform_node::*;

use super::{skeleton::*, bone::*};

    pub fn sys_skin_dirty_by_bone(
        mut skins: Query<&mut SkeletonBonesDirty>,
        bones: Query<&SkeletonID, (Or<(Changed<SkeletonID>, Changed<BoneAbsoluteInv>)>, With<BoneAbsoluteInv>)>,
    ) {
        bones.iter().for_each(|skin| {
            // log::error!("skin_dirty_by_bone {:?}", &skin.0);
            if let Some(idskin) = skin.0 {
                if let Ok(mut item) = skins.get_mut(idskin) {
                    // log::error!("skin_dirty_by_bone {:?}", (2));
                    *item = SkeletonBonesDirty(true);
                }
            }
        });
    }
    
    pub fn sys_bones_local_dirty(
        mut skins: Query<(&Skeleton, &mut SkeletonBonesDirty, &mut SkeletonBoneWorldMatrixDirty)>,
        bonelinked: Query<&BoneLinked>,
        nodes: Query<Entity, Changed<GlobalMatrix>>,
        bones: Query<&SkeletonID, Changed<SkeletonID>>,
    ) {
        skins.iter_mut().for_each(|(skeleton, mut dirty, mut item)| {
            skeleton.bones.iter().for_each(|bone| {
                if let Ok(BoneLinked(Some(linked))) = bonelinked.get(*bone) {
                    if nodes.contains(*linked) {
                        *item = SkeletonBoneWorldMatrixDirty;
                        *dirty = SkeletonBonesDirty(true);
                    }
                }
            });
        });
        bones.iter().for_each(|skin| {
            if let Some(idskin) = skin.0 {
                if let Ok((_, mut dirty, mut item)) = skins.get_mut(idskin) {
                    *item = SkeletonBoneWorldMatrixDirty;
                    *dirty = SkeletonBonesDirty(true);
                }
            }
        });
    }

    pub fn sys_bones_worldmatrix(
        items: Query<
            &Skeleton,
            Changed<SkeletonBoneWorldMatrixDirty>
        >,
        nodes: Query<&LocalMatrix>,
        mut bones: Query<(&BoneLinked, &mut BoneWorldMatrix)>,
        parents: EntityTree,
    ) {
        let mut roots = XHashSet::default();
        items.iter().for_each(|skeleton| {
            // log::error!("sys_bones_local_dirty ");
            let parent = skeleton.root;
            roots.insert(parent);
            // let temp_ids: Vec<(ObjectID, Matrix)> = vec![(parent, Matrix::identity())];
            // calc_bone_world_matrix(&nodes, &mut bones, temp_ids, &parents);
            // if bones.contains(parent) == false {
            //     let temp_ids: Vec<(ObjectID, Matrix)> = vec![(parent, Matrix::identity())];
            //     calc_bone_world_matrix(&nodes, &mut bones, temp_ids, &parents);
            // }

            // skeleton.bones.iter().for_each(|bone| {
            //     if let Some(parent) = parents.get_up(*bone) {
            //         let parent = parent.parent();
            //         if bones.contains(parent) == false {
            //             let temp_ids: Vec<(ObjectID, Matrix)> = vec![(parent, Matrix::identity())];
            //             calc_bone_world_matrix(&mut bones, temp_ids, &parents);
            //         }
            //     }
            // });
        });
        roots.iter().for_each(|root| {
            if let Ok((link, mut world)) = bones.get_mut(*root) {
                if let Some(link) = &link.0 {
                    world.0 = if let Ok(local) = nodes.get(*link) {
                        local.0.clone()
                    } else { Matrix::identity() }
                }

                let temp_ids: Vec<(ObjectID, Matrix)> = vec![(*root, world.0.clone())];
                calc_bone_world_matrix(&nodes, &mut bones, temp_ids, &parents);
            }
        });
    }

    pub fn sys_bones_absolute(
        items: Query<
            &Skeleton,
            Changed<SkeletonInitBaseMatrix>
        >,
        mut bones: Query<(&BoneBaseMatrix, &mut BoneAbsolute, &mut BoneAbsoluteInv)>,
        // parents: Query<&NodeChilds>,
        parents: EntityTree,
    ) {
        items.iter().for_each(|skeleton| {
            let parent = skeleton.root;
            if let Ok((base, mut abs, mut absinv)) = bones.get_mut(parent) {
                abs.0 = base.0.clone();
                absinv.update(&abs);
                let temp_ids: Vec<(ObjectID, Matrix)> = vec![(parent, abs.0.clone())];
                calc_bone_absolute(&mut bones, temp_ids, &parents);
            }
            // skeleton.bones.iter().for_each(|bone| {
            //     if let Some(parent) = parents.get_up(*bone) {
            //         let parent = parent.parent();
            //         if bones.contains(parent) == false {
            //             let temp_ids: Vec<(ObjectID, Matrix)> = vec![(parent, Matrix::identity())];
            //             calc_bone_absolute(&mut bones, temp_ids, &parents);
            //         }
            //     }
            // });
        });
    }

    pub fn sys_skin_buffer_update(
        mut items: Query<
            (
                &Skeleton,
                &SkeletonBonesDirty
            ),
            Changed<SkeletonBonesDirty>
        >,
        bones: Query<(&BoneWorldMatrix, &BoneAbsoluteInv)>,
    ) {
        items.iter_mut().for_each(|(skel, skindirty)| {
            if skindirty.0 {
                match skel.mode {
                    ESkinCode::None => {},
                    ESkinCode::UBO(_, _, cache) => {
                        // log::error!("skin_buffer_update {:?}", (cache, skel.bones.len()));
                        if cache == 1 {
                            let mut data = vec![];
                            skel.bones.iter().for_each(|bone| {
                                if let Ok((matrix, absinv)) = bones.get(bone.clone()) {
                                    let matrix = matrix.0 * absinv.0;
                                    matrix.as_slice().iter().for_each(|v| {
                                        data.push(*v);
                                    });
                                }
                            });
                            // log::warn!("skin_buffer_update");
                            skel.bind.as_ref().unwrap().data().write_data(0, bytemuck::cast_slice(&data));
                        }
                    },
                    ESkinCode::RowTexture(_) => {
                        // if let Some(tex) = tex {
                        //     let mut data = vec![];
                        //     skel.bones.iter().for_each(|bone| {
                        //         if let Some(matrix) = bones.get(bone.clone()) {
                        //             matrix.0.as_slice().iter().for_each(|v| {
                        //                 data.push(*v);
                        //             });
                        //         }
                        //     });
        
                        //     let mut buff_data = tex.tex.create_data();
        
                        //     log::debug!("Skeleton Tex: {:?}, {:?}", tex.tex.size(), buff_data.len());
                
                        //     tex.tex.update_row(0, bytemuck::cast_slice(data.as_slice()), &mut buff_data);
        
                        //     tex.tex.update_texture(&queue, buff_data.as_slice());
                        // }
                    },
                    ESkinCode::FramesTexture(_) => {},
                }
                
            }
        });
    }

    fn calc_bone_world_matrix(
        nodes: &Query<&LocalMatrix>,
        bones: &mut Query<(&BoneLinked, &mut BoneWorldMatrix)>,
        mut temp_ids: Vec<(ObjectID, Matrix)>,
        // parents: &Query<&NodeChilds>,
        tree: &EntityTree,
    ) {
            // 广度优先遍历 - 最大遍历到深度 65535
            let max = 128;
            let mut deep = 0;
            loop {
                let mut temp_list = vec![];
                if temp_ids.len() > 0 && deep < max {
                    temp_ids.into_iter().for_each(|(p_id, p_world)| {
                        match tree.get_down(p_id) {
                            Some(node_children_head) => {
                                // let node_children_head = node_children_head.head.0;
                                let node_children_head = node_children_head.head;
                                tree.iter(node_children_head).for_each(|entity| {
                                    calc_bone_world_matrix_one(
                                        nodes,
                                        bones,
                                        &mut temp_list,
                                        entity,
                                        &p_world,
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
    fn calc_bone_world_matrix_one(
        nodes: &Query<&LocalMatrix>,
        bones: &mut Query<(&BoneLinked, &mut BoneWorldMatrix)>,
        temp_list: &mut Vec<(ObjectID, Matrix)>,
        entity: ObjectID,
        p_world: &Matrix,
    ) {
        if let Ok((link, mut world)) = bones.get_mut(entity) {
            if let Some(link) = &link.0 {
                world.0 = if let Ok(local) = nodes.get(*link) {
                    p_world * local.0
                } else {
                    p_world.clone()
                }
            } else {
                world.0 = p_world.clone();
            }
            temp_list.push((entity, world.0.clone()));
        }

        // match (nodes.get(entity), bones.get_mut(entity)) {
        //     (Ok(local), Ok(mut world)) => {
        //         world.0 = p_world * local.0;
        //         temp_list.push((entity, world.0.clone()));
        //     },
        //     (Ok(local), _) => {
        //         temp_list.push((entity, p_world * local.0));
        //     },
        //     (_, _) => {
                
        //     },
        // }
    }

    fn calc_bone_absolute(
        bones: &mut Query<(&BoneBaseMatrix, &mut BoneAbsolute, &mut BoneAbsoluteInv)>,
        mut temp_ids: Vec<(ObjectID, Matrix)>,
        // parents: &Query<&NodeChilds>,
        tree: &EntityTree,
    ) {
            // 广度优先遍历 - 最大遍历到深度 65535
            let max = 128;
            let mut deep = 0;
            loop {
                let mut temp_list = vec![];
                if temp_ids.len() > 0 && deep < max {
                    temp_ids.into_iter().for_each(|(p_id, p_abs)| {
                        match tree.get_down(p_id) {
                            Some(node_children_head) => {
                                // let node_children_head = node_children_head.head.0;
                                let node_children_head = node_children_head.head;
                                tree.iter(node_children_head).for_each(|entity| {
                                    calc_bone_absolute_one(
                                        bones,
                                        &mut temp_list,
                                        entity,
                                        &p_abs,
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
    
    fn calc_bone_absolute_one(
        bones: &mut Query<(&BoneBaseMatrix, &mut BoneAbsolute, &mut BoneAbsoluteInv)>,
        temp_list: &mut Vec<(ObjectID, Matrix)>,
        entity: ObjectID,
        p_abs: &Matrix,
    ) {
        match bones.get_mut(entity) {
            Ok((_base, mut abs, mut absinv)) => {
                abs.0 = p_abs * _base.0;
                // abs.update(p_abs);
                absinv.update(&abs);

                temp_list.push((entity, abs.0.clone()));
            },
            Err(_e) => {
                
            },
        }
    }

    pub fn sys_dispose_about_skeleton(
        items: Query<(Entity, &DisposeReady, &SkeletonRefs, &Skeleton), Or<(Changed<DisposeReady>, Changed<SkeletonRefs>)>>,
        mut disposecanlist: ResMut<ActionListDisposeCan>,
    ) {
        items.iter().for_each(|(entity, state, refs, skeleton)| {
            if state.0 == false || refs.len() > 0 { return };

            skeleton.bones.iter().for_each(|entity| {
                disposecanlist.push(OpsDisposeCan::ops(*entity));
            });

            disposecanlist.push(OpsDisposeCan::ops(entity));
        });
    }
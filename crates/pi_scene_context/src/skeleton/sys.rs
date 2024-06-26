
use pi_scene_shell::prelude::*;
use pi_scene_math::Matrix;

use crate::transforms::transform_node::*;

use super::{skeleton::*, bone::*};

    pub fn sys_skin_dirty_by_bone(
        mut skins: Query<&mut SkeletonBonesDirty>,
        bones: Query<&SkeletonID, Changed<GlobalMatrix>>,
    ) {
        bones.iter().for_each(|skin| {
            if let Some(idskin) = skin.0 {
                if let Ok(mut item) = skins.get_mut(idskin) {
                    *item = SkeletonBonesDirty(true);
                }
            }
        });
    }

    pub fn sys_bones_initial(
        items: Query<
            &Skeleton,
            Changed<SkeletonInitBaseMatrix>
        >,
        mut bones: Query<(&BoneBaseMatrix, &mut BoneAbsolute, &mut BoneAbsoluteInv)>,
        // parents: Query<&NodeChilds>,
        parents: EntityTree,
    ) {
        items.iter().for_each(|skeleton| {
            let root = skeleton.root;
            let temp = if let Ok((base, mut abs, mut absinv)) = bones.get_mut(root) {
                abs.0.copy_from(&base.0);
                absinv.update(&abs);
                (root, abs.0.clone())
            } else {
                (root, Matrix::identity())
            };
            let (node, abs) = temp;
            let temp_ids: Vec<(ObjectID, Matrix)> = vec![(node, abs)];
            calc_bone(&mut bones, temp_ids, &parents);
        });
    }

    pub fn sys_skin_buffer_update(
        mut items: Query<
            (
                &Skeleton,
                &mut SkeletonBonesDirty
            ),
            Changed<SkeletonBonesDirty>
        >,
        bones: Query<(&GlobalMatrix, &BoneAbsoluteInv)>,
    ) {
        items.iter_mut().for_each(|(skel, mut skindirty)| {
            if skindirty.0 {
                match skel.mode {
                    ESkinCode::None => {},
                    ESkinCode::UBO(_, _, cache) => {
                        if cache == 1 {
                            let mut data = vec![];
                            skel.bones.iter().for_each(|bone| {
                                if let Ok((matrix, absinv)) = bones.get(bone.clone()) {
                                    let matrix = matrix.matrix * absinv.0;
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

            skindirty.0 = false;
        });
    }


    fn calc_bone(
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
                        // if let Ok(childs) = parents.get(p_id) {
                        //     childs.iter().for_each(|child| {
                        //         calc_bone_one(
                        //             bones,
                        //             &mut temp_list,
                        //             *child,
                        //             &p_abs
                        //         );
                        //     });
                        // }
                        match tree.get_down(p_id) {
                            Some(node_children_head) => {
                                // let node_children_head = node_children_head.head.0;
                                let node_children_head = node_children_head.head.0;
                                tree.iter(node_children_head).for_each(|entity| {
                                    calc_bone_one(
                                        bones,
                                        &mut temp_list,
                                        entity,
                                        &p_abs
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
    
    fn calc_bone_one(
        bones: &mut Query<(&BoneBaseMatrix, &mut BoneAbsolute, &mut BoneAbsoluteInv)>,
        temp_list: &mut Vec<(ObjectID, Matrix)>,
        entity: ObjectID,
        p_abs: &Matrix,
    ) {
        match bones.get_mut(entity) {
            Ok((_base, mut abs, mut absinv)) => {
                abs.update(p_abs);
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